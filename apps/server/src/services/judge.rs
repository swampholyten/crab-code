use async_trait::async_trait;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::time::{Duration, timeout};
use uuid::Uuid;

use crate::{
    errors::{JudgeError, Result, ServiceError},
    models::{judge::*, submission::*},
    repositories::{
        judge::JudgeRepositoryTrait, language::LanguageRepositoryTrait,
        submission::SubmissionRepositoryTrait, test_case::TestCaseRepositoryTrait,
    },
};

#[async_trait]
pub trait JudgeServiceTrait: Send + Sync {
    async fn queue_submission(&self, submission_id: Uuid) -> Result<()>;
    async fn get_execution_logs(&self, submission_id: Uuid) -> Result<Vec<ExecutionLog>>;
    async fn get_supported_languages(&self) -> Result<Vec<String>>;
    async fn is_language_supported(&self, language: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct JudgeService {
    judge_repository: Arc<dyn JudgeRepositoryTrait + Send + Sync>,
    submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
    test_case_repository: Arc<dyn TestCaseRepositoryTrait + Send + Sync>,
    language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>,
}

impl JudgeService {
    pub fn new(
        judge_repository: Arc<dyn JudgeRepositoryTrait + Send + Sync>,
        submission_repository: Arc<dyn SubmissionRepositoryTrait + Send + Sync>,
        test_case_repository: Arc<dyn TestCaseRepositoryTrait + Send + Sync>,
        language_repository: Arc<dyn LanguageRepositoryTrait + Send + Sync>,
    ) -> Self {
        Self {
            judge_repository,
            submission_repository,
            test_case_repository,
            language_repository,
        }
    }

    async fn compile_code(
        &self,
        code: &str,
        language: &str,
        work_dir: &Path,
    ) -> Result<Option<String>> {
        let config = self
            .judge_repository
            .get_language_config(language)
            .await?
            .ok_or_else(|| JudgeError::UnsupportedLanguage(language.to_string()))?;

        if let Some(compile_command) = &config.compile_command {
            // Write source code to file
            let source_file = work_dir.join(format!("solution{}", config.file_extension));
            fs::write(&source_file, code).map_err(|e| JudgeError::SystemError(e.to_string()))?;

            // Execute compile command
            let output = Command::new("sh")
                .arg("-c")
                .arg(compile_command)
                .current_dir(work_dir)
                .output()
                .map_err(|e| JudgeError::SystemError(e.to_string()))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                return Ok(Some(error_msg.to_string()));
            }
        } else {
            // For interpreted languages, just write the source file
            let source_file = work_dir.join(format!("solution{}", config.file_extension));
            fs::write(&source_file, code).map_err(|e| JudgeError::SystemError(e.to_string()))?;
        }

        Ok(None)
    }

    async fn execute_with_timeout(
        &self,
        code: &str,
        language: &str,
        input: &str,
        time_limit: i32,
    ) -> Result<(String, String, i32, i32, i32)> {
        // (stdout, stderr, execution_time, memory_used, exit_code)
        let config = self
            .judge_repository
            .get_language_config(language)
            .await?
            .ok_or_else(|| JudgeError::UnsupportedLanguage(language.to_string()))?;

        // Create temporary directory
        let temp_dir = TempDir::new().map_err(|e| JudgeError::SystemError(e.to_string()))?;
        let work_dir = temp_dir.path();

        // Compile if needed
        if let Some(compile_error) = self.compile_code(code, language, work_dir).await? {
            return Ok((String::new(), compile_error, 0, 0, 1));
        }

        // Execute with timeout
        let start_time = std::time::Instant::now();

        let execution_future = async {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(&config.execute_command)
                .current_dir(work_dir)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| JudgeError::SystemError(e.to_string()))?;

            // Write input to stdin
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin
                    .write_all(input.as_bytes())
                    .map_err(|e| JudgeError::SystemError(e.to_string()))?;
            }

            // Wait for completion
            let output = child
                .wait_with_output()
                .map_err(|e| JudgeError::SystemError(e.to_string()))?;

            Ok::<_, crate::errors::Error>(output)
        };

        let timeout_duration = Duration::from_millis(time_limit as u64);
        let output = match timeout(timeout_duration, execution_future).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                // Timeout occurred
                return Ok((
                    String::new(),
                    "Time limit exceeded".to_string(),
                    time_limit,
                    0,
                    124, // Timeout exit code
                ));
            }
        };

        let execution_time = start_time.elapsed().as_millis() as i32;
        let memory_used = 1024; // Placeholder - would need proper memory monitoring
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, execution_time, memory_used, exit_code))
    }

    fn compare_output(&self, expected: &str, actual: &str) -> bool {
        // Normalize whitespace and compare
        let expected_normalized = expected.trim().replace('\r', "");
        let actual_normalized = actual.trim().replace('\r', "");
        expected_normalized == actual_normalized
    }

    async fn execute_submission_internal(&self, submission_id: Uuid) -> Result<()> {
        tracing::info!("Starting execution for submission {}", submission_id);

        // Get submission details
        let submission = self
            .submission_repository
            .find_by_id(submission_id)
            .await?
            .ok_or_else(|| ServiceError::NotFoundError("Submission not found".to_string()))?;

        // Validate language is supported
        if !self
            .language_repository
            .exists(&submission.language_id)
            .await?
        {
            let error_msg = format!("Unsupported language: {}", submission.language_id);
            tracing::warn!(
                "Unsupported language for submission {}: {}",
                submission_id,
                submission.language_id
            );

            // Log execution attempt
            let log_request = CreateExecutionLogRequest {
                submission_id,
                language: submission.language_id.clone(),
                execution_time: None,
                memory_used: None,
                exit_code: None,
                stdout: None,
                stderr: Some(error_msg.clone()),
                status: SubmissionStatus::RuntimeError,
                error_message: Some(error_msg.clone()),
            };
            self.judge_repository
                .create_execution_log(log_request)
                .await?;

            // Update submission status
            self.submission_repository
                .update_status(
                    submission_id,
                    SubmissionStatus::RuntimeError,
                    None,
                    None,
                    Some(error_msg),
                )
                .await?;

            return Ok(());
        }

        // Get test cases for the problem
        let test_cases = self
            .test_case_repository
            .find_by_problem(submission.problem_id, true)
            .await?;

        if test_cases.is_empty() {
            let error_msg = "No test cases found for this problem";
            tracing::warn!("No test cases for submission {}", submission_id);

            // Log execution attempt
            let log_request = CreateExecutionLogRequest {
                submission_id,
                language: submission.language_id.clone(),
                execution_time: None,
                memory_used: None,
                exit_code: None,
                stdout: None,
                stderr: Some(error_msg.to_string()),
                status: SubmissionStatus::RuntimeError,
                error_message: Some(error_msg.to_string()),
            };
            self.judge_repository
                .create_execution_log(log_request)
                .await?;

            // Update submission status
            self.submission_repository
                .update_status(
                    submission_id,
                    SubmissionStatus::RuntimeError,
                    None,
                    None,
                    Some(error_msg.to_string()),
                )
                .await?;

            return Ok(());
        }

        // Get language configuration
        let config = self
            .judge_repository
            .get_language_config(&submission.language_id)
            .await?
            .ok_or_else(|| JudgeError::UnsupportedLanguage(submission.language_id.clone()))?;

        let mut passed_tests = 0;
        let mut max_execution_time = 0;
        let mut max_memory_used = 0;
        let mut overall_status = SubmissionStatus::Accepted;
        let mut error_message = None;

        // Run each test case
        for (i, test_case) in test_cases.iter().enumerate() {
            tracing::debug!(
                "Running test case {} for submission {}",
                i + 1,
                submission_id
            );

            let (stdout, stderr, execution_time, memory_used, exit_code) = match self
                .execute_with_timeout(
                    &submission.code,
                    &submission.language_id,
                    &test_case.input_data,
                    config.time_limit,
                )
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    tracing::error!("Execution error for submission {}: {}", submission_id, e);
                    overall_status = SubmissionStatus::RuntimeError;
                    error_message = Some(format!("Execution error: {}", e));
                    break;
                }
            };

            max_execution_time = max_execution_time.max(execution_time);
            max_memory_used = max_memory_used.max(memory_used);

            // Check for compilation/runtime errors
            if exit_code != 0 {
                if stderr.contains("Time limit exceeded") {
                    overall_status = SubmissionStatus::TimeLimitExceeded;
                    error_message = Some("Time limit exceeded".to_string());
                } else {
                    overall_status = SubmissionStatus::RuntimeError;
                    error_message = Some(if stderr.is_empty() {
                        format!("Runtime error (exit code: {})", exit_code)
                    } else {
                        stderr.clone()
                    });
                }
                break;
            }

            // Check if output matches expected
            let passed = self.compare_output(&test_case.expected_output, &stdout);

            if passed {
                passed_tests += 1;
                tracing::debug!(
                    "Test case {} passed for submission {}",
                    i + 1,
                    submission_id
                );
            } else if overall_status == SubmissionStatus::Accepted {
                overall_status = SubmissionStatus::WrongAnswer;
                error_message = Some("Wrong answer".to_string());
                tracing::debug!(
                    "Test case {} failed for submission {}",
                    i + 1,
                    submission_id
                );
            }

            // Log this test case execution
            let log_request = CreateExecutionLogRequest {
                submission_id,
                language: submission.language_id.clone(),
                execution_time: Some(execution_time),
                memory_used: Some(memory_used),
                exit_code: Some(exit_code),
                stdout: Some(stdout),
                stderr: if stderr.is_empty() {
                    None
                } else {
                    Some(stderr)
                },
                status: if passed {
                    SubmissionStatus::Accepted
                } else {
                    SubmissionStatus::WrongAnswer
                },
                error_message: if passed {
                    None
                } else {
                    Some(format!("Test case {} failed", i + 1))
                },
            };
            self.judge_repository
                .create_execution_log(log_request)
                .await?;

            // Stop on first wrong answer or error (optional - you can run all tests)
            if !passed && overall_status == SubmissionStatus::WrongAnswer {
                break;
            }
        }

        tracing::info!(
            "Submission {} completed: status={:?}, passed={}/{}, time={}ms",
            submission_id,
            overall_status,
            passed_tests,
            test_cases.len(),
            max_execution_time
        );

        // Update submission with final results
        self.submission_repository
            .update_status(
                submission_id,
                overall_status,
                if max_execution_time > 0 {
                    Some(max_execution_time)
                } else {
                    None
                },
                if max_memory_used > 0 {
                    Some(max_memory_used)
                } else {
                    None
                },
                error_message,
            )
            .await?;

        Ok(())
    }
}

#[async_trait]
impl JudgeServiceTrait for JudgeService {
    async fn queue_submission(&self, submission_id: Uuid) -> Result<()> {
        tracing::info!("Queuing submission {} for execution", submission_id);

        // Spawn async task to execute submission
        let judge_service = self.clone();
        tokio::spawn(async move {
            if let Err(e) = judge_service
                .execute_submission_internal(submission_id)
                .await
            {
                tracing::error!("Failed to execute submission {}: {}", submission_id, e);

                // Try to update submission with error status
                if let Err(update_err) = judge_service
                    .submission_repository
                    .update_status(
                        submission_id,
                        SubmissionStatus::RuntimeError,
                        None,
                        None,
                        Some(format!("Judge system error: {}", e)),
                    )
                    .await
                {
                    tracing::error!(
                        "Failed to update submission {} with error status: {}",
                        submission_id,
                        update_err
                    );
                }
            }
        });

        Ok(())
    }

    async fn get_execution_logs(&self, submission_id: Uuid) -> Result<Vec<ExecutionLog>> {
        self.judge_repository
            .find_execution_logs_by_submission(submission_id)
            .await
    }

    async fn get_supported_languages(&self) -> Result<Vec<String>> {
        let languages = self.language_repository.list().await?;
        Ok(languages.into_iter().map(|lang| lang.name).collect())
    }

    async fn is_language_supported(&self, language: &str) -> Result<bool> {
        self.language_repository.exists(language).await
    }
}


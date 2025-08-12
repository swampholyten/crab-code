CREATE TABLE execution_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    submission_id UUID NOT NULL REFERENCES submissions(id) ON DELETE CASCADE,
    language TEXT NOT NULL,
    execution_time INTEGER, -- in milliseconds
    memory_used INTEGER, -- in KB
    exit_code INTEGER,
    stdout TEXT,
    stderr TEXT,
    status submission_status NOT NULL,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_execution_logs_submission_id ON execution_logs(submission_id);
CREATE INDEX idx_execution_logs_created_at ON execution_logs(created_at);
CREATE INDEX idx_execution_logs_status ON execution_logs(status);

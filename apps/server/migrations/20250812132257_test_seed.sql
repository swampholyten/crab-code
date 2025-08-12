INSERT INTO languages (name, display_name, file_extension) VALUES
('javascript', 'JavaScript', '.js'),
('python', 'Python', '.py'),
('java', 'Java', '.java'),
('cpp', 'C++', '.cpp'),
('c', 'C', '.c'),
('csharp', 'C#', '.cs'),
('go', 'Go', '.go'),
('rust', 'Rust', '.rs'),
('typescript', 'TypeScript', '.ts'),
('php', 'PHP', '.php');

-- Insert users (passwords are argon2 hashed versions of 'password123')
-- Hash generated with: argon2::hash_encoded(b"password123", b"randomsalt123456", &argon2::Config::default()).unwrap()
INSERT INTO users (id, username, email, password_hash, role) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'admin', 'admin@codeplatform.com', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzQ1Ng$qC+5jmQoAEiJzHj5+lY2fA+I+dOHGz8zHKDe6U8F9wg', 'admin'),
('550e8400-e29b-41d4-a716-446655440002', 'alice_coder', 'alice@example.com', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzQ1Ng$qC+5jmQoAEiJzHj5+lY2fA+I+dOHGz8zHKDe6U8F9wg', 'user'),
('550e8400-e29b-41d4-a716-446655440003', 'bob_dev', 'bob@example.com', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzQ1Ng$qC+5jmQoAEiJzHj5+lY2fA+I+dOHGz8zHKDe6U8F9wg', 'user'),
('550e8400-e29b-41d4-a716-446655440004', 'charlie_hacker', 'charlie@example.com', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzQ1Ng$qC+5jmQoAEiJzHj5+lY2fA+I+dOHGz8zHKDe6U8F9wg', 'user'),
('550e8400-e29b-41d4-a716-446655440005', 'diana_programmer', 'diana@example.com', '$argon2i$v=19$m=4096,t=3,p=1$cmFuZG9tc2FsdDEyMzQ1Ng$qC+5jmQoAEiJzHj5+lY2fA+I+dOHGz8zHKDe6U8F9wg', 'user');

-- Insert tags
INSERT INTO tags (name, description) VALUES
('array', 'Problems involving arrays and lists'),
('string', 'String manipulation problems'),
('hash-table', 'Problems using hash tables or dictionaries'),
('dynamic-programming', 'Dynamic programming problems'),
('math', 'Mathematical problems'),
('two-pointers', 'Two pointers technique'),
('sliding-window', 'Sliding window technique'),
('binary-search', 'Binary search problems'),
('tree', 'Tree data structure problems'),
('graph', 'Graph algorithms'),
('sorting', 'Sorting algorithms'),
('stack', 'Stack data structure'),
('queue', 'Queue data structure'),
('linked-list', 'Linked list problems'),
('recursion', 'Recursive solutions'),
('greedy', 'Greedy algorithms'),
('backtracking', 'Backtracking problems'),
('bit-manipulation', 'Bit manipulation techniques');

-- Insert problems (fixed with proper string escaping)
INSERT INTO problems (id, title, slug, description, difficulty) VALUES
('650e8400-e29b-41d4-a716-446655440001', 'Two Sum', 'two-sum',
'Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:
Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:
Input: nums = [3,3], target = 6
Output: [0,1]', 'easy'),

('650e8400-e29b-41d4-a716-446655440002', 'Valid Parentheses', 'valid-parentheses',
'Given a string s containing just the characters ''('')'', ''{'', ''}'', ''['' and '']'', determine if the input string is valid.

An input string is valid if:
1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.', 'easy'),

('650e8400-e29b-41d4-a716-446655440003', 'Longest Substring Without Repeating Characters', 'longest-substring-without-repeating-characters',
'Given a string s, find the length of the longest substring without repeating characters.', 'medium'),

('650e8400-e29b-41d4-a716-446655440004', 'Maximum Subarray', 'maximum-subarray',
'Given an integer array nums, find the subarray with the largest sum, and return its sum.', 'medium'),

('650e8400-e29b-41d4-a716-446655440005', 'Merge k Sorted Lists', 'merge-k-sorted-lists',
'You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.', 'hard');

-- Insert problem tags
INSERT INTO problem_tags (problem_id, tag_id) VALUES
('650e8400-e29b-41d4-a716-446655440001', 'array'),
('650e8400-e29b-41d4-a716-446655440001', 'hash-table'),
('650e8400-e29b-41d4-a716-446655440002', 'string'),
('650e8400-e29b-41d4-a716-446655440002', 'stack'),
('650e8400-e29b-41d4-a716-446655440003', 'string'),
('650e8400-e29b-41d4-a716-446655440003', 'sliding-window'),
('650e8400-e29b-41d4-a716-446655440003', 'hash-table'),
('650e8400-e29b-41d4-a716-446655440004', 'array'),
('650e8400-e29b-41d4-a716-446655440004', 'dynamic-programming'),
('650e8400-e29b-41d4-a716-446655440005', 'linked-list'),
('650e8400-e29b-41d4-a716-446655440005', 'sorting');

-- Insert test cases (fixed with proper string escaping)
INSERT INTO test_cases (id, problem_id, input_data, expected_output, is_sample) VALUES
('750e8400-e29b-41d4-a716-446655440001', '650e8400-e29b-41d4-a716-446655440001', '[2,7,11,15]\n9', '[0,1]', true),
('750e8400-e29b-41d4-a716-446655440002', '650e8400-e29b-41d4-a716-446655440001', '[3,2,4]\n6', '[1,2]', true),
('750e8400-e29b-41d4-a716-446655440003', '650e8400-e29b-41d4-a716-446655440001', '[3,3]\n6', '[0,1]', true),
('750e8400-e29b-41d4-a716-446655440004', '650e8400-e29b-41d4-a716-446655440002', '()', 'true', true),
('750e8400-e29b-41d4-a716-446655440005', '650e8400-e29b-41d4-a716-446655440002', '()[]{}', 'true', true),
('750e8400-e29b-41d4-a716-446655440006', '650e8400-e29b-41d4-a716-446655440002', '(]', 'false', true);

-- Insert submissions
INSERT INTO submissions (id, user_id, problem_id, language_id, code, status, execution_time, memory_used) VALUES
('850e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', '650e8400-e29b-41d4-a716-446655440001', 'python',
'def twoSum(nums, target):
    num_map = {}
    for i, num in enumerate(nums):
        complement = target - num
        if complement in num_map:
            return [num_map[complement], i]
        num_map[num] = i
    return []', 'accepted', 15, 1024),

('850e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003', '650e8400-e29b-41d4-a716-446655440002', 'javascript',
'function isValid(s) {
    const stack = [];
    const map = {")": "(", "}": "{", "]": "["};
    
    for (let char of s) {
        if (char === "(" || char === "{" || char === "[") {
            stack.push(char);
        } else {
            if (stack.length === 0 || stack.pop() !== map[char]) {
                return false;
            }
        }
    }
    return stack.length === 0;
}', 'accepted', 8, 512);

/// https://leetcode.com/problems/valid-parentheses/submissions/941544201/
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    let mut is_valid = true;
    for c in s.chars() {
        if !is_valid {
            break;
        }

        match c {
            '(' => stack.push(c),
            ')' => is_valid = stack.pop().unwrap_or(' ') == '(',
            '{' => stack.push(c),
            '}' => is_valid = stack.pop().unwrap_or(' ') == '{',
            '[' => stack.push(c),
            ']' => is_valid = stack.pop().unwrap_or(' ') == '[',
            _ => is_valid = false
        }
    }
    is_valid && stack.is_empty()
}
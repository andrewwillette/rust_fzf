use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

/// Prompts the user to select the item in the CLI using the fzf tool.
/// It's required that fzf is installed in the environment.
pub fn select(fzf_input: Vec<String>, args: Vec<String>) -> String {
    let mut child = Command::new("fzf")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let mut fzf_in = String::new();
    for selection in fzf_input {
        fzf_in.push_str(&selection);
        fzf_in.push('\n');
    }
    stdin
        .write_all(fzf_in.as_bytes())
        .expect("Failed to write fzf_input to fzf command stdin");
    let output = child
        .wait_with_output()
        .expect("Failed to read fzf command stdout");
    String::from(str::from_utf8(&output.stdout).unwrap().trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let test_input = vec!["test".to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!("test", output);
    }

    #[test]
    fn test_select_with_args() {
        let test_input = vec!["test".to_string(), "me".to_string()];
        let output = select(test_input, vec![String::from("--layout=reverse")]);
        assert_eq!("test", output);
    }
}

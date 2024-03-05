use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

/// Prompts the user to select the item in the CLI using the fzf tool.
/// It's required that fzf is installed in the environment.
pub fn select(fzf_selections: Vec<String>, fzf_cli_args: Vec<String>) -> String {
    let mut fzf_in = String::new();
    for selection in fzf_selections {
        fzf_in.push_str(&selection);
        // newline splits up the selections for fzf
        fzf_in.push('\n');
    }

    let mut child = Command::new("fzf")
        .args(fzf_cli_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    // write the selections to fzf command's stdin
    child
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(fzf_in.as_bytes())
        .expect("Failed to write fzf_input to fzf command stdin");

    let mut selection = String::from(
        str::from_utf8(
            &child
                .wait_with_output()
                .expect("Failed to read fzf command stdout")
                .stdout,
        )
        .unwrap(),
    );

    // trime the newline we added previously from the selection
    trim_newline(&mut selection);

    return selection;
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    // run with
    // `cargo test -- --test-threads=1`
    use super::*;

    #[test]
    fn test_select() {
        let expected = "test";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_select_with_args() {
        let expected = "test";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, vec![String::from("--layout=reverse")]);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_select_with_trailing_spaces() {
        let expected = "test ";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_select_with_middle_spaces() {
        let expected = "test test ";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(expected, output);
    }
}

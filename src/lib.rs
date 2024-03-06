use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

/// Prompts the user to select from the fzf_selections using the fzf CLI tool.
/// It's required that fzf is installed in the environment.
pub fn select(
    fzf_selections: Vec<String>,
    fzf_cli_args: Vec<String>,
) -> Result<Vec<String>, String> {
    // we input the fzf_selections to fzf command's
    // stdin as a single string with newline separated values
    let mut fzf_in = String::new();
    for selection in fzf_selections {
        fzf_in.push_str(&selection);
        fzf_in.push('\n');
    }

    let mut fzf_cmd: std::process::Child = match Command::new("fzf")
        .args(fzf_cli_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return Err("failed to spawn fzf CLI tool".to_string()),
    };

    // write the selections to fzf command's stdin
    fzf_cmd
        .stdin
        .take()
        .expect("Failed to open stdin")
        .write_all(fzf_in.as_bytes())
        .expect("Failed to write fzf_input to fzf command stdin");

    let selection = String::from(
        str::from_utf8(
            &fzf_cmd
                .wait_with_output()
                .expect("Failed to read fzf command stdout")
                .stdout,
        )
        .unwrap(),
    );

    // split the selection string by newline to account for multi select
    let selections: Vec<String> = selection.split('\n').map(|s| s.to_string()).collect();

    // final value is always empty string from trailing newline in last selection
    Ok(selections[..selections.len() - 1].to_vec())
}

// run with
// `cargo test -- --test-threads=1`
#[cfg(test)]
mod select_tests {
    use super::*;

    #[test]
    fn basic() {
        let expected = "pickme";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(true, output.is_ok());
        assert_eq!(vec![expected], output.unwrap());
    }

    #[test]
    fn with_args() {
        let expected = "pickme";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, vec![String::from("--layout=reverse")]);
        assert_eq!(vec![expected], output.unwrap());
    }

    #[test]
    fn trailing_spaces() {
        let expected = "pickme ";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(vec![expected], output.unwrap());
    }

    #[test]
    fn middle_spaces() {
        let expected = "pick me ";
        let test_input = vec![expected.to_string(), "me".to_string()];
        let output = select(test_input, Vec::new());
        assert_eq!(vec![expected], output.unwrap());
    }

    #[test]
    fn multi_select() {
        let expected = "pick me multi ";
        let expected2 = "pick me too!";
        let test_input = vec![expected.to_string(), expected2.to_string()];
        let mut output = select(test_input, vec!["--multi".to_string()]).unwrap();
        output.sort();
        let mut expected_sorted = vec![expected, expected2];
        expected_sorted.sort();
        assert_eq!(expected_sorted, output);
    }
}

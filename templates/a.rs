use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn case1() {
        let input = r#"
1
2 3
test
        "#;
        let expected_output = r#"
6 test
        "#
        .trim();
        let actual_output = run_with_input(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn case2() {
        let input = r#"
72
128 256
myonmyon
        "#;
        let expected_output = r#"
456 myonmyon
        "#
        .trim();
        let actual_output = run_with_input(input);
        assert_eq!(actual_output, expected_output);
    }

    fn run_with_input(input: &str) -> String {
        use std::env;
        use std::io::Write;
        use std::path::Path;
        use std::process::{Command, Stdio};

        let current_file = env::args().nth(0).expect("Failed to get current file name");
        let file_stem = Path::new(&current_file)
            .file_stem()
            .expect("Failed to get file stem")
            .to_str()
            .expect("Failed to convert to str");

        // Remove the ID appended to the file name in debug builds
        let file_stem = file_stem
            .split('-')
            .next()
            .expect("Failed to split file stem");

        let mut child = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(file_stem)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = child.wait_with_output().expect("Failed to read stdout");
        String::from_utf8(output.stdout)
            .expect("Failed to convert output to string")
            .trim()
            .to_string()
    }
}

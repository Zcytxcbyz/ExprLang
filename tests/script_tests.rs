use std::fs;
use std::path::Path;

#[test]
fn test_script_files() {
    let script_dir = Path::new("tests/scripts");
    if !script_dir.exists() {
        return;
    }

    let entries = fs::read_dir(script_dir).expect("Failed to read scripts directory");
    for entry in entries {
        let entry = entry.expect("Invalid directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("expr") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read script file: {:?}", path));

        let (script_body, expected) = if let Some(last_line) = content.lines().last() {
            if let Some(stripped) = last_line.trim().strip_prefix("#=>") {
                let expected_str = stripped.trim().to_string();
                let body = content
                    .lines()
                    .rev()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n");
                (body, Some(expected_str))
            } else {
                (content.clone(), None)
            }
        } else {
            (content.clone(), None)
        };

        match ExprLang::evaluate(&script_body) {
            Ok(val) => {
                let result_str = val.to_string();
                if let Some(expected) = expected {
                    assert_eq!(
                        result_str,
                        expected,
                        "Script {:?} produced unexpected output.\nExpected: {}\nGot: {}",
                        path.file_name().unwrap(),
                        expected,
                        result_str
                    );
                } else {
                    eprintln!(
                        "Script {:?} executed successfully (no expected value)",
                        path.file_name().unwrap()
                    );
                }
            }
            Err(e) => {
                panic!(
                    "Script {:?} failed to evaluate: {}",
                    path.file_name().unwrap(),
                    e
                );
            }
        }
    }
}

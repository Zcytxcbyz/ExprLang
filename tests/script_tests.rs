use std::fs;
use std::path::Path;

/// 读取脚本文件，执行并验证结果是否与 `#=>` 标记一致
#[test]
fn test_script_files() {
    let script_dir = Path::new("tests/scripts");
    if !script_dir.exists() {
        // 如果目录不存在，跳过测试（或创建）
        return;
    }

    let entries = fs::read_dir(script_dir).expect("Failed to read scripts directory");
    for entry in entries {
        let entry = entry.expect("Invalid directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("expr") {
            continue; // 只处理 .expr 文件
        }

        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read script file: {:?}", path));

        // 提取期望值行（最后一行以 #=> 开头）
        let (script_body, expected) = if let Some(last_line) = content.lines().last() {
            if let Some(stripped) = last_line.trim().strip_prefix("#=>") {
                let expected_str = stripped.trim().to_string();
                let body = content.lines().rev().skip(1).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                (body, Some(expected_str))
            } else {
                (content.clone(), None)
            }
        } else {
            (content.clone(), None)
        };

        // 执行脚本
        match ExprLang::evaluate(&script_body) {
            Ok(val) => {
                let result_str = val.to_string();
                if let Some(expected) = expected {
                    assert_eq!(
                        result_str, expected,
                        "Script {:?} produced unexpected output.\nExpected: {}\nGot: {}",
                        path.file_name().unwrap(),
                        expected,
                        result_str
                    );
                } else {
                    // 没有期望值，只确保执行成功
                    eprintln!("Script {:?} executed successfully (no expected value)", path.file_name().unwrap());
                }
            }
            Err(e) => {
                // 如果脚本本应失败，可以通过期望值标记，但这里未实现，先 panic
                panic!("Script {:?} failed to evaluate: {}", path.file_name().unwrap(), e);
            }
        }
    }
}

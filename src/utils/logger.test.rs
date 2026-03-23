// src/utils/logger.test.rs

use super::logger;
use std::io::Cursor;

/**
 * test_log returns an error when the syslog file does not exist.
 */
#[test]
fn test_log_returns_error_on_missing_file() {
    let result = logger::test_log();

    /* On most CI/test environments, /var/log/syslog may not exist. */
    /* If it does exist, the function should succeed. */
    /* If it doesn't, we expect an Err. */
    /* Either outcome is valid — we just verify no panic. */
    match result {
        Ok(()) => { /* syslog exists and was readable — that's fine. */ }
        Err(e) => {
            let msg = format!("{}", e);
            assert!(
                msg.contains("No such file")
                    || msg.contains("Permission denied")
                    || msg.contains("cannot find the path")
                    || msg.contains("cannot find the file"),
                "Expected file-not-found or permission error, got: {}",
                msg
            );
        }
    }
}

/**
 * read_log processes lines from an in-memory buffer.
 */
#[test]
fn read_log_from_memory() {
    let data = "line one\nline two\nline three\n";
    let cursor = Cursor::new(data.as_bytes());
    let reader = std::io::BufReader::new(cursor);

    let result = logger::read_log(reader);
    assert!(result.is_ok());
}

/**
 * read_log handles empty input.
 */
#[test]
fn read_log_empty_input() {
    let data = "";
    let cursor = Cursor::new(data.as_bytes());
    let reader = std::io::BufReader::new(cursor);

    let result = logger::read_log(reader);
    assert!(result.is_ok());
}

/**
 * read_log handles single line without newline.
 */
#[test]
fn read_log_single_line() {
    let data = "single line no newline";
    let cursor = Cursor::new(data.as_bytes());
    let reader = std::io::BufReader::new(cursor);

    let result = logger::read_log(reader);
    assert!(result.is_ok());
}

/**
 * read_log handles unicode content.
 */
#[test]
fn read_log_unicode() {
    let data = "日本語ログ\n🚀 rocket launch\n";
    let cursor = Cursor::new(data.as_bytes());
    let reader = std::io::BufReader::new(cursor);

    let result = logger::read_log(reader);
    assert!(result.is_ok());
}

/**
 * process_line does not panic on normal input.
 */
#[test]
fn process_line_does_not_panic() {
    logger::process_line("Hello, log line!".to_string());
}

/**
 * process_line does not panic on empty input.
 */
#[test]
fn process_line_empty_string() {
    logger::process_line(String::new());
}

/**
 * process_line does not panic on unicode input.
 */
#[test]
fn process_line_unicode() {
    logger::process_line("日本語テスト 🚀".to_string());
}

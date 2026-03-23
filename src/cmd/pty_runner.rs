// src/cmd/pty_runner.rs

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{BufRead, BufReader};

/**
 * Run Interactive Commands
 *
 * Spawns a PTY shell and sends a sequence of commands into it.
 * Each line of output is passed to `on_line` for live streaming.
 * All output is collected and returned as a single String.
 *
 * The `commands` slice contains the shell commands to execute.
 * They are joined with " && " and run as a single shell invocation,
 * which eliminates the need for arbitrary sleep() timers.
 *
 * For long-running processes that produce streaming output (e.g.,
 * build scripts, downloads), the on_line callback fires per line
 * so progress can be relayed to the user in real time.
 */
pub fn run_interactive(
    commands: &[String],
    mut on_line: impl FnMut(&str),
) -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        return Err("run_interactive is not supported on Windows.".into());
    }

    /* Open a PTY pair. */
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    /* Build the shell command. */
    let joined = commands.join(" && ");
    let mut cmd = CommandBuilder::new("bash");
    cmd.arg("-c");
    cmd.arg(&joined);

    /* Spawn the child in the PTY. */
    let mut child = pair.slave.spawn_command(cmd)?;

    /* Drop the slave side so we get EOF when the child exits. */
    drop(pair.slave);

    /* Read output line by line from the master side. */
    let reader = pair.master.try_clone_reader()?;
    let buf_reader = BufReader::new(reader);
    let mut collected = String::new();

    for line in buf_reader.lines() {
        match line {
            Ok(text) => {
                on_line(&text);
                collected.push_str(&text);
                collected.push('\n');
            }
            Err(_) => break,
        }
    }

    /* Wait for the child to exit. */
    let status = child.wait()?;
    if !status.success() {
        /* Still return the collected output, but wrap in Err with context. */
        return Err(format!(
            "Command exited with status {:?}.\nOutput:\n{}",
            status, collected
        )
        .into());
    }

    Ok(collected)
}

/**
 * Run Interactive With Exit Callback
 *
 * Same as run_interactive, but calls `on_exit` after the child exits
 * successfully. This preserves the InteractiveProcess::new_with_exit_callback
 * behavior from the original code.
 */
pub fn run_interactive_with_exit_callback(
    commands: &[String],
    on_line: impl FnMut(&str),
    on_exit: impl FnOnce(),
) -> Result<String, Box<dyn std::error::Error>> {
    let result = run_interactive(commands, on_line);

    if result.is_ok() {
        on_exit();
    }

    result
}

#[cfg(test)]
mod pty_runner_test {
    use super::*;

    /**
     * run_interactive returns Err on Windows.
     */
    #[test]
    fn run_interactive_windows_guard() {
        if cfg!(target_os = "windows") {
            let result = run_interactive(&["echo hello".to_string()], |_| {});
            assert!(result.is_err());
        }
    }

    /**
     * run_interactive can execute a simple echo command.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_echo() {
        let result = run_interactive(&["echo hello_pty_test".to_string()], |line| {
            println!("    ↳ {}", line);
        });

        assert!(
            result.is_ok(),
            "run_interactive('echo hello_pty_test') should succeed, got: {:?}",
            result.unwrap_err()
        );

        let output = result.unwrap();
        assert!(
            output.contains("hello_pty_test"),
            "Output should contain 'hello_pty_test', got: {}",
            output
        );
    }

    /**
     * run_interactive collects multi-line output.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_multiline() {
        let result = run_interactive(
            &["echo line_one".to_string(), "echo line_two".to_string()],
            |_| {},
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("line_one"));
        assert!(output.contains("line_two"));
    }

    /**
     * run_interactive returns Err on failing command.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_failing_command() {
        let result = run_interactive(&["false".to_string()], |_| {});

        assert!(
            result.is_err(),
            "run_interactive('false') should return Err"
        );
    }

    /**
     * run_interactive on_line callback fires for each line.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_callback_fires() {
        let mut line_count = 0;

        let _ = run_interactive(&["echo aaa".to_string(), "echo bbb".to_string()], |_line| {
            line_count += 1;
        });

        assert!(
            line_count >= 2,
            "on_line callback should fire at least 2 times, got: {}",
            line_count
        );
    }

    /**
     * run_interactive_with_exit_callback calls on_exit on success.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_with_exit_callback_fires() {
        let mut exit_called = false;

        let result = run_interactive_with_exit_callback(
            &["echo done".to_string()],
            |_| {},
            || {
                exit_called = true;
            },
        );

        assert!(result.is_ok());
        assert!(exit_called, "on_exit callback should have been called");
    }

    /**
     * run_interactive_with_exit_callback does NOT call on_exit on failure.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_with_exit_callback_no_fire_on_failure() {
        let mut exit_called = false;

        let _ = run_interactive_with_exit_callback(
            &["false".to_string()],
            |_| {},
            || {
                exit_called = true;
            },
        );

        assert!(
            !exit_called,
            "on_exit callback should NOT be called on failure"
        );
    }

    /**
     * run_interactive handles empty command list.
     */
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn run_interactive_empty_commands() {
        let result = run_interactive(&[], |_| {});

        /* Empty join produces "bash -c ''" which exits 0. */
        assert!(result.is_ok());
    }
}

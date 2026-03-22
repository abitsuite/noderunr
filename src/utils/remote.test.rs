// src/utils/remote.test.rs

use super::remote;

/**
 * start_download completes without panicking.
 *
 * NOTE: This is a smoke test. The function runs a progress bar
 * simulation with thread::sleep. It exercises all lines in remote.rs.
 */
#[test]
fn start_download_does_not_panic() {
    remote::start_download();
}

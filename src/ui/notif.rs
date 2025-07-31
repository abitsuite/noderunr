use serde_json::json;

use crate::utils;

/**
 * Print Test
 */
pub fn test_print() {
    println!("{:#}\n", json!({
        "type": "message",
        "content": "Hi there!",
        "createdAt": utils::epoch::ms(),
    }));
}

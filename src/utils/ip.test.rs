// src/utils/ip.test.rs

use crate::utils::ip;

/**
 * ip::get() returns a result (Ok or Err) without panicking.
 *
 * NOTE: This requires network access. On CI without network,
 * an Err is acceptable.
 */
#[test]
fn get_ip_does_not_panic() {
    let result = ip::get();

    match result {
        Ok(map) => {
            /* httpbin.org/ip returns {"origin": "x.x.x.x"} */
            assert!(
                map.contains_key("origin"),
                "Response should contain 'origin' key, got: {:?}",
                map
            );

            let origin = &map["origin"];
            assert!(
                !origin.is_empty(),
                "Origin IP should not be empty"
            );
        }
        Err(e) => {
            /* Network may not be available — just verify it's a real error. */
            let msg = format!("{}", e);
            assert!(
                !msg.is_empty(),
                "Error message should not be empty"
            );
        }
    }
}

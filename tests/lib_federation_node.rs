// tests/lib_federation_node.rs

use noderunr::{FederationNode, Validator};

/**
 * Federation Node Creation
 *
 * Verify that a FederationNode can be created with the expected fields.
 */
#[test]
fn federation_node_creation() {
    let node = FederationNode {
        id: String::from("190171ee-ac37-4e05-988b-a7e683c1e5d3"),
        owner: String::from("Shomari"),
        title: String::from("Awesome Node # 1337"),
        created_at: String::from("2025-08-09"),
    };

    assert_eq!(node.id, "190171ee-ac37-4e05-988b-a7e683c1e5d3");
    assert_eq!(node.owner, "Shomari");
    assert_eq!(node.title, "Awesome Node # 1337");
    assert_eq!(node.created_at, "2025-08-09");
}

/**
 * Federation Node Get ID (Validator Trait)
 *
 * Verify that the Validator trait's get_id() returns the expected format.
 */
#[test]
fn federation_node_get_id() {
    let node = FederationNode {
        id: String::from("abc-123"),
        owner: String::from("TestOwner"),
        title: String::from("TestNode"),
        created_at: String::from("2025-01-01"),
    };

    let result = node.get_id();

    assert_eq!(result, "id is abc-123");
}

/**
 * Federation Node Empty Fields
 *
 * Verify that a FederationNode handles empty strings without panicking.
 */
#[test]
fn federation_node_empty_fields() {
    let node = FederationNode {
        id: String::new(),
        owner: String::new(),
        title: String::new(),
        created_at: String::new(),
    };

    assert_eq!(node.get_id(), "id is ");
    assert!(node.id.is_empty());
    assert!(node.owner.is_empty());
}

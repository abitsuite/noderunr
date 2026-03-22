// src/lib.test.rs

use super::*;

/**
 * get_version() starts with 'v' and contains "(alpha)".
 */
#[test]
fn version_format() {
    let v = get_version();

    assert!(v.starts_with('v'), "Version '{}' must start with 'v'", v);
    assert!(
        v.contains("(alpha)"),
        "Version '{}' must contain '(alpha)'",
        v
    );
}

/**
 * get_version() is non-empty.
 */
#[test]
fn version_not_empty() {
    let v = get_version();

    assert!(!v.is_empty(), "Version string should not be empty");
}

/**
 * get_version() contains at least one numeric character.
 */
#[test]
fn version_contains_numeric() {
    let v = get_version();

    assert!(
        v.chars().any(|c| c.is_ascii_digit()),
        "Version '{}' does not contain any numeric characters",
        v
    );
}

/**
 * get_version() has a semver-like structure (major.minor or major.minor.patch).
 */
#[test]
fn version_has_semver_structure() {
    let v = get_version();

    /* Strip the leading 'v' and trailing ' (alpha)'. */
    let semver_part = v
        .trim_start_matches('v')
        .trim_end_matches(" (alpha)");

    let parts: Vec<&str> = semver_part.split('.').collect();

    assert!(
        parts.len() >= 2,
        "Version '{}' should have at least major.minor, got {} parts",
        v,
        parts.len()
    );

    for part in &parts {
        assert!(
            part.chars().all(|c| c.is_ascii_digit()),
            "Version segment '{}' should be numeric in '{}'",
            part,
            v
        );
    }
}

/**
 * FederationNode fields are stored correctly.
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
 * Validator trait get_id() returns the expected format.
 */
#[test]
fn federation_node_get_id() {
    let node = FederationNode {
        id: String::from("abc-123"),
        owner: String::from("TestOwner"),
        title: String::from("TestNode"),
        created_at: String::from("2025-01-01"),
    };

    let result = Validator::get_id(&node);

    assert_eq!(result, "id is abc-123");
}

/**
 * FederationNode handles empty strings without panicking.
 */
#[test]
fn federation_node_empty_fields() {
    let node = FederationNode {
        id: String::new(),
        owner: String::new(),
        title: String::new(),
        created_at: String::new(),
    };

    assert_eq!(Validator::get_id(&node), "id is ");
    assert!(node.id.is_empty());
    assert!(node.owner.is_empty());
}

/**
 * FederationNode with unicode in fields does not panic.
 */
#[test]
fn federation_node_unicode_fields() {
    let node = FederationNode {
        id: String::from("nödë-ïd-🚀"),
        owner: String::from("Shömari"),
        title: String::from("NodΞ Tëst"),
        created_at: String::from("二〇二五"),
    };

    assert_eq!(node.id, "nödë-ïd-🚀");
    assert_eq!(Validator::get_id(&node), "id is nödë-ïd-🚀");
}

/**
 * Subnet trait get_id() returns the expected format.
 */
#[test]
fn federation_node_subnet_get_id() {
    let node = FederationNode {
        id: String::from("subnet-test-42"),
        owner: String::from("TestOwner"),
        title: String::from("TestNode"),
        created_at: String::from("2025-01-01"),
    };

    let result = Subnet::get_id(&node);

    assert_eq!(result, "subnet-subnet-test-42");
}

/**
 * Validator trait is object-safe (can be used as trait object).
 */
#[test]
fn validator_trait_object_safe() {
    let node = FederationNode {
        id: String::from("trait-obj-1"),
        owner: String::from("Owner"),
        title: String::from("Node"),
        created_at: String::from("2025-01-01"),
    };

    let validator: &dyn Validator = &node;
    assert_eq!(validator.get_id(), "id is trait-obj-1");
}

/**
 * Subnet trait is object-safe (can be used as trait object).
 */
#[test]
fn subnet_trait_object_safe() {
    let node = FederationNode {
        id: String::from("trait-obj-2"),
        owner: String::from("Owner"),
        title: String::from("Node"),
        created_at: String::from("2025-01-01"),
    };

    let subnet: &dyn Subnet = &node;
    assert_eq!(subnet.get_id(), "subnet-trait-obj-2");
}

use pretty_assertions::assert_eq;
use syndex::CONFIG;

/// TODO: Example integration test, feel free to replace it with something meaningful.
#[test]
fn integrate() {
    let x = 42;
    assert_eq!(x, 42);
}

/// Make sure all necessary values are available in the environment.
#[test]
fn test_config() {
    let _ = CONFIG.example_bool;
}

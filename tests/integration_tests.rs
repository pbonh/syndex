use pretty_assertions::assert_eq;
use syndex::synthesis_state::builder::{DesignState, Flow};
use syndex::CONFIG;

/// TODO: Example integration test, feel free to replace it with something meaningful.
#[test]
fn integrate() {
    let x = 42;
    assert_eq!(x, 42);
}

/// Make sure all necessary values are available in the environment.
#[test]
#[ignore]
fn test_config() {
    let _ = CONFIG.example_bool;
}

#[test]
fn simple_flow_load() {
    let input = indoc::indoc! {"
            entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                %null = const time 0s 1e
                %and1 = and i1 %in1, %in2
                %and2 = and i1 %in3, %in4
                %or1 = or i1 %and1, %and2
                drv i1$ %out1, %or1, %null
            }
        "};

    let module = llhd::assembly::parse_module(input).unwrap();
    let _ = Flow::load(module.into());
}

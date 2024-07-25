use std::str::FromStr;

use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;
use peginator::{ParseError, PegParser};

use super::graph::LCircuit;
use super::spice::SPICENetlist;

#[derive(Debug, Clone, Default, Resource, Component)]
pub struct LNetlist {
    graph: LCircuit,
    spice: Option<SPICENetlist>,
}

impl FromStr for LNetlist {
    type Err = ParseError;

    fn from_str(spice_netlist_str: &str) -> Result<Self, Self::Err> {
        match SPICENetlist::parse(spice_netlist_str) {
            Ok(ast) => {
                let graph = LCircuit::from(&ast);
                Ok(Self {
                    graph,
                    spice: Some(ast),
                })
            }
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn default_netlist() {
        let _netlist = LNetlist::default();
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn spice_sky130_dk_a211o_2_netlist() {
        let mut spice_netlist_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        spice_netlist_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.spice",
        );
        let spice_netlist_str: String = fs::read_to_string(spice_netlist_path).unwrap();
        let _netlist = LNetlist::from_str(&spice_netlist_str).unwrap();
    }
}

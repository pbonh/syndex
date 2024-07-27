use bevy_ecs::component::Component;
use bevy_ecs::prelude::Resource;

use super::graph::LCircuit;
use super::spice::SPICENetlist;
use crate::circuit::equations::DeviceEquationMap;

#[derive(Debug, Clone, Default, Resource, Component)]
pub struct LNetlist {
    graph: LCircuit,
    spice: Option<SPICENetlist>,
}

impl From<(SPICENetlist, &DeviceEquationMap)> for LNetlist {
    fn from(spice_netlist_and_map: (SPICENetlist, &DeviceEquationMap)) -> Self {
        let spice_netlist = spice_netlist_and_map.0;
        let device_equation_map = spice_netlist_and_map.1;
        let graph = LCircuit::from((&spice_netlist, device_equation_map));
        Self {
            graph,
            spice: Some(spice_netlist),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use peginator::PegParser;

    use super::*;
    use crate::circuit::equations::DeviceEquation;

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

        let eq = indoc::indoc! {"
            e = 2.718281828459045;
            Is = 1e-12;
            eta = 1.5;
            Vt = T/11586;
            I = Is*(e^(vds/(eta*Vt)) - 1)
        "};
        let dev_eq = DeviceEquation::from_str(eq).unwrap();
        let device_eq_map = DeviceEquationMap::from([("m".to_owned(), dev_eq)]);

        let spice_netlist = SPICENetlist::parse(&spice_netlist_str).unwrap();
        let _netlist = LNetlist::from((spice_netlist, &device_eq_map));
    }
}

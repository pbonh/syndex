pub mod macros;

use std::collections::BTreeSet;
use std::hash::Hash;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use euclid::default::Box2D;
use llhd::ir::prelude::*;
use llhd::ir::InstData;

use crate::circuit::graph::{LCircuit, LCircuitEdgeID};
use crate::llhd::components::unit::UnitBundle;

/// Type Constraint for Use in a Datalog Relation Column
pub trait FlatIndex: Clone + PartialEq + Eq + Hash {}

/// Design Unit's Circuit Component
#[derive(Debug, Clone, Component)]
pub struct CircuitComponent {
    circuit: LCircuit,
}

/// Design Unit Component
#[derive(Debug, Clone, Bundle)]
pub struct SynthesisUnitBundle {
    unit: UnitBundle,
    circuit: CircuitComponent,
}

/// `FlatIndex` for Design Units
pub type DesignUnitIndex = (UnitId, BTreeSet<LCircuitEdgeID>, Vec<Box2D<usize>>);

/// `FlatIndex` for Design Gates
pub type DesignGateIndex = (
    UnitId,
    Inst,
    InstData,
    BTreeSet<LCircuitEdgeID>,
    Vec<Box2D<usize>>,
);

/// `FlatIndex` for Design Nets
pub type DesignNetIndex = (
    UnitId,
    Inst,
    Value,
    BTreeSet<LCircuitEdgeID>,
    Vec<Box2D<usize>>,
);

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ascent::*;
    use bevy_ecs::world::World;
    use euclid::Point2D;
    use itertools::Itertools;
    use llhd::ir::InstData;
    use llhd::table::TableKey;

    use super::*;
    use crate::llhd::components::inst::*;
    use crate::llhd::components::unit::*;

    #[test]
    fn ascent_column_compatability_design_unit_index() {
        ascent! {
           relation node(DesignUnitIndex, Rc<Vec<DesignUnitIndex>>);
           relation edge(DesignUnitIndex, DesignUnitIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let unit1_nets = 1;
        let unit_loc = Point2D::zero();
        let unit1 = (
            UnitId::new(1),
            BTreeSet::from([unit1_nets]),
            vec![Box2D::new(unit_loc, unit_loc)],
        );
        let unit2_nets = 2;
        let unit2 = (
            UnitId::new(2),
            BTreeSet::from([unit2_nets]),
            vec![Box2D::new(unit_loc, unit_loc)],
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(unit1, unit2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_gate_index() {
        ascent! {
           relation node(DesignGateIndex, Rc<Vec<DesignGateIndex>>);
           relation edge(DesignGateIndex, DesignGateIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = 1;
        let node_loc = Point2D::zero();
        let node1_data = InstData::default();
        let node1 = (
            UnitId::new(1),
            Inst::new(1),
            node1_data,
            BTreeSet::from([node1_nets]),
            vec![Box2D::new(node_loc, node_loc)],
        );
        let node2_nets = 2;
        let node2_data = InstData::default();
        let node2 = (
            UnitId::new(1),
            Inst::new(2),
            node2_data,
            BTreeSet::from([node2_nets]),
            vec![Box2D::new(node_loc, node_loc)],
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_net_index() {
        ascent! {
           relation node(DesignNetIndex, Rc<Vec<DesignNetIndex>>);
           relation edge(DesignNetIndex, DesignNetIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = 1;
        let node_loc = Point2D::zero();
        let node1_net = Value::new(0);
        let node1 = (
            UnitId::new(1),
            Inst::new(1),
            node1_net,
            BTreeSet::from([node1_nets]),
            vec![Box2D::new(node_loc, node_loc)],
        );
        let node2_nets = 2;
        let node2_net = Value::new(0);
        let node2 = (
            UnitId::new(1),
            Inst::new(2),
            node2_net,
            BTreeSet::from([node2_nets]),
            vec![Box2D::new(node_loc, node_loc)],
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }

    #[test]
    fn llhd_bevy_ecs_bundle_manual_creation() {
        let input = indoc::indoc! {"
            entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                %null = const time 0s 1e
                %and1 = and i1 %in1, %in2
                %and2 = and i1 %in3, %in4
                %or1 = or i1 %and1, %and2
                drv i1$ %out1, %or1, %null
            }
        "};

        let mut ecs = World::default();

        let circuit = LCircuit::default();

        let module = llhd::assembly::parse_module(input).unwrap();
        let test_unit = module.units().next().unwrap();
        let test_unit_id = test_unit.id();
        let test_unit_name = test_unit.name();
        let test_unit_kind = test_unit.kind();
        let test_unit_sig = test_unit.sig();
        let args = test_unit.args().collect_vec();
        assert_eq!(5, args.len(), "There should be 5 args in unit.");
        let insts = test_unit.all_insts().collect_vec();
        assert_eq!(6, insts.len(), "There should be 6 Insts in unit.");
        let inst_const_time_id = insts[0];
        let inst_const_time_data = test_unit[inst_const_time_id].clone();
        assert_eq!(
            Opcode::ConstTime,
            inst_const_time_data.opcode(),
            "First Inst should be `const time`."
        );
        let inst_and1_id = insts[1];
        let inst_and1_data = test_unit[inst_and1_id].clone();
        assert_eq!(
            Opcode::And,
            inst_and1_data.opcode(),
            "Second Inst should be `and`."
        );
        let inst_and2_id = insts[2];
        let inst_and2_data = test_unit[inst_and2_id].clone();
        assert_eq!(
            Opcode::And,
            inst_and2_data.opcode(),
            "Third Inst should be `and`."
        );
        let inst_or1_id = insts[3];
        let inst_or1_data = test_unit[inst_or1_id].clone();
        assert_eq!(
            Opcode::Or,
            inst_or1_data.opcode(),
            "Fourth Inst should be `or`."
        );
        let inst_drv1_id = insts[4];
        let inst_drv1_data = test_unit[inst_drv1_id].clone();
        assert_eq!(
            Opcode::Drv,
            inst_drv1_data.opcode(),
            "Fifth Inst should be `drv`."
        );
        let inst_null_id = insts[5];
        let inst_null_data = test_unit[inst_null_id].clone();
        assert!(
            matches!(inst_null_data, InstData::Nullary { .. }),
            "Sixth Inst should be Null instruction(doesn't actually exist)."
        );
        // let _inst_const_time_arg1 = inst_const_time_data.args()[0];
        // let _inst_const_time_arg2 = inst_const_time_data.args()[1];
        let inst_and1_arg1 = inst_and1_data.args()[0];
        let inst_and1_arg2 = inst_and1_data.args()[1];
        let inst_and2_arg1 = inst_and2_data.args()[0];
        let inst_and2_arg2 = inst_and2_data.args()[1];
        let inst_or1_arg1 = inst_or1_data.args()[0];
        let inst_or1_arg2 = inst_or1_data.args()[1];
        let inst_drv1_arg1 = inst_drv1_data.args()[0];
        let inst_drv1_arg2 = inst_drv1_data.args()[1];
        let inst_drv1_arg3 = inst_drv1_data.args()[2];

        let unit_id_component = UnitIdComponent { id: test_unit_id };
        let test_unit_bundle = SynthesisUnitBundle {
            unit: UnitBundle {
                unit: unit_id_component.clone(),
                name: UnitNameComponent {
                    name: test_unit_name.clone(),
                    kind: test_unit_kind,
                    signature: test_unit_sig.clone(),
                },
            },
            circuit: CircuitComponent { circuit },
        };
        let _unit_entity_id = ecs.spawn(test_unit_bundle);

        let test_unit_arg1_bundle = UnitArgBundle {
            unit: unit_id_component.clone(),
            arg: ValueComponent { value: args[0] },
        };
        let test_unit_arg2_bundle = UnitArgBundle {
            unit: unit_id_component.clone(),
            arg: ValueComponent { value: args[1] },
        };
        let test_unit_arg3_bundle = UnitArgBundle {
            unit: unit_id_component.clone(),
            arg: ValueComponent { value: args[2] },
        };
        let test_unit_arg4_bundle = UnitArgBundle {
            unit: unit_id_component.clone(),
            arg: ValueComponent { value: args[3] },
        };
        let test_unit_out1_bundle = UnitArgBundle {
            unit: unit_id_component.clone(),
            arg: ValueComponent { value: args[4] },
        };
        let arg_entities = ecs
            .spawn_batch([
                test_unit_arg1_bundle,
                test_unit_arg2_bundle,
                test_unit_arg3_bundle,
                test_unit_arg4_bundle,
                test_unit_out1_bundle,
            ])
            .collect_vec();
        assert_eq!(5, arg_entities.len());

        let test_unit_inst1_bundle = InstBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent {
                id: inst_const_time_id,
            },
            data: InstDataComponent {
                data: inst_const_time_data,
            },
        };
        let test_unit_inst2_bundle = InstBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            data: InstDataComponent {
                data: inst_and1_data,
            },
        };
        let test_unit_inst3_bundle = InstBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            data: InstDataComponent {
                data: inst_and2_data,
            },
        };
        let test_unit_inst4_bundle = InstBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            data: InstDataComponent {
                data: inst_or1_data,
            },
        };
        let test_unit_inst5_bundle = InstBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_drv1_id },
            data: InstDataComponent {
                data: inst_drv1_data,
            },
        };
        let inst_entities = ecs
            .spawn_batch([
                test_unit_inst1_bundle,
                test_unit_inst2_bundle,
                test_unit_inst3_bundle,
                test_unit_inst4_bundle,
                test_unit_inst5_bundle,
            ])
            .collect_vec();
        assert_eq!(5, inst_entities.len());

        let test_unit_inst_value1_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            value: ValueComponent {
                value: inst_and1_arg1,
            },
        };
        let test_unit_inst_value2_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            value: ValueComponent {
                value: inst_and1_arg2,
            },
        };
        let test_unit_inst_value3_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            value: ValueComponent {
                value: inst_and2_arg1,
            },
        };
        let test_unit_inst_value4_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            value: ValueComponent {
                value: inst_and2_arg2,
            },
        };
        let test_unit_inst_value5_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            value: ValueComponent {
                value: inst_or1_arg1,
            },
        };
        let test_unit_inst_value6_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            value: ValueComponent {
                value: inst_or1_arg2,
            },
        };
        let test_unit_inst_value7_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg1,
            },
        };
        let test_unit_inst_value8_bundle = ValueBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg2,
            },
        };
        let test_unit_inst_value9_bundle = ValueBundle {
            unit: unit_id_component,
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg3,
            },
        };
        let value_entities = ecs
            .spawn_batch([
                test_unit_inst_value1_bundle,
                test_unit_inst_value2_bundle,
                test_unit_inst_value3_bundle,
                test_unit_inst_value4_bundle,
                test_unit_inst_value5_bundle,
                test_unit_inst_value6_bundle,
                test_unit_inst_value7_bundle,
                test_unit_inst_value8_bundle,
                test_unit_inst_value9_bundle,
            ])
            .collect_vec();
        assert_eq!(9, value_entities.len());
    }
}

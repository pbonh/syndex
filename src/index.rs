pub mod category;
pub mod macros;

use core::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::Hash;

use ascent::Lattice;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use derive_getters::{Dissolve, Getters};
use euclid::default::Box2D;
use llhd::ir::prelude::*;
use llhd::ir::InstData;
use typed_builder::TypedBuilder;

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct DesignUnitIndex {
    unit: UnitId,
    nets: BTreeSet<LCircuitEdgeID>,
    bb: Vec<Box2D<usize>>,
}

/// `FlatIndex` for Design Ports
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct DesignValueDefIndex {
    unit: UnitId,
    value: Value,
    nets: BTreeSet<LCircuitEdgeID>,
    bb: Vec<Box2D<usize>>,
}

/// `FlatIndex` for Design Gates
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters, Dissolve)]
pub struct DesignGateIndex {
    unit: UnitId,
    id: Inst,
    value: Value,
    data: InstData,
    nets: BTreeSet<LCircuitEdgeID>,
    bb: Vec<Box2D<usize>>,
}

impl PartialOrd for DesignGateIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.nets.cmp(&other.nets))
    }
}

impl Lattice for DesignGateIndex {
    fn meet(self, _other: Self) -> Self {
        todo!()
    }

    fn join(self, _other: Self) -> Self {
        todo!()
    }
}

/// `FlatIndex` for Design Nets
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypedBuilder, Getters)]
pub struct DesignValueRefIndex {
    unit: UnitId,
    id: Inst,
    value: Value,
    nets: BTreeSet<LCircuitEdgeID>,
    bb: Vec<Box2D<usize>>,
}

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
        let unit1 = DesignUnitIndex::builder()
            .unit(UnitId::new(1))
            .nets(BTreeSet::from([unit1_nets]))
            .bb(vec![Box2D::new(unit_loc, unit_loc)])
            .build();
        let unit2_nets = 2;
        let unit2 = DesignUnitIndex::builder()
            .unit(UnitId::new(2))
            .nets(BTreeSet::from([unit2_nets]))
            .bb(vec![Box2D::new(unit_loc, unit_loc)])
            .build();
        let mut prog = AscentProgram::default();
        prog.edge = vec![(unit1, unit2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_port_index() {
        ascent! {
           relation node(DesignValueDefIndex, Rc<Vec<DesignValueDefIndex>>);
           relation edge(DesignValueDefIndex, DesignValueDefIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let unit1_nets = 1;
        let unit_loc = Point2D::zero();
        let unit1 = DesignValueDefIndex::builder()
            .unit(UnitId::new(1))
            .value(Value::new(0))
            .nets(BTreeSet::from([unit1_nets]))
            .bb(vec![Box2D::new(unit_loc, unit_loc)])
            .build();
        let unit2_nets = 2;
        let unit2 = DesignValueDefIndex::builder()
            .unit(UnitId::new(2))
            .value(Value::new(0))
            .nets(BTreeSet::from([unit2_nets]))
            .bb(vec![Box2D::new(unit_loc, unit_loc)])
            .build();
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
        let node1_value = Value::new(1);
        let node1 = DesignGateIndex::builder()
            .unit(UnitId::new(1))
            .id(Inst::new(1))
            .value(node1_value)
            .data(node1_data)
            .nets(BTreeSet::from([node1_nets]))
            .bb(vec![Box2D::new(node_loc, node_loc)])
            .build();
        let node2_nets = 2;
        let node2_data = InstData::default();
        let node2_value = Value::new(2);
        let node2 = DesignGateIndex::builder()
            .unit(UnitId::new(1))
            .id(Inst::new(2))
            .value(node2_value)
            .data(node2_data)
            .nets(BTreeSet::from([node2_nets]))
            .bb(vec![Box2D::new(node_loc, node_loc)])
            .build();
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_net_index() {
        ascent! {
           relation node(DesignValueRefIndex, Rc<Vec<DesignValueRefIndex>>);
           relation edge(DesignValueRefIndex, DesignValueRefIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = 1;
        let node_loc = Point2D::zero();
        let node1_value = Value::new(0);
        let node1 = DesignValueRefIndex::builder()
            .unit(UnitId::new(1))
            .id(Inst::new(1))
            .value(node1_value)
            .nets(BTreeSet::from([node1_nets]))
            .bb(vec![Box2D::new(node_loc, node_loc)])
            .build();
        let node2_nets = 2;
        let node2_value = Value::new(0);
        let node2 = DesignValueRefIndex::builder()
            .unit(UnitId::new(1))
            .id(Inst::new(2))
            .value(node2_value)
            .nets(BTreeSet::from([node2_nets]))
            .bb(vec![Box2D::new(node_loc, node_loc)])
            .build();
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

        let inst_and1_result1 = test_unit.inst_result(inst_and1_id);
        let inst_and2_result1 = test_unit.inst_result(inst_and2_id);
        let inst_or1_result1 = test_unit.inst_result(inst_or1_id);

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

        let test_unit_arg1_bundle = ValueDefBundle {
            unit: unit_id_component.clone(),
            value: ValueComponent { value: args[0] },
        };
        let test_unit_arg2_bundle = ValueDefBundle {
            unit: unit_id_component.clone(),
            value: ValueComponent { value: args[1] },
        };
        let test_unit_arg3_bundle = ValueDefBundle {
            unit: unit_id_component.clone(),
            value: ValueComponent { value: args[2] },
        };
        let test_unit_arg4_bundle = ValueDefBundle {
            unit: unit_id_component.clone(),
            value: ValueComponent { value: args[3] },
        };
        let test_unit_out1_bundle = ValueDefBundle {
            unit: unit_id_component.clone(),
            value: ValueComponent { value: args[4] },
        };
        let value_def_entities = ecs
            .spawn_batch([
                test_unit_arg1_bundle,
                test_unit_arg2_bundle,
                test_unit_arg3_bundle,
                test_unit_arg4_bundle,
                test_unit_out1_bundle,
            ])
            .collect_vec();
        assert_eq!(5, value_def_entities.len());

        let test_unit_inst1_bundle = GateBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent {
                id: inst_const_time_id,
            },
            data: InstDataComponent {
                data: inst_const_time_data,
            },
        };
        let test_unit_inst2_bundle = GateBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            data: InstDataComponent {
                data: inst_and1_data,
            },
        };
        let test_unit_inst3_bundle = GateBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            data: InstDataComponent {
                data: inst_and2_data,
            },
        };
        let test_unit_inst4_bundle = GateBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            data: InstDataComponent {
                data: inst_or1_data,
            },
        };
        let test_unit_inst5_bundle = GateBundle {
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

        let test_unit_inst_value1_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            value: ValueComponent {
                value: inst_and1_arg1,
            },
        };
        let test_unit_inst_value2_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            value: ValueComponent {
                value: inst_and1_arg2,
            },
        };
        let test_unit_inst_value3_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and1_id },
            value: ValueComponent {
                value: inst_and1_result1,
            },
        };
        let test_unit_inst_value4_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            value: ValueComponent {
                value: inst_and2_arg1,
            },
        };
        let test_unit_inst_value5_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            value: ValueComponent {
                value: inst_and2_arg2,
            },
        };
        let test_unit_inst_value6_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_and2_id },
            value: ValueComponent {
                value: inst_and2_result1,
            },
        };
        let test_unit_inst_value7_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            value: ValueComponent {
                value: inst_or1_arg1,
            },
        };
        let test_unit_inst_value8_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            value: ValueComponent {
                value: inst_or1_arg2,
            },
        };
        let test_unit_inst_value9_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_or1_id },
            value: ValueComponent {
                value: inst_or1_result1,
            },
        };
        let test_unit_inst_value10_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg1,
            },
        };
        let test_unit_inst_value11_bundle = ValueRefBundle {
            unit: unit_id_component.clone(),
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg2,
            },
        };
        let test_unit_inst_value12_bundle = ValueRefBundle {
            unit: unit_id_component,
            id: InstIdComponent { id: inst_drv1_id },
            value: ValueComponent {
                value: inst_drv1_arg3,
            },
        };
        let value_ref_entities = ecs
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
                test_unit_inst_value10_bundle,
                test_unit_inst_value11_bundle,
                test_unit_inst_value12_bundle,
            ])
            .collect_vec();
        assert_eq!(12, value_ref_entities.len());
    }
}

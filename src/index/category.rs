use derive_getters::Getters;
use typed_builder::TypedBuilder;

use crate::index::unit::{
    DesignGateIndex, DesignUnitIndex, DesignValueDefIndex, DesignValueRefIndex,
};

pub type DesignUnitSet = Vec<DesignUnitIndex>;
pub type DesignGateSet = Vec<DesignGateIndex>;
pub type DesignValueDefSet = Vec<DesignValueDefIndex>;
pub type DesignValueRefSet = Vec<DesignValueRefIndex>;

#[derive(Debug, Clone, Default, TypedBuilder, Getters)]
pub struct DICategoryObject {
    units: DesignUnitSet,
    gates: DesignGateSet,
    value_defs: DesignValueDefSet,
    value_refs: DesignValueRefSet,
}

// pub type DICagetoryMorphism = Fn(&DICategoryObject) -> DICategoryObject;
pub trait DICagetoryMorphism {
    fn arrow(self, domain_object: &DICategoryObject) -> DICategoryObject;
}

#[cfg(test)]
mod tests {

    use ascent::ascent_run;
    use itertools::Itertools;
    use llhd::ir::prelude::*;

    use super::*;

    struct ExampleCategory;

    fn transform_design_unit_via_lattice(design_index_sets: &DICategoryObject) -> DICategoryObject {
        let units = design_index_sets.units().clone();
        let value_defs = design_index_sets.value_defs().clone();
        let value_refs = design_index_sets.value_refs().clone();
        let facts: Vec<(Opcode, Value, Value, Value, DesignGateIndex)> = design_index_sets
            .gates()
            .iter()
            .map(|gate_object| {
                let inst_data = &gate_object.data();
                let opcode = inst_data.opcode();
                let inst_val = gate_object.value().to_owned();
                let arg1 = inst_data.args()[0];
                let arg2 = inst_data.args()[1];
                (opcode, inst_val, arg1, arg2, gate_object.clone())
            })
            .collect_vec();
        let design_unit_demorgans = ascent_run! {
           lattice gates(Opcode, Value, Value, Value, DesignGateIndex) = facts;
           lattice demorgan(Opcode, Value, Value, Value, DesignGateIndex);

           demorgan(Opcode::And, out_idx, a, and1_idx, obj_or),
           demorgan(Opcode::Or, and1_idx, b, c, obj_and1)
           <-- gates(Opcode::Or, out_idx, and1_idx, and2_idx, obj_or),
               gates(Opcode::And, and1_idx, a, b, obj_and1),
               gates(Opcode::And, and2_idx, a, c, obj_and2);
        }
        .demorgan;
        DICategoryObject::builder()
            .units(units)
            .gates(
                design_unit_demorgans
                    .into_iter()
                    .map(|gate| {
                        let _new_opcode = gate.0;
                        let new_value = gate.1;
                        let _new_arg1 = gate.2;
                        let _new_arg2 = gate.3;
                        let gate_object = gate.4;
                        let (unit, id, _value, data, nets, bb) = gate_object.dissolve();
                        DesignGateIndex::builder()
                            .unit(unit)
                            .id(id)
                            .value(new_value)
                            .data(data)
                            .nets(nets)
                            .bb(bb)
                            .build()
                    })
                    .collect_vec(),
            )
            .value_defs(value_defs)
            .value_refs(value_refs)
            .build()
    }

    fn transform_design_unit_via_values_and_opcode(
        design_index_sets: &DICategoryObject,
    ) -> DICategoryObject {
        let units = design_index_sets.units().clone();
        let value_defs = design_index_sets.value_defs().clone();
        let value_refs = design_index_sets.value_refs().clone();
        let facts: Vec<(Opcode, Value, Value, Value, DesignGateIndex)> = design_index_sets
            .gates()
            .iter()
            .map(|gate_object| {
                let inst_data = &gate_object.data();
                let opcode = inst_data.opcode();
                let inst_val = gate_object.value().to_owned();
                let arg1 = inst_data.args()[0];
                let arg2 = inst_data.args()[1];
                (opcode, inst_val, arg1, arg2, gate_object.clone())
            })
            .collect_vec();
        let design_unit_demorgans = ascent_run! {
           relation gates(Opcode, Value, Value, Value, DesignGateIndex) = facts;
           relation demorgan(Opcode, Value, Value, Value, DesignGateIndex);

           demorgan(Opcode::And, out_idx, a, and1_idx, obj_or),
           demorgan(Opcode::Or, and1_idx, b, c, obj_and1)
           <-- gates(Opcode::Or, out_idx, and1_idx, and2_idx, obj_or),
               gates(Opcode::And, and1_idx, a, b, obj_and1),
               gates(Opcode::And, and2_idx, a, c, obj_and2);
        }
        .demorgan;
        DICategoryObject::builder()
            .units(units)
            .gates(
                design_unit_demorgans
                    .into_iter()
                    .map(|gate| {
                        let _new_opcode = gate.0;
                        let new_value = gate.1;
                        let _new_arg1 = gate.2;
                        let _new_arg2 = gate.3;
                        let gate_object = gate.4;
                        let (unit, id, _value, data, nets, bb) = gate_object.dissolve();
                        DesignGateIndex::builder()
                            .unit(unit)
                            .id(id)
                            .value(new_value)
                            .data(data)
                            .nets(nets)
                            .bb(bb)
                            .build()
                    })
                    .collect_vec(),
            )
            .value_defs(value_defs)
            .value_refs(value_refs)
            .build()
    }

    #[test]
    fn default_value() {
        let _ = DICategoryObject::default();
    }
}

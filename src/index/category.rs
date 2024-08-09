use derive_getters::Getters;
use typed_builder::TypedBuilder;

use crate::index::{DesignGateIndex, DesignUnitIndex, DesignValueDefIndex, DesignValueRefIndex};

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
    use crate::circuit::graph::LCircuitEdgeID;
    use ascent::ascent_run;
    use euclid::default::Box2D;
    use itertools::Itertools;
    use llhd::ir::prelude::*;
    use llhd::ir::InstData;
    use llhd::table::TableKey;
    use std::collections::BTreeSet;

    use super::*;

    struct ExampleCategory;

    fn transform_design_unit(design_index_sets: &DICategoryObject) -> DICategoryObject {
        let units = design_index_sets.units().clone();
        let value_defs = design_index_sets.value_defs().clone();
        let value_refs = design_index_sets.value_refs().clone();
        let unit_id = design_index_sets.gates()[0].0;
        let facts: Vec<(Opcode, Value, Value, Value)> = design_index_sets
            .gates()
            .iter()
            .map(|domain_object| {
                let inst_data = &domain_object.3;
                let opcode = inst_data.opcode();
                let inst_val = domain_object.2;
                let arg1 = inst_data.args()[0];
                let arg2 = inst_data.args()[1];
                (opcode, inst_val, arg1, arg2)
            })
            .collect_vec();
        let design_unit_demorgans = ascent_run! {
           relation gates(Opcode, Value, Value, Value) = facts;
           relation demorgan(Opcode, Value, Value, Value);

           demorgan(Opcode::And, out_idx, a, and1_idx),
           demorgan(Opcode::Or, and1_idx, b, c)
           <-- gates(Opcode::Or, out_idx, and1_idx, and2_idx),
               gates(Opcode::And, and1_idx, a, b),
               gates(Opcode::And, and2_idx, a, c);
        }
        .demorgan;
        DICategoryObject::builder()
            .units(units)
            .gates(
                design_unit_demorgans
                    .into_iter()
                    .map(|_gate| {
                        (
                            unit_id,
                            Inst::new(0),
                            Value::new(0),
                            InstData::default(),
                            BTreeSet::<LCircuitEdgeID>::default(),
                            Vec::<Box2D<usize>>::default(),
                        )
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

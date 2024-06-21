use ascent::ascent_run;
use llhd::ir::prelude::*;

use crate::llhd_world::components::inst::LLHDInstComponent;
use crate::llhd_world::world::LLHDWorld;

fn run_divisor_extraction(llhd_world: &mut LLHDWorld, unit_id: UnitId) {
    let _unit_program_and_inst: Vec<(Value, Value, Value, u64)> = llhd_world
        .unit_program_inst::<LLHDInstComponent>(unit_id)
        .filter(|((_unit_id, _inst_id), inst_component)| {
            Opcode::And == inst_component.data.opcode()
        })
        .map(|(_inst_idx, inst_component)| {
            let inst_value = inst_component.value.unwrap();
            let args = inst_component.data.args();
            let input1 = args[0];
            let input2 = args[1];
            (inst_value, input1, input2, 2)
        })
        .collect();
    ascent_run! {
        relation andi(Value, Value, Value, u64);
        relation ori(Value, Value, Value, u64);

        andi(or_idx, and1_idx, and2_idx, and1_area),
        ori(and1_idx, and1_in1, and1_in2, or_area),
        ori(and2_idx, and2_in1, and2_in2, or_area)
        <-- ori(or_idx, and1_idx, and2_idx, or_area),
            andi(and1_idx, and1_in1, and1_in2, and1_area),
            andi(and2_idx, and2_in1, and2_in2, and2_area),
        if and1_area + and2_area + or_area > 2*or_area + and1_area;

        ori(and_idx, or1_idx, or2_idx, or1_area),
        andi(or1_idx, or1_in1, or1_in2, and_area),
        andi(or2_idx, or2_in1, or2_in2, and_area)
        <-- andi(and_idx, or1_idx, or2_idx, and_area),
            ori(or1_idx, or1_in1, or1_in2, or1_area),
            ori(or2_idx, or2_in1, or2_in2, or2_area),
        if or1_area + or2_area + and_area > 2*and_area + or1_area;
    };
}

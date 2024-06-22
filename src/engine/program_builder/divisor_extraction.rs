use ascent::ascent_run;
use bevy_ecs::prelude::Component;
use itertools::Itertools;
use llhd::ir::prelude::*;

use crate::llhd_world::world::LLHDWorld;

#[derive(Clone, PartialEq, Eq, Hash, Component)]
struct LLHDInstLocation(u64, u64);

type LLHDProgramWithInstLocation = Vec<(Value, Value, Value, LLHDInstLocation)>;

fn get_llhd_insts(
    llhd_world: &LLHDWorld,
    unit_id: UnitId,
    opcode: Opcode,
) -> LLHDProgramWithInstLocation {
    llhd_world
        .unit_program_inst::<LLHDInstLocation>(unit_id)
        .filter(|((_unit_id, _inst_id), inst_component, _inst_data)| {
            opcode == inst_component.data.opcode()
        })
        .map(|(_inst_idx, inst_component, inst_data)| {
            let inst_value = inst_component.value.unwrap();
            let args = inst_component.data.args();
            let input1 = args[0];
            let input2 = args[1];
            (inst_value, input1, input2, inst_data)
        })
        .collect()
}

fn run_divisor_extraction(llhd_world: &LLHDWorld, unit_id: UnitId) -> LLHDProgramWithInstLocation {
    let unit_program_and_inst = get_llhd_insts(llhd_world, unit_id, Opcode::And);
    let unit_program_or_inst = get_llhd_insts(llhd_world, unit_id, Opcode::Or);
    let unit_program_not_inst = get_llhd_insts(llhd_world, unit_id, Opcode::Not);

    // Replace a*b + a*c
    // with    a*(b + c)
    let ascent_program = ascent_run! {
        relation andi(Value, Value, Value, LLHDInstLocation) = unit_program_and_inst;
        relation ori(Value, Value, Value, LLHDInstLocation) = unit_program_or_inst;
        relation noti(Value, Value, Value, LLHDInstLocation) = unit_program_not_inst;

        andi(or_idx, and1_idx, and2_idx, and1_area),
        ori(and1_idx, and1_in1, and1_in2, or_area),
        ori(and2_idx, and2_in1, and2_in2, or_area)
        <-- ori(or_idx, and1_idx, and2_idx, or_area),
            andi(and1_idx, and1_in1, and1_in2, and1_area),
            andi(and2_idx, and2_in1, and2_in2, and2_area);

        ori(and_idx, or1_idx, or2_idx, or1_area),
        andi(or1_idx, or1_in1, or1_in2, and_area),
        andi(or2_idx, or2_in1, or2_in2, and_area)
        <-- andi(and_idx, or1_idx, or2_idx, and_area),
            ori(or1_idx, or1_in1, or1_in2, or1_area),
            ori(or2_idx, or2_in1, or2_in2, or2_area);
    };

    ascent_program
        .noti
        .into_iter()
        .chain(ascent_program.andi)
        .chain(ascent_program.ori)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llhd::module::LLHDModule;

    #[test]
    #[should_panic]
    fn test_divisor_extraction() {
        // Replace a*b + a*c
        // with    a*(b + c)
        let input = indoc::indoc! {"
                entity @test_extraction (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                    %instant = const time 0s 1e
                    %and1 = and i1 %in1, %in2
                    %and2 = and i1 %in3, %in4
                    %or1 = or i1 %and1, %and2
                    drv i1$ %out1, %or1, %instant
                }
            "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = llhd_world.module().units().next().unwrap().id();
        let unit_insts = llhd_world.module().unit(unit_id).all_insts().collect_vec();
        assert_eq!(
            6,
            unit_insts.len(),
            "There should be 6 Instructions in original Unit."
        );

        let extracted_divisor_program = run_divisor_extraction(&llhd_world, unit_id);
        assert_eq!(
            2,
            extracted_divisor_program.len(),
            "There should be 2 instructions remaining in the extracted program(`a*b + a*c` -> `a*(b + c)`)."
        );
    }
}

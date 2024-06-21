use ascent::ascent;
use llhd::ir::prelude::*;

ascent! {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llhd::module::*;
    use crate::llhd_world::components::inst::LLHDInstComponent;
    use crate::llhd_world::world::*;
    use llhd::table::TableKey;

    #[test]
    fn rewrite_llhd_inst_via_ascent_engine_2and_1or() {
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
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_program_and_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::And == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 5)
            })
            .collect();
        let unit_program_or_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::Or == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 2)
            })
            .collect();

        assert_eq!(
            2,
            unit_program_and_inst.len(),
            "There should be 2 And instructions before program optimization."
        );
        assert_eq!(
            1,
            unit_program_or_inst.len(),
            "There should be 1 Or instruction before program optimization."
        );
        let mut area_before = 0;
        for inst in &unit_program_and_inst {
            area_before += inst.3;
        }
        for inst in &unit_program_or_inst {
            area_before += inst.3;
        }
        assert_eq!(
            12, area_before,
            "Area-Before should be 12, with 2 And's and 1 Or."
        );
        let mut prog = AscentProgram {
            andi: unit_program_and_inst,
            ori: unit_program_or_inst,
            ..Default::default()
        };
        assert_eq!(
            2,
            prog.andi.len(),
            "There should be 2 And instructions before running the engine."
        );
        assert_eq!(
            1,
            prog.ori.len(),
            "There should be 1 Or instructions before running the engine."
        );
        prog.run();
        assert_eq!(
            3,
            prog.andi.len(),
            "There should be 3 And instructions after running the engine."
        );
        assert_eq!(
            3,
            prog.ori.len(),
            "There should be 3 Or instructions after running the engine."
        );
    }

    #[test]
    fn rewrite_llhd_inst_via_ascent_engine_2or_1and() {
        let input = indoc::indoc! {"
                entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                    %null = const time 0s 1e
                    %or1 = or i1 %in1, %in2
                    %or2 = or i1 %in3, %in4
                    %and1 = and i1 %or1, %or2
                    drv i1$ %out1, %and1, %null
                }
            "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_program_and_inst: Vec<(Value, Value, Value, u64)> = llhd_world
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
        let unit_program_or_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::Or == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 5)
            })
            .collect();

        assert_eq!(
            1,
            unit_program_and_inst.len(),
            "There should be 1 And instructions before program optimization."
        );
        assert_eq!(
            2,
            unit_program_or_inst.len(),
            "There should be 2 Or instruction before program optimization."
        );
        let mut area_before = 0;
        for inst in &unit_program_and_inst {
            area_before += inst.3;
        }
        for inst in &unit_program_or_inst {
            area_before += inst.3;
        }
        assert_eq!(
            12, area_before,
            "Area-Before should be 9, with 2 Or's and 1 And."
        );
        let mut prog = AscentProgram {
            andi: unit_program_and_inst,
            ori: unit_program_or_inst,
            ..Default::default()
        };
        assert_eq!(
            1,
            prog.andi.len(),
            "There should be 1 And instructions before running the engine."
        );
        assert_eq!(
            2,
            prog.ori.len(),
            "There should be 2 And instructions before running the engine."
        );
        prog.run();
        assert_eq!(
            3,
            prog.andi.len(),
            "There should be 3 And instructions after running the engine."
        );
        assert_eq!(
            3,
            prog.ori.len(),
            "There should be 3 And instructions after running the engine."
        );
    }

    #[test]
    fn rewrite_llhd_inst_via_ascent_engine_no_op_2or_1and() {
        let input = indoc::indoc! {"
                entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                    %null = const time 0s 1e
                    %or1 = or i1 %in1, %in2
                    %or2 = or i1 %in3, %in4
                    %and1 = and i1 %or1, %or2
                    drv i1$ %out1, %and1, %null
                }
            "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_program_and_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::And == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 5)
            })
            .collect();
        let unit_program_or_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::Or == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 2)
            })
            .collect();

        assert_eq!(
            1,
            unit_program_and_inst.len(),
            "There should be 1 And instructions before program optimization."
        );
        assert_eq!(
            2,
            unit_program_or_inst.len(),
            "There should be 2 Or instruction before program optimization."
        );
        let mut area_before = 0;
        for inst in &unit_program_and_inst {
            area_before += inst.3;
        }
        for inst in &unit_program_or_inst {
            area_before += inst.3;
        }
        assert_eq!(
            9, area_before,
            "Area-Before should be 9, with 2 Or's and 1 And."
        );
        let mut prog = AscentProgram {
            andi: unit_program_and_inst,
            ori: unit_program_or_inst,
            ..Default::default()
        };
        assert_eq!(
            1,
            prog.andi.len(),
            "There should be 1 And instructions before running the engine."
        );
        assert_eq!(
            2,
            prog.ori.len(),
            "There should be 2 And instructions before running the engine."
        );
        prog.run();
        assert_eq!(
            1,
            prog.andi.len(),
            "There should be 1 And instructions after running the engine."
        );
        assert_eq!(
            2,
            prog.ori.len(),
            "There should be 2 And instructions after running the engine."
        );
    }

    #[test]
    fn rewrite_llhd_inst_via_ascent_engine_no_op_2and_1or() {
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
        let llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = UnitId::new(0);
        let unit_program_and_inst: Vec<(Value, Value, Value, u64)> = llhd_world
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
        let unit_program_or_inst: Vec<(Value, Value, Value, u64)> = llhd_world
            .unit_program_inst::<LLHDInstComponent>(unit_id)
            .filter(|((_unit_id, _inst_id), inst_component)| {
                Opcode::Or == inst_component.data.opcode()
            })
            .map(|(_inst_idx, inst_component)| {
                let inst_value = inst_component.value.unwrap();
                let args = inst_component.data.args();
                let input1 = args[0];
                let input2 = args[1];
                (inst_value, input1, input2, 5)
            })
            .collect();

        assert_eq!(
            2,
            unit_program_and_inst.len(),
            "There should be 2 And instructions before program optimization."
        );
        assert_eq!(
            1,
            unit_program_or_inst.len(),
            "There should be 1 Or instruction before program optimization."
        );
        let mut area_before = 0;
        for inst in &unit_program_and_inst {
            area_before += inst.3;
        }
        for inst in &unit_program_or_inst {
            area_before += inst.3;
        }
        assert_eq!(
            9, area_before,
            "Area-Before should be 12, with 2 And's and 1 Or."
        );
        let mut prog = AscentProgram {
            andi: unit_program_and_inst,
            ori: unit_program_or_inst,
            ..Default::default()
        };
        assert_eq!(
            2,
            prog.andi.len(),
            "There should be 2 And instructions before running the engine."
        );
        assert_eq!(
            1,
            prog.ori.len(),
            "There should be 1 Or instructions before running the engine."
        );
        prog.run();
        assert_eq!(
            2,
            prog.andi.len(),
            "There should be 2 And instructions after running the engine."
        );
        assert_eq!(
            1,
            prog.ori.len(),
            "There should be 1 Or instructions after running the engine."
        );
    }
}

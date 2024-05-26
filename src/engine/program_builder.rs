mod bevy_ecs_to_ascent_program_example {
    use crate::llhd_world::components::inst::LLHDInstComponent;
    use crate::llhd_world::world::*;
    use ascent::*;

    type LLHDInstRelation = (InstIndex, LLHDInstComponent);

    ascent! {
        relation llhd_unit_inst(Vec<LLHDInstRelation>);

        llhd_unit_inst(inst_info) <-- llhd_unit_inst(inst_info);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::llhd::module::*;
        use llhd::ir::prelude::*;
        use llhd::table::TableKey;

        #[test]
        fn rewrite_llhd_inst_via_ascent_engine() {
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
            let unit_program_inst: Vec<LLHDInstRelation> = llhd_world
                .unit_program_inst::<LLHDInstComponent>(unit_id)
                .collect();

            let mut prog = AscentProgram {
                llhd_unit_inst: vec![(unit_program_inst,)],
                ..Default::default()
            };
            prog.run();
        }
    }
}

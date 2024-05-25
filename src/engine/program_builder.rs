mod bevy_ecs_to_ascent_program_example {
    #[cfg(test)]
    mod tests {
        // use llhd::ir::prelude::*;

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

            let _module = llhd::assembly::parse_module(input).unwrap();
        }
    }
}

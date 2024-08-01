use llhd::ir::prelude::*;

use super::LLHDUnitArg;

pub(crate) fn iterate_unit_ids(module: &Module) -> impl Iterator<Item = UnitId> + '_ {
    module.units().map(|unit| unit.id())
}

pub(crate) fn iterate_unit_arg_defs<'unit>(
    unit: &'unit Unit,
) -> impl Iterator<Item = LLHDUnitArg> + 'unit {
    unit.args().map(|arg| (unit.id(), arg))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn build_unit_component() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let units = iterate_unit_ids(&module).collect_vec();
        assert_eq!(2, units.len(), "There should be 2 Units present in Module.");
        let first_unit = module.units().collect_vec()[0];
        let second_unit = module.units().collect_vec()[1];
        let first_unit_args = iterate_unit_arg_defs(&first_unit).collect_vec();
        assert_eq!(
            4,
            first_unit_args.len(),
            "There should be 4 args present in first unit."
        );
        let second_unit_args = iterate_unit_arg_defs(&second_unit).collect_vec();
        assert_eq!(
            0,
            second_unit_args.len(),
            "There should be 3 args present in second unit."
        );
    }
}

pub mod datatype;
mod egglog_names;
mod inst;
pub(crate) mod unit;
pub use unit::LLHDEgglogFacts;
pub mod llhd;
pub mod rules;
pub mod schedules;

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::egraph::EgglogCommandList;
    use crate::llhd_egraph::datatype::LLHDEgglogSorts;

    #[test]
    fn llhd_unit_dft_sort_valid_egglog_program() {
        static LLHD_UNIT_SORT_EGGLOG_RESOURCES_STR: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/egglog/llhd_dfg_sort.egg"
        ));
        if let Err(err_msg) = utilities::parse_egglog_program(LLHD_UNIT_SORT_EGGLOG_RESOURCES_STR) {
            panic!("Failure to parse LLHD Unit DFT Sort. ERROR: {:?}", err_msg);
        }
    }

    #[test]
    fn llhd_egglog_dfg_datatypes() {
        static LLHD_UNIT_SORT_EGGLOG_RESOURCES_STR: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/egglog/llhd_dfg_sort.egg"
        ));
        let llhd_dfg_sort: EgglogCommandList = LLHDEgglogSorts::llhd_dfg().into();
        let expected_str = utilities::trim_expr_whitespace(LLHD_UNIT_SORT_EGGLOG_RESOURCES_STR);
        assert_eq!(
            expected_str,
            llhd_dfg_sort.into_iter().join(""),
            "LLHD DFG Egglog sorts don't match expected string."
        );
    }
}

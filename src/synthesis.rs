use egglog::*;

fn build_egraph(program: &str) -> (EGraph, Vec<String>) {
    let mut egraph = EGraph::default();
    // egraph.run_mode = RunMode::ShowDesugaredEgglog;
    let msgs = egraph
        .parse_and_run_program(program)
        .expect("Failure to run program on egraph.");
    (egraph, msgs)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn build_egraph_with_string_math_example() {
        let mut egglog_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_file_path.push("resources/egglog/math_example.egg");
        let egglog_program: String = fs::read_to_string(egglog_file_path).unwrap();
        let egraph_info = build_egraph(&egglog_program);
        let egraph = egraph_info.0;
        assert_eq!(
            1578,
            egraph.num_tuples(),
            "There should be 1578 facts remaining in the egraph."
        );
    }

    #[test]
    fn build_egraph_with_string_bdd_example() {
        let mut egglog_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_file_path.push("resources/egglog/bdd_example.egg");
        let egglog_program: String = fs::read_to_string(egglog_file_path).unwrap();
        let egraph_info = build_egraph(&egglog_program);
        let egraph = egraph_info.0;
        let egraph_msgs = egraph_info.1;
        assert_eq!(
            4,
            egraph.num_tuples(),
            "There should be 4 facts remaining in the egraph."
        );
        println!("{:?}", egraph_msgs);
    }

    #[test]
    fn build_egraph_with_string_llhd_example() {
        let mut egglog_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_file_path.push("resources/egglog/llhd_dfg_example1.egg");
        let egglog_program: String = fs::read_to_string(egglog_file_path).unwrap();
        let egraph_info = build_egraph(&egglog_program);
        let egraph = egraph_info.0;
        let run_report_matches = egraph
            .get_run_report()
            .clone()
            .unwrap()
            .num_matches_per_rule
            .len();
        assert_eq!(
            2, run_report_matches,
            "There should be 2 rule matches in program."
        );
        assert_eq!(
            31,
            egraph.num_tuples(),
            "There should be 31 facts remaining in the egraph."
        );
    }

    #[test]
    #[should_panic]
    fn build_egraph_with_string_llhd_div_extract_w_placement() {
        let mut egglog_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_file_path.push("resources/egglog/llhd_dfg_div_extract_w_placement.egg");
        let egglog_program: String = fs::read_to_string(egglog_file_path).unwrap();
        let egraph_info = build_egraph(&egglog_program);
        let egraph = egraph_info.0;
        let run_report_matches = egraph
            .get_run_report()
            .clone()
            .unwrap()
            .num_matches_per_rule
            .len();
        assert_eq!(
            1, run_report_matches,
            "There should be 1 rule match in program."
        );
        assert_eq!(
            27,
            egraph.num_tuples(),
            "There should be 27 facts remaining in the egraph."
        );
    }

    #[test]
    fn build_egraph_with_string_llhd_nested_expr() {
        let mut egglog_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        egglog_file_path.push("resources/egglog/llhd_dfg_nested_expr.egg");
        let egglog_program: String = fs::read_to_string(egglog_file_path).unwrap();
        let egraph_info = build_egraph(&egglog_program);
        let egraph = egraph_info.0;
        let run_report_matches = egraph
            .get_run_report()
            .clone()
            .unwrap()
            .num_matches_per_rule
            .len();
        assert_eq!(
            1, run_report_matches,
            "There should be 1 rule match in program."
        );
        assert_eq!(
            23,
            egraph.num_tuples(),
            "There should be 23 facts remaining in the egraph."
        );
    }
}

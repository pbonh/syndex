use egglog::*;

fn build_egraph(program: &str) -> EGraph {
    let mut egraph = EGraph::default();
    let _ = egraph.parse_and_run_program(program);
    egraph
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
        let egraph = build_egraph(&egglog_program);
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
        let egraph = build_egraph(&egglog_program);
        assert_eq!(
            4,
            egraph.num_tuples(),
            "There should be 4 facts remaining in the egraph."
        );
    }
}

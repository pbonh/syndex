use egglog::ast::Command;

mod datatype;
mod egglog_names;
mod inst;
mod unit;

type EgglogProgram = Vec<Command>;

#[cfg(test)]
mod tests {
    use egglog::EGraph;

    use super::*;

    extern crate utilities;

    #[test]
    fn valid_egglog_datatypes() {
        let dfg_datatype = inst::dfg();
        let mut egraph = EGraph::default();
        let egraph_msgs = egraph.run_program(dfg_datatype);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }
}

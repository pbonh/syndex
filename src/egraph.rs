use egglog::ast::Command;
use egglog::{EGraph, Error};

mod datatype;
mod egglog_names;
mod inst;
mod unit;

type EgglogProgram = Vec<Command>;

#[derive(Clone, Default)]
pub struct LLHDEgraph(EGraph);

impl TryFrom<EgglogProgram> for LLHDEgraph {
    type Error = Error;

    fn try_from(program: EgglogProgram) -> Result<Self, Self::Error> {
        let mut egraph = EGraph::default();
        let _llhd_inst_msgs = egraph.run_program(inst::dfg())?;
        match egraph.run_program(program) {
            Ok(_msgs) => Ok(Self(egraph)),
            Err(egraph_error) => Err(egraph_error),
        }
    }
}

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

    #[test]
    fn build_llhd_egraph() {
        let egraph_msgs = LLHDEgraph::try_from(vec![]);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }
}

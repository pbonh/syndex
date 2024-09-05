use egglog::ast::Command;
use egglog::{EGraph, Error};
use specs::World;

mod datatype;
mod egglog_names;
mod inst;
mod unit;

type EgglogProgram = Vec<Command>;

#[derive(Clone, Default)]
pub struct LLHDEGraph(EGraph);

impl TryFrom<EgglogProgram> for LLHDEGraph {
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

impl From<&World> for LLHDEGraph {
    fn from(_world: &World) -> Self {
        todo!()
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
        let program: EgglogProgram = Default::default();
        let egraph_msgs = LLHDEGraph::try_from(program);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn build_llhd_egraph_from_world() {
        let world = World::default();
        let _egraph = LLHDEGraph::from(&world);
    }
}

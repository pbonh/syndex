use std::ops::{Add, Deref, DerefMut};

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

impl Add for LLHDEGraph {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        rhs
    }
}

impl Deref for LLHDEGraph {
    type Target = EGraph;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LLHDEGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<EGraph> AsRef<EGraph> for LLHDEGraph
where
    EGraph: ?Sized,
    <Self as Deref>::Target: AsRef<EGraph>,
{
    fn as_ref(&self) -> &EGraph {
        self.deref().as_ref()
    }
}

impl<EGraph> AsMut<EGraph> for LLHDEGraph
where
    <Self as Deref>::Target: AsMut<EGraph>,
{
    fn as_mut(&mut self) -> &mut EGraph {
        self.deref_mut().as_mut()
    }
}

impl From<&World> for LLHDEGraph {
    fn from(_world: &World) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
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
    fn add_llhd_egraph_null() {
        let program1: EgglogProgram = Default::default();
        let egraph1_msgs = LLHDEGraph::try_from(program1);
        assert!(
            egraph1_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph1_msgs.err().unwrap()
        );
        let mut egraph1 = egraph1_msgs.unwrap();

        let egraph1_msgs_rules =
            utilities::load_egraph_rewrite_rules("llhd_div_extract.egg", &mut egraph1);
        assert!(egraph1_msgs_rules.is_ok());
        let egraph1_ruleset_symbols = (*egraph1).rulesets_symbols();
        assert_eq!(2, egraph1_ruleset_symbols.len());

        let combined_egraph = egraph1 + LLHDEGraph::default();
        let combined_egraph_ruleset_symbols = (*combined_egraph).rulesets_symbols();
        assert_eq!(2, combined_egraph_ruleset_symbols.len());
    }

    #[test]
    fn add_llhd_egraph() {
        let program1: EgglogProgram = Default::default();
        let program2: EgglogProgram = Default::default();
        let egraph1_msgs = LLHDEGraph::try_from(program1);
        assert!(
            egraph1_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph1_msgs.err().unwrap()
        );
        let egraph2_msgs = LLHDEGraph::try_from(program2);
        assert!(
            egraph2_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph2_msgs.err().unwrap()
        );

        let mut egraph1 = egraph1_msgs.unwrap();
        let mut egraph2 = egraph2_msgs.unwrap();

        let egraph1_msgs_rules =
            utilities::load_egraph_rewrite_rules("llhd_div_extract.egg", &mut egraph1);
        assert!(egraph1_msgs_rules.is_ok());
        let egraph1_ruleset_symbols = (*egraph1).rulesets_symbols();
        assert_eq!(2, egraph1_ruleset_symbols.len());
        let egraph2_msgs_rules =
            utilities::load_egraph_rewrite_rules("llhd_demorgans_theorem.egg", &mut egraph2);
        assert!(egraph2_msgs_rules.is_ok());
        let egraph2_ruleset_symbols = (*egraph2).rulesets_symbols();
        assert_eq!(2, egraph2_ruleset_symbols.len());

        let combined_egraph = egraph1 + egraph2;
        let combined_egraph_ruleset_symbols = (*combined_egraph).rulesets_symbols();
        assert_eq!(4, combined_egraph_ruleset_symbols.len());
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn build_llhd_egraph_from_world() {
        let world = World::default();
        let _egraph = LLHDEGraph::from(&world);
    }
}

use std::ops::{Add, Deref, DerefMut};

use datatype::LLHDEgglogSorts;
use egglog::ast::Command;
use egglog::{EGraph, Error};
use rules::LLHDEgglogRules;
use typed_builder::TypedBuilder;
pub use unit::LLHDEgglogFacts;

mod datatype;
mod egglog_names;
mod inst;
mod rules;
mod unit;

type EgglogProgram = Vec<Command>;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct LLHDEgglogProgram {
    #[builder(default=LLHDEgglogSorts::llhd_dfg())]
    sorts: LLHDEgglogSorts,

    #[builder(default)]
    rules: LLHDEgglogRules,

    #[builder(default)]
    facts: LLHDEgglogFacts,
}

impl LLHDEgglogProgram {
    const fn sorts(&self) -> &LLHDEgglogSorts {
        &self.sorts
    }

    const fn rules(&self) -> &LLHDEgglogRules {
        &self.rules
    }

    const fn facts(&self) -> &LLHDEgglogFacts {
        &self.facts
    }
}

impl Add for LLHDEgglogProgram {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.sorts.0.append(&mut rhs.sorts.0);
        self.rules.0.append(&mut rhs.rules.0);
        self.facts.0.append(&mut rhs.facts.0);
        self
    }
}

impl From<LLHDEgglogFacts> for LLHDEgglogProgram {
    fn from(_facts: LLHDEgglogFacts) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct LLHDEGraph(EGraph);

impl TryFrom<LLHDEgglogProgram> for LLHDEGraph {
    type Error = Error;

    fn try_from(program: LLHDEgglogProgram) -> Result<Self, Self::Error> {
        let mut egraph = EGraph::default();
        match egraph.run_program(program.sorts().to_owned().0) {
            Ok(_sorts_msgs) => match egraph.run_program(program.rules().to_owned().0) {
                Ok(_rules_msgs) => match egraph.run_program(program.facts().to_owned().0) {
                    Ok(_facts_msgs) => Ok(Self(egraph)),
                    Err(egraph_error) => Err(egraph_error),
                },
                Err(egraph_error) => Err(egraph_error),
            },
            Err(egraph_error) => Err(egraph_error),
        }
    }
}

impl Default for LLHDEGraph {
    fn default() -> Self {
        let mut egraph = EGraph::default();
        let llhd_inst_msgs = egraph.run_program(inst::dfg());
        if let Err(egraph_msg) = llhd_inst_msgs {
            panic!("Failure to load LLHD Prelude. Err: {:?}", egraph_msg);
        }
        Self(egraph)
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    extern crate utilities;

    #[test]
    fn build_llhd_egglog_program() {
        let llhd_egglog_program = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules("llhd_div_extract.egg"))
                    .unwrap(),
            )
            .build();
        let egraph_msgs = LLHDEGraph::try_from(llhd_egglog_program);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }

    #[test]
    fn add_llhd_egglog_programs() {
        let llhd_egglog_program_div_extract = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules("llhd_div_extract.egg"))
                    .unwrap(),
            )
            .build();
        assert_eq!(
            2,
            llhd_egglog_program_div_extract.rules().0.len(),
            "There should be 2 rules in div_extract program."
        );
        let llhd_egglog_program_demorgans_theorem = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules(
                    "llhd_demorgans_theorem.egg",
                ))
                .unwrap(),
            )
            .build();
        assert_eq!(
            2,
            llhd_egglog_program_demorgans_theorem.rules().0.len(),
            "There should be 2 rules in demorgans_theorem program."
        );
        let combined_program =
            llhd_egglog_program_div_extract + llhd_egglog_program_demorgans_theorem;
        assert_eq!(
            4,
            combined_program.rules().0.len(),
            "There should be 4 rules in combined program."
        );
    }

    #[test]
    fn default_llhd_egraph() {
        let _egraph = LLHDEGraph::default();
    }

    #[test]
    fn build_llhd_egraph() {
        let program: LLHDEgglogProgram = Default::default();
        let egraph_msgs = LLHDEGraph::try_from(program);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }

    #[test]
    fn egglog_program_from_llhd_unit() {
        let test_module = utilities::load_llhd_module("2and_1or_common.llhd");
        let llhd_egglog_program = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules("llhd_div_extract.egg"))
                    .unwrap(),
            )
            .facts(LLHDEgglogFacts::from_module(&test_module))
            .build();
        let _egraph = LLHDEGraph::try_from(llhd_egglog_program).unwrap();
    }
}

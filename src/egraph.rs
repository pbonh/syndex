use std::ops::{Add, Deref, DerefMut};

use datatype::EgglogSorts;
use egglog::ast::Command;
use egglog::{EGraph, Error};
use rules::LLHDEgglogRules;
use specs::World;
use typed_builder::TypedBuilder;
use unit::LLHDEgglogFacts;

mod datatype;
mod egglog_names;
mod inst;
mod rules;
mod unit;

type EgglogProgram = Vec<Command>;

#[derive(Debug, Clone, TypedBuilder)]
pub struct LLHDEgglogProgram {
    #[builder(default=EgglogSorts::llhd_dfg())]
    sorts: EgglogSorts,

    rules: LLHDEgglogRules,

    #[builder(default)]
    facts: LLHDEgglogFacts,
}

impl LLHDEgglogProgram {
    const fn sorts(&self) -> &EgglogSorts {
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
        self.rules.0.append(&mut rhs.rules.0);
        self.facts.0.append(&mut rhs.facts.0);
        self
    }
}

#[derive(Clone)]
pub struct LLHDEGraph(EGraph);

impl TryFrom<EgglogProgram> for LLHDEGraph {
    type Error = Error;

    fn try_from(program: EgglogProgram) -> Result<Self, Self::Error> {
        let mut egraph = Self::default();
        match egraph.run_program(program) {
            Ok(_msgs) => Ok(egraph),
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
    use std::str::FromStr;

    use super::*;

    extern crate utilities;

    #[test]
    fn build_llhd_egglog_program() {
        let _llhd_egglog_program = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules("llhd_div_extract.egg"))
                    .unwrap(),
            )
            .build();
    }

    #[test]
    fn add_llhd_egglog_programs() {
        let llhd_egglog_program_div_extract = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules("llhd_div_extract.egg"))
                    .unwrap(),
            )
            .build();
        let llhd_egglog_program_demorgans_theorem = LLHDEgglogProgram::builder()
            .rules(
                LLHDEgglogRules::from_str(&utilities::get_egglog_rules(
                    "llhd_demorgans_theorem.egg",
                ))
                .unwrap(),
            )
            .build();
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
        let program: EgglogProgram = Default::default();
        let egraph_msgs = LLHDEGraph::try_from(program);
        assert!(
            egraph_msgs.is_ok(),
            "Error loading LLHD DFG Datatype. Error: {:?}",
            egraph_msgs.err().unwrap()
        );
    }

    #[test]
    #[should_panic]
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
    #[should_panic]
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

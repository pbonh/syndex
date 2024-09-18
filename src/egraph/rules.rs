use std::ops::Deref;
use std::str::FromStr;

use egglog::ast::{Command, GenericRule, Symbol};
use egglog::{EGraph, Error};
use itertools::Itertools;

use crate::egraph::llhd::LLHDEGraph;
use crate::egraph::EgglogCommandList;

#[derive(Debug, Clone, Default)]
pub struct EgglogRules(EgglogCommandList);

type EgglogRule<Call, Var> = (Symbol, Symbol, GenericRule<Call, Var>);

impl EgglogRules {
    pub fn add_rules<SymbolList>(self, ruleset_names: SymbolList) -> Self
    where
        SymbolList: IntoIterator<Item = Command>,
    {
        let mut rulesets = ruleset_names
            .into_iter()
            .filter(|command| {
                matches!(*command, Command::AddRuleset(..))
                    || matches!(*command, Command::Rule { .. })
                    || matches!(*command, Command::Rewrite { .. })
                    || matches!(*command, Command::BiRewrite { .. })
            })
            .collect_vec();
        let mut updated_rulesets = Self(self.0);
        updated_rulesets.0.append(&mut rulesets);
        updated_rulesets
    }

    pub fn add_rule_str(self, rule_str: &str) -> Self {
        match EGraph::default().parse_program(None, rule_str) {
            Ok(rule_commands) => Self::add_rules(self, rule_commands),
            Err(error) => panic!("Failure to build rules from string: {:?}", error),
        }
    }
}

impl Deref for EgglogRules {
    type Target = EgglogCommandList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<EgglogCommandList> AsRef<EgglogCommandList> for EgglogRules
where
    EgglogCommandList: ?Sized,
    <Self as Deref>::Target: AsRef<EgglogCommandList>,
{
    fn as_ref(&self) -> &EgglogCommandList {
        self.deref().as_ref()
    }
}

impl Into<EgglogCommandList> for EgglogRules {
    fn into(self) -> EgglogCommandList {
        self.0
    }
}

impl IntoIterator for EgglogRules {
    type Item = Command;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LLHDEgglogRules(pub(in crate::egraph) EgglogCommandList);

impl FromStr for LLHDEgglogRules {
    type Err = Error;

    fn from_str(rule_str: &str) -> Result<Self, Self::Err> {
        let llhd_egraph = LLHDEGraph::default();
        match (*llhd_egraph).parse_program(None, rule_str) {
            Ok(rule_cmds) => Ok(Self(rule_cmds)),
            Err(err_msgs) => Err(err_msgs),
        }
    }
}

impl Into<EgglogCommandList> for LLHDEgglogRules {
    fn into(self) -> EgglogCommandList {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use egglog::ast::GenericCommand;

    use super::*;

    #[test]
    fn create_egglog_rules_from_ruleset() {
        let ruleset_symbol = Symbol::new("rule1");
        let rule1 = GenericCommand::AddRuleset(ruleset_symbol);
        let egglog_rules = EgglogRules::default().add_rules(vec![rule1]);
        assert_eq!(
            1,
            egglog_rules.len(),
            "There should be 1 command in ruleset."
        );
        assert!(
            matches!(egglog_rules[0], GenericCommand::AddRuleset(..)),
            "First command should be a ruleset."
        );
        if let GenericCommand::AddRuleset(rule_symbol) = egglog_rules[0] {
            assert_eq!("rule1", rule_symbol.as_str(), "Rule name does not match.");
        }
    }

    #[test]
    fn create_egglog_rules_from_str() {
        let egglog_rule_str = utilities::get_egglog_commands("llhd_div_extract.egg");
        let egglog_rules = EgglogRules::default().add_rule_str(&egglog_rule_str);
        assert_eq!(
            2,
            egglog_rules.len(),
            "There should be 2 command in rules, 1 ruleset and 1 rule."
        );
        assert!(
            matches!(egglog_rules[0], GenericCommand::AddRuleset(..)),
            "First command should be a ruleset."
        );
        assert!(
            matches!(egglog_rules[1], GenericCommand::Rewrite { .. }),
            "Second command should be a rule."
        );
        if let GenericCommand::AddRuleset(rule_symbol) = egglog_rules[0] {
            assert_eq!("div-ext", rule_symbol.as_str(), "Rule name does not match.");
        }
    }

    #[test]
    fn create_llhd_rules_from_str() {
        let mut llhd_egraph = LLHDEGraph::default();
        let rule_cmds_result =
            LLHDEgglogRules::from_str(&utilities::get_egglog_commands("llhd_div_extract.egg"));
        if let Err(err_msg) = rule_cmds_result {
            panic!("Failure to parse LLHD Egglog rules. Err: {:?}", err_msg);
        }
        let rule_cmds = rule_cmds_result.unwrap();
        let egraph_with_rules_msgs = llhd_egraph.run_program(rule_cmds.clone().into());
        assert!(egraph_with_rules_msgs.is_ok());
        assert_eq!(
            2,
            <LLHDEgglogRules as Into<EgglogCommandList>>::into(rule_cmds).len(),
            "There should be 1 rule present in rewrite(2 since there is always null ruleset)."
        );
    }
}

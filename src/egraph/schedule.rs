use std::ops::Deref;

use egglog::ast::Command;
use egglog::EGraph;
use itertools::Itertools;

use crate::egraph::EgglogCommandList;

#[derive(Debug, Clone, Default)]
pub struct EgglogSchedule(EgglogCommandList);

impl EgglogSchedule {
    pub fn add_schedule<CommandList>(mut self, schedule_list: CommandList) -> Self
    where
        CommandList: IntoIterator<Item = Command>,
    {
        let mut schedules = schedule_list
            .into_iter()
            .filter(|command| matches!(*command, Command::RunSchedule(..)))
            .collect_vec();
        self.0.append(&mut schedules);
        self
    }

    pub fn add_schedule_str(mut self, rule_str: &str) -> Self {
        match EGraph::default().parse_program(None, rule_str) {
            Ok(mut rule_commands) => {
                self.0.append(&mut rule_commands);
                self
            }
            Err(error) => panic!("Failure to build schedule from string: {:?}", error),
        }
    }
}

impl Deref for EgglogSchedule {
    type Target = EgglogCommandList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<EgglogCommandList> AsRef<EgglogCommandList> for EgglogSchedule
where
    EgglogCommandList: ?Sized,
    <Self as Deref>::Target: AsRef<EgglogCommandList>,
{
    fn as_ref(&self) -> &EgglogCommandList {
        self.deref().as_ref()
    }
}

impl Into<EgglogCommandList> for EgglogSchedule {
    fn into(self) -> EgglogCommandList {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use egglog::ast::{GenericCommand, GenericRunConfig, Schedule, Symbol, DUMMY_SPAN};

    use super::*;

    #[test]
    fn create_egglog_schedule_from_cmd() {
        let schedule1 = GenericCommand::RunSchedule(Schedule::Run(
            DUMMY_SPAN.clone(),
            GenericRunConfig {
                ruleset: Symbol::new("schedule1"),
                until: None,
            },
        ));
        let egglog_sorts = EgglogSchedule::default().add_schedule(vec![schedule1]);
        assert_eq!(
            1,
            egglog_sorts.len(),
            "There should be 1 commands present, one for each schedule."
        );
    }
}

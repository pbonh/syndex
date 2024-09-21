use std::str::FromStr;

use egglog::Error;

use crate::egraph::EgglogCommandList;
use crate::llhd_egraph::llhd::LLHDEGraph;

#[derive(Debug, Clone, Default)]
pub struct LLHDEgglogSchedules(pub(in crate::llhd_egraph) EgglogCommandList);

impl FromStr for LLHDEgglogSchedules {
    type Err = Error;

    fn from_str(schedule_str: &str) -> Result<Self, Self::Err> {
        let llhd_egraph = LLHDEGraph::default();
        match (*llhd_egraph).parse_program(None, schedule_str) {
            Ok(schedule_cmds) => Ok(Self(schedule_cmds)),
            Err(err_msgs) => Err(err_msgs),
        }
    }
}

impl From<LLHDEgglogSchedules> for EgglogCommandList {
    fn from(schedules: LLHDEgglogSchedules) -> Self {
        schedules.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_llhd_schedules_from_str() {
        let mut llhd_egraph = LLHDEGraph::default();
        let schedule_cmds_result = LLHDEgglogSchedules::from_str(&utilities::get_egglog_commands(
            "llhd_div_extract_schedule.egg",
        ));
        if let Err(err_msg) = schedule_cmds_result {
            panic!("Failure to parse LLHD Egglog schedules. Err: {:?}", err_msg);
        }
        let schedule_cmds = schedule_cmds_result.unwrap();
        let egraph_with_schedules_msgs = llhd_egraph.run_program(schedule_cmds.clone().into());
        assert!(egraph_with_schedules_msgs.is_ok());
        assert_eq!(
            3,
            <LLHDEgglogSchedules as Into<EgglogCommandList>>::into(schedule_cmds).len(),
            "There should be 1 schedule present in program."
        );
    }
}

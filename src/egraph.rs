mod datatype;
mod egglog_names;
pub mod facts;
mod inst;
pub mod rules;
pub mod schedule;
pub mod sorts;
mod unit;
use facts::EgglogFacts;
use rules::EgglogRules;
use schedule::EgglogSchedule;
use sorts::EgglogSorts;
pub use unit::LLHDEgglogFacts;
pub mod llhd;

use egglog::ast::Command;
use frunk::monoid::Monoid;
use frunk::semigroup::Semigroup;

type EgglogCommandList = Vec<Command>;

#[derive(Debug, Clone, Default)]
pub struct EgglogProgram {
    sorts: EgglogSorts,
    facts: EgglogFacts,
    rules: EgglogRules,
    schedule: EgglogSchedule,
}

impl Semigroup for EgglogProgram {
    fn combine(&self, _program_update: &Self) -> Self {
        todo!()
    }
}

impl Monoid for EgglogProgram {
    fn empty() -> Self {
        Self::default()
    }
}

// impl Into<EgglogCommandList> for EgglogProgram {
//     fn into(self) -> EgglogCommandList {
//         self.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_empty_egglog_program() {
        let _empty_egglog_program = EgglogProgram::default();
    }

    #[test]
    fn init_egglog_program() {}

    use bon::builder;

    #[builder]
    fn greet(name: &str, level: Option<u32>) -> String {
        let level = level.unwrap_or(0);

        format!("Hello {name}! Your level is {level}")
    }

    #[test]
    fn bon_ordered_builder_free_function() {
        let greeting = greet()
            .level(24) // <- setting `level` is optional, we could omit it
            .name("Bon")
            .call();

        assert_eq!(greeting, "Hello Bon! Your level is 24");
    }
}

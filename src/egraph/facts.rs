use std::ops::Deref;

use egglog::ast::Command;
use itertools::Itertools;

use crate::egraph::EgglogCommandList;

#[derive(Debug, Clone, Default)]
pub struct EgglogFacts(EgglogCommandList);

impl EgglogFacts {
    pub fn add_facts<CommandList>(mut self, sort_list: CommandList) -> Self
    where
        CommandList: IntoIterator<Item = Command>,
    {
        let mut sorts = sort_list
            .into_iter()
            .filter(|command| matches!(*command, Command::Action(..)))
            .collect_vec();
        self.0.append(&mut sorts);
        self
    }
}

impl Deref for EgglogFacts {
    type Target = EgglogCommandList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<EgglogCommandList> AsRef<EgglogCommandList> for EgglogFacts
where
    EgglogCommandList: ?Sized,
    <Self as Deref>::Target: AsRef<EgglogCommandList>,
{
    fn as_ref(&self) -> &EgglogCommandList {
        self.deref().as_ref()
    }
}

impl Into<EgglogCommandList> for EgglogFacts {
    fn into(self) -> EgglogCommandList {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use egglog::ast::{Action, Expr, GenericCommand, Literal, Symbol, DUMMY_SPAN};

    use super::*;

    #[test]
    fn create_egglog_facts_from_cmd() {
        let let_stmt1 = GenericCommand::Action(Action::Let(
            DUMMY_SPAN.clone(),
            Symbol::new("var1"),
            Expr::Lit(DUMMY_SPAN.clone(), Literal::UInt(0)),
        ));
        let egglog_sorts = EgglogFacts::default().add_facts(vec![let_stmt1]);
        assert_eq!(
            1,
            egglog_sorts.len(),
            "There should be 1 commands present, one for each action stmt."
        );
    }
}

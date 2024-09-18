use std::ops::Deref;

use egglog::ast::Command;
use itertools::Itertools;

use crate::egraph::EgglogCommandList;

#[derive(Debug, Clone, Default)]
pub struct EgglogSorts(EgglogCommandList);

impl EgglogSorts {
    pub fn add_sorts<CommandList>(mut self, sort_list: CommandList) -> Self
    where
        CommandList: IntoIterator<Item = Command>,
    {
        let mut sorts = sort_list
            .into_iter()
            .filter(|command| {
                matches!(*command, Command::Sort(..))
                    || matches!(*command, Command::Datatype { .. })
                    || matches!(*command, Command::Relation { .. })
                    || matches!(*command, Command::Function(..))
            })
            .collect_vec();
        self.0.append(&mut sorts);
        self
    }
}

impl Deref for EgglogSorts {
    type Target = EgglogCommandList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<EgglogCommandList> AsRef<EgglogCommandList> for EgglogSorts
where
    EgglogCommandList: ?Sized,
    <Self as Deref>::Target: AsRef<EgglogCommandList>,
{
    fn as_ref(&self) -> &EgglogCommandList {
        self.deref().as_ref()
    }
}

impl Into<EgglogCommandList> for EgglogSorts {
    fn into(self) -> EgglogCommandList {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use egglog::ast::{GenericActions, GenericCommand, Schema, Symbol, DUMMY_SPAN};

    use super::*;

    #[test]
    fn create_egglog_sorts_from_cmd() {
        let sort1 = GenericCommand::Sort(DUMMY_SPAN.clone(), Symbol::new("sort1"), None);
        let datatype1 = GenericCommand::Datatype {
            span: DUMMY_SPAN.clone(),
            name: Symbol::new("datatype1"),
            variants: vec![],
        };
        let relation1 = GenericCommand::Relation {
            span: DUMMY_SPAN.clone(),
            constructor: Symbol::new("relation1"),
            inputs: vec![],
        };
        let function1 = GenericCommand::Function(egglog::ast::GenericFunctionDecl {
            name: Symbol::new("func1"),
            schema: Schema {
                input: vec![],
                output: Symbol::new("func1_out"),
            },
            default: None,
            merge: None,
            merge_action: GenericActions::default(),
            cost: None,
            unextractable: false,
            ignore_viz: false,
            span: DUMMY_SPAN.clone(),
        });
        let egglog_sorts =
            EgglogSorts::default().add_sorts(vec![sort1, datatype1, relation1, function1]);
        assert_eq!(
            4,
            egglog_sorts.len(),
            "There should be 4 commands present, one for each declaration."
        );
    }
}

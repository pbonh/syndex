use common::filter_nullary;
use enode::LLHDENode;
use llhd::ir::{Inst, InstData, Module, Opcode, UnitId, Value, ValueData};
use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

/// LLHD Inst `ENode` Type
pub mod enode;

/// Helper Functions for LLHD Types
pub mod common;

/// `Net` Identifier within LLHD `Unit`
pub type LLHDNet = (UnitId, Value, Opcode);

/// `NewType` Wrapper for an LLHD Module
pub struct LModule(Module);

impl LModule {
    pub(crate) const fn new(module: Module) -> Self {
        Self(module)
    }

    pub(crate) fn get(&self, net: LLHDNet) -> LLHDENode {
        match &self.0.unit(net.0)[net.1] {
            ValueData::Inst { ty, inst } => LLHDENode {
                id: net.1,
                ty: ty.clone(),
                data: self.0.unit(net.0)[*inst].clone(),
            },
            ValueData::Arg { ty, arg: _arg } => LLHDENode {
                id: net.1,
                ty: ty.clone(),
                data: InstData::default(),
            },
            _ => panic!(),
        }
    }

    pub(crate) fn all_args(&self, unit_id: UnitId) -> Vec<Value> {
        let unit = self.0.unit(unit_id);
        unit.input_args().collect()
    }

    pub(crate) fn all_insts(&self, unit_id: UnitId) -> Vec<Inst> {
        let unit = self.0.unit(unit_id);
        unit.all_insts()
            .filter(|inst| filter_nullary(&unit, inst))
            .collect()
    }

    pub(crate) fn all_args_data(&self, unit_id: UnitId) -> Vec<LLHDENode> {
        let unit = self.0.unit(unit_id);
        unit.input_args()
            .map(|arg| LLHDENode {
                id: arg,
                ty: unit.value_type(arg),
                data: InstData::default(),
            })
            .collect()
    }

    pub(crate) fn all_insts_data(&self, unit_id: UnitId) -> Vec<LLHDENode> {
        let unit = self.0.unit(unit_id);
        unit.all_insts()
            .filter(|inst| filter_nullary(&unit, inst))
            .map(|inst| LLHDENode {
                id: unit.inst_result(inst),
                ty: unit.inst_type(inst),
                data: unit[inst].clone(),
            })
            .collect()
    }

    pub(crate) fn all_nets(&self, unit_id: UnitId) -> impl Iterator<Item = LLHDNet> + '_ {
        let unit = self.0.unit(unit_id);
        unit.input_args()
            .map(move |arg| (unit.id(), arg, InstData::default().opcode()))
            .chain(
                unit.all_insts()
                    .filter(move |inst| filter_nullary(&unit, inst))
                    .map(move |inst| (unit.id(), unit.inst_result(inst), unit[inst].opcode())),
            )
    }
}

impl Deref for LModule {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Module> AsRef<Module> for LModule
where
    Module: ?Sized,
    <LModule as Deref>::Target: AsRef<Module>,
{
    fn as_ref(&self) -> &Module {
        self.deref().as_ref()
    }
}

impl<Module> AsMut<Module> for LModule
where
    <LModule as Deref>::Target: AsMut<Module>,
{
    fn as_mut(&mut self) -> &mut Module {
        self.deref_mut().as_mut()
    }
}

impl From<Module> for LModule {
    fn from(module: Module) -> Self {
        Self(module)
    }
}

impl Default for LModule {
    fn default() -> Self {
        LModule(Module::new())
    }
}

impl fmt::Debug for LModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.dump().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llhd::table::TableKey;

    #[test]
    fn simple_module_creation_via_default() {
        let _ = LModule::default();
    }

    #[test]
    fn simple_module_creation_via_constructor() {
        let llhd_module = Module::new();
        let _ = LModule::new(llhd_module);
    }

    #[test]
    fn simple_module_creation_via_newtype() {
        let llhd_module = Module::new();
        let _ = LModule(llhd_module);
    }

    #[test]
    fn simple_module_creation_via_from() {
        let llhd_module = Module::new();
        let _ = LModule::from(llhd_module);
    }

    #[test]
    fn two_unit_llhd_module_iterate_unit_inst_data() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let unit_args = llhd_module.all_args(unit_id);
        let unit_args_data = llhd_module.all_args_data(unit_id);
        let unit_insts = llhd_module.all_insts(unit_id);
        let unit_insts_data = llhd_module.all_insts_data(unit_id);
        assert_eq!(3, unit_args.len(), "Unit should contain 3 Args");
        assert_eq!(2, unit_insts.len(), "Unit should contain 2 Instructions");
        assert_eq!(3, unit_args_data.len(), "Unit should contain 3 ArgData's");
        assert_eq!(2, unit_insts_data.len(), "Unit should contain 2 InstData's");
        let unit_net: Vec<_> = llhd_module.all_nets(unit_id).collect();
        assert_eq!(
            5,
            unit_net.len(),
            "Unit should contain 5 Nets(Combined Args & Insts)."
        );
    }

    #[test]
    fn llhd_module_get_net() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let net_id = (unit_id, Value::new(3), Opcode::And);
        let net_data = llhd_module.get(net_id);
        assert_eq!(
            Opcode::And,
            net_data.data.opcode(),
            "Opcode should be And type."
        );
    }
}

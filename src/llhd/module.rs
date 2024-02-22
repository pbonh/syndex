use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use llhd::ir::{Inst, InstData, Module, UnitId, Value, ValueData};

use super::common::filter_nullary;
use super::enode::LLHDENode;
use super::{LLHDInst, LLHDNet};

type InstNameMap = HashMap<LLHDInst, String>;
type NameInstMap = HashMap<(UnitId, String), Inst>;

/// `NewType` Wrapper for an LLHD Module
pub struct LModule {
    module: Module,
    inst_name_map: InstNameMap,
    name_inst_map: NameInstMap,
}

impl LModule {
    pub(crate) fn new(module: Module) -> Self {
        let init = Self {
            module,
            ..Default::default()
        };
        let mut name_inst_map = NameInstMap::default();
        init.module.units().for_each(|unit| {
            let unit_id = unit.id();
            init.all_insts(unit_id).into_iter().for_each(|inst| {
                let net_name = unit.inst_result(inst).to_string();
                name_inst_map.insert((unit_id, net_name), inst);
            });
        });
        Self {
            module: init.module,
            inst_name_map: HashMap::default(),
            name_inst_map,
        }
    }

    pub(crate) fn get(&self, net: LLHDNet) -> LLHDENode {
        match &self.module.unit(net.0)[net.1] {
            ValueData::Inst { ty, inst } => LLHDENode {
                id: net.1,
                ty: ty.clone(),
                data: self.module.unit(net.0)[*inst].clone(),
            },
            ValueData::Arg { ty, arg: _arg } => LLHDENode {
                id: net.1,
                ty: ty.clone(),
                data: InstData::default(),
            },
            _ => panic!(),
        }
    }

    pub(crate) fn get_unit_name(&self, unit_id: UnitId) -> String {
        let unit = self.module.unit(unit_id);
        unit.name().to_string()
    }

    pub(crate) fn get_inst_name(&self, llhd_inst: LLHDInst) -> String {
        let unit_id = llhd_inst.0;
        let inst_id = llhd_inst.1;
        let unit = self.module.unit(unit_id);
        unit.inst_result(inst_id).to_string()
        // inst_id
        //     .dump(&unit)
        //     .to_string()
        //     .split_once(char::is_whitespace)
        //     .expect("Instruction contains no whitespace, can't split off instruction name")
        //     .0
        //     .to_owned()
    }

    pub(crate) fn get_inst(&self, unit_id: UnitId, name: &str) -> Inst {
        self.name_inst_map[&(unit_id, name.to_owned())]
    }

    pub(crate) fn all_args(&self, unit_id: UnitId) -> Vec<Value> {
        let unit = self.module.unit(unit_id);
        unit.input_args().collect()
    }

    pub(crate) fn all_insts(&self, unit_id: UnitId) -> Vec<Inst> {
        let unit = self.module.unit(unit_id);
        unit.all_insts()
            .filter(|inst| filter_nullary(&unit, inst))
            .collect()
    }

    pub(crate) fn all_args_data(&self, unit_id: UnitId) -> Vec<LLHDENode> {
        let unit = self.module.unit(unit_id);
        unit.input_args()
            .map(|arg| LLHDENode {
                id: arg,
                ty: unit.value_type(arg),
                data: InstData::default(),
            })
            .collect()
    }

    pub(crate) fn all_insts_data(&self, unit_id: UnitId) -> Vec<LLHDENode> {
        let unit = self.module.unit(unit_id);
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
        let unit = self.module.unit(unit_id);
        unit.input_args().map(move |arg| (unit.id(), arg)).chain(
            unit.all_insts()
                .filter(move |inst| filter_nullary(&unit, inst))
                .map(move |inst| (unit.id(), unit.inst_result(inst))),
        )
    }
}

impl Deref for LModule {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl DerefMut for LModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.module
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
    <Self as Deref>::Target: AsMut<Module>,
{
    fn as_mut(&mut self) -> &mut Module {
        self.deref_mut().as_mut()
    }
}

impl From<Module> for LModule {
    fn from(module: Module) -> Self {
        Self::new(module)
    }
}

impl Default for LModule {
    fn default() -> Self {
        Self {
            module: Module::new(),
            inst_name_map: HashMap::new(),
            name_inst_map: HashMap::new(),
        }
    }
}

impl fmt::Debug for LModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.module.dump().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use llhd::ir::prelude::*;
    use llhd::table::TableKey;

    use super::*;

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
        let net_id = (unit_id, Value::new(3));
        let net_data = llhd_module.get(net_id);
        assert_eq!(
            Opcode::And,
            net_data.data.opcode(),
            "Opcode should be And type."
        );
    }

    #[test]
    fn llhd_module_get_unit_name() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let unit_name = llhd_module.get_unit_name(unit_id);
        assert_eq!("@ent2", unit_name, "Unit name does not match stored name.");
    }

    #[test]
    fn llhd_module_get_net_name() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let net_id = (unit_id, Inst::new(1));
        let net_name = llhd_module.get_inst_name(net_id);
        // assert_eq!("%and1", net_name, "Inst name does not match");
        assert_eq!("v3", net_name, "Inst name does not match");
    }

    #[test]
    fn llhd_module_get_net_id() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let net_id = llhd_module.get_inst(unit_id, "v3");
        let and_net_id = (unit_id, Inst::new(1));
        assert_eq!(and_net_id.1, net_id, "Inst Id's do not match");
    }
}

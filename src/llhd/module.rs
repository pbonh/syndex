use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use llhd::ir::{Inst, InstData, Module, UnitId, Value, ValueData};
use llhd::table::TableKey;

use super::common::{filter_nullary, get_inst_name, get_unit_name};
use super::enode::LLHDENode;
use super::{LLHDInst, LLHDNet};

type NameUnitMap = HashMap<String, UnitId>;
type InstNameMap = HashMap<LLHDInst, String>;
type NameInstMap = HashMap<(UnitId, String), Inst>;

/// `NewType` Wrapper for an LLHD Module
pub struct LModule {
    module: Module,
    name_unit_map: NameUnitMap,
    name_inst_map: NameInstMap,
    inst_name_map: InstNameMap,
}

impl LModule {
    pub(crate) fn new(module: Module) -> Self {
        let init = Self {
            module,
            ..Default::default()
        };
        let mut name_inst_map = NameInstMap::default();
        let mut name_unit_map = NameUnitMap::default();
        init.module.units().for_each(|unit| {
            let unit_id = unit.id();
            let unit_name = get_unit_name(&unit);
            name_unit_map.insert(unit_name, unit_id);
            init.all_insts(unit_id).into_iter().for_each(|inst| {
                let net_name = get_inst_name(&init.module, &unit, inst);
                name_inst_map.insert((unit_id, net_name), inst);
            });
        });
        Self {
            module: init.module,
            name_unit_map,
            name_inst_map,
            inst_name_map: HashMap::default(),
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
        get_unit_name(&unit)
    }

    pub(crate) fn get_unit_id(&self, unit_name: &str) -> UnitId {
        self.name_unit_map[unit_name]
    }

    pub(crate) fn get_unit_id_from_inst(&self, llhd_inst: LLHDInst) -> Option<UnitId> {
        let unit_id = llhd_inst.0;
        let inst_id = llhd_inst.1;
        if let InstData::Call { unit, .. } = &self.module.unit(unit_id)[inst_id] {
            match self
                .module
                .lookup_ext_unit(*unit, unit_id)
                .expect("ExtUnit does not exist in Module.")
            {
                llhd::ir::LinkedUnit::Def(ext_unit_id) => Some(ext_unit_id),
                llhd::ir::LinkedUnit::Decl(_) => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_inst_name(&self, llhd_inst: LLHDInst) -> String {
        let unit_id = llhd_inst.0;
        let inst_id = llhd_inst.1;
        let parent_unit = self.module.unit(unit_id);
        get_inst_name(&self.module, &parent_unit, inst_id)
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
            .filter(|inst| filter_nullary(&unit, *inst))
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
            .filter(|inst| filter_nullary(&unit, *inst))
            .map(|inst| LLHDENode {
                id: unit.get_inst_result(inst).unwrap_or_else(|| Value::new(usize::max_value())),
                ty: unit.inst_type(inst),
                data: unit[inst].clone(),
            })
            .collect()
    }

    pub(crate) fn all_nets(&self, unit_id: UnitId) -> impl Iterator<Item = LLHDNet> + '_ {
        let unit = self.module.unit(unit_id);
        unit.input_args().map(move |arg| (unit.id(), arg)).chain(
            unit.all_insts()
                .filter(move |inst| filter_nullary(&unit, *inst))
                .map(move |inst| {
                    (
                        unit.id(),
                        unit.get_inst_result(inst).unwrap_or_else(|| Value::new(usize::max_value())),
                    )
                }),
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
            name_unit_map: HashMap::new(),
            name_inst_map: HashMap::new(),
            inst_name_map: HashMap::new(),
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
    use llhd::ir::ExtUnit;

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
    fn llhd_module_get_unit_id_from_name() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_name = "@ent2".to_owned();
        let unit_id = llhd_module.get_unit_id(&unit_name);
        assert_eq!(
            UnitId::new(0),
            unit_id,
            "Unit name does not match stored name."
        );
    }

    #[test]
    fn llhd_module_get_inst_name() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let inst_id = (unit_id, Inst::new(1));
        let inst_name = llhd_module.get_inst_name(inst_id);
        assert_eq!("and.v3", inst_name, "Inst name does not match");
    }

    #[test]
    fn llhd_module_get_inst_name_for_cell() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                drv i1$ %out1, %and1, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let inst_info = llhd_module
            .units()
            .map(|module_unit| {
                let unit_id = module_unit.id();
                let unit_name = module_unit.name().to_string();
                (module_unit, unit_id, unit_name)
            })
            .filter(|(_, _, unit_name)| unit_name == "@top")
            .map(|(module_unit, unit_id, _)| (module_unit, unit_id))
            .flat_map(|(module_unit, unit_id)| {
                module_unit
                    .all_insts()
                    .filter(move |inst| match &module_unit[*inst] {
                        InstData::Call { .. } => true,
                        _ => false,
                    })
                    .map(move |inst| (unit_id, inst, module_unit[inst].to_owned()))
                    .map(|(unit_id, inst, inst_data)| {
                        let mut ext_unit_id = ExtUnit::new(0);
                        if let InstData::Call { unit, .. } = inst_data {
                            ext_unit_id = unit;
                        }
                        (unit_id, inst, ext_unit_id)
                    })
            })
            .collect::<Vec<(UnitId, Inst, ExtUnit)>>();
        let (and_inst_parent_unit_id, and_inst_id) = (inst_info[0].0, inst_info[0].1);
        let and_inst_name = llhd_module.get_inst_name((and_inst_parent_unit_id, and_inst_id));
        assert_eq!("%top.and.i7", and_inst_name, "Inst name does not match");
    }

    #[test]
    fn llhd_module_get_inst_id() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.units().next().unwrap().id();
        let inst_id = llhd_module.get_inst(unit_id, "and.v3");
        let and_inst_id = (unit_id, Inst::new(1));
        assert_eq!(and_inst_id.1, inst_id, "Inst Id's do not match");
    }

    #[test]
    fn llhd_module_get_unit_id_from_inst() {
        let input = indoc::indoc! {"
            proc %top.always_ff.227.0 (i1$ %0, i1$ %1, i32$ %2) -> (i32$ %3) {
            %4:
                %const_i1_0 = const i1 0
                br %init
            %init:
                %clk = prb i1$ %0
                %rst_n = prb i1$ %1
                wait %check, %0, %1
            %check:
                %clk0 = prb i1$ %0
                %7 = eq i1 %clk, %const_i1_0
                %8 = neq i1 %clk0, %const_i1_0
                %posedge = and i1 %7, %8
                %rst_n0 = prb i1$ %1
                %9 = eq i1 %rst_n0, %const_i1_0
                %10 = neq i1 %rst_n, %const_i1_0
                %negedge = and i1 %9, %10
                %event_or = or i1 %posedge, %negedge
                br %event_or, %init, %event
            %event:
                %rst_n1 = prb i1$ %1
                %12 = not i1 %rst_n1
                %epsilon = const time 0s 1e
                br %12, %if_false, %if_true
            %if_true:
                %const_i32_0 = const i32 0
                drv i32$ %3, %const_i32_0, %epsilon
                wait %16 for %epsilon
            %if_false:
                %count = prb i32$ %2
                %const_i32_1 = const i32 1
                %17 = add i32 %count, %const_i32_1
                drv i32$ %3, %17, %epsilon
                wait %20 for %epsilon
            %if_exit:
                br %4
            %16:
                br %if_exit
            %20:
                br %if_exit
            }

            proc %top.initial.228.0 (i1$ %0) -> (i1$ %1, i1$ %2) {
            %3:
                %time_1e = const time 0s 1e
                %time_1ns = const time 1ns
                %const_i1_0 = const i1 0
                %const_i1_1 = const i1 1
                wait %5 for %time_1ns
            %5:
                drv i1$ %2, %const_i1_0, %time_1e
                wait %8 for %time_1e
            %8:
                wait %10 for %time_1ns
            %10:
                drv i1$ %2, %const_i1_1, %time_1e
                wait %13 for %time_1e
            %13:
                wait %15 for %time_1ns
            %15:
                %clk = prb i1$ %1
                %16 = not i1 %clk
                drv i1$ %1, %16, %time_1e
                wait %19 for %time_1e
            %19:
                wait %21 for %time_1ns
            %21:
                %clk0 = prb i1$ %1
                %22 = not i1 %clk0
                drv i1$ %1, %22, %time_1e
                wait %25 for %time_1e
            %25:
                wait %27 for %time_1ns
            %27:
                %clk1 = prb i1$ %1
                %28 = not i1 %clk1
                drv i1$ %1, %28, %time_1e
                wait %31 for %time_1e
            %31:
                wait %33 for %time_1ns
            %33:
                %clk2 = prb i1$ %1
                %34 = not i1 %clk2
                drv i1$ %1, %34, %time_1e
                wait %37 for %time_1e
            %37:
                wait %39 for %time_1ns
            %39:
                %clk3 = prb i1$ %1
                %40 = not i1 %clk3
                drv i1$ %1, %40, %time_1e
                wait %43 for %time_1e
            %43:
                wait %45 for %time_1ns
            %45:
                %clk4 = prb i1$ %1
                %46 = not i1 %clk4
                drv i1$ %1, %46, %time_1e
                wait %49 for %time_1e
            %49:
                wait %51 for %time_1ns
            %51:
                %clk5 = prb i1$ %1
                %52 = not i1 %clk5
                drv i1$ %1, %52, %time_1e
                wait %55 for %time_1e
            %55:
                wait %57 for %time_1ns
            %57:
                %clk6 = prb i1$ %1
                %58 = not i1 %clk6
                drv i1$ %1, %58, %time_1e
                wait %61 for %time_1e
            %61:
                wait %63 for %time_1ns
            %63:
                halt
            }

            entity @top () -> () {
                %clk_init = const i1 0
                %clk = sig i1 %clk_init
                %rst_n_init = const i1 1
                %rst_n = sig i1 %rst_n_init
                %count_init = const i32 99
                %count = sig i32 %count_init
                inst %top.always_ff.227.0 (i1$ %clk, i1$ %rst_n, i32$ %count) -> (i32$ %count)
                inst %top.initial.228.0 (i1$ %clk) -> (i1$ %clk, i1$ %rst_n)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let inst_info = llhd_module
            .units()
            .map(|module_unit| {
                let unit_id = module_unit.id();
                let unit_name = module_unit.name().to_string();
                (module_unit, unit_id, unit_name)
            })
            .filter(|(_, _, unit_name)| unit_name == "@top")
            .map(|(module_unit, unit_id, _)| (module_unit, unit_id))
            .flat_map(|(module_unit, unit_id)| {
                module_unit
                    .all_insts()
                    .filter(move |inst| match &module_unit[*inst] {
                        InstData::Call { .. } => true,
                        _ => false,
                    })
                    .map(move |inst| (unit_id, inst, module_unit[inst].to_owned()))
                    .map(|(unit_id, inst, inst_data)| {
                        let mut ext_unit_id = ExtUnit::new(0);
                        if let InstData::Call { unit, .. } = inst_data {
                            ext_unit_id = unit;
                        }
                        (unit_id, inst, ext_unit_id)
                    })
            })
            .collect::<Vec<(UnitId, Inst, ExtUnit)>>();
        let (always_ff_parent_unit_id, always_ff_inst_id, _always_ff_template_id) =
            inst_info[0].to_owned();
        let (initial_parent_unit_id, initial_inst_id, _initial_template_id) =
            inst_info[1].to_owned();
        let always_ff_unit_id = llhd_module
            .get_unit_id_from_inst((always_ff_parent_unit_id, always_ff_inst_id))
            .unwrap();
        let initial_unit_id = llhd_module
            .get_unit_id_from_inst((initial_parent_unit_id, initial_inst_id))
            .unwrap();
        let always_ff_parent_unit_name = "%top.always_ff.227.0".to_owned();
        let initial_parent_unit_name = "%top.initial.228.0".to_owned();
        assert_eq!(
            always_ff_parent_unit_name,
            llhd_module.get_unit_name(always_ff_unit_id),
            "Template Unit name should match."
        );
        assert_eq!(
            initial_parent_unit_name,
            llhd_module.get_unit_name(initial_unit_id),
            "Template Unit name should match."
        );
    }
}

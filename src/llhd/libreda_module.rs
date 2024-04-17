use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;

use llhd::ir::prelude::*;
use llhd::ir::{InstData, LinkedUnit, ValueData};
use llhd::table::TableKey;
use rayon::prelude::*;

use super::common::{
    build_unit_name, filter_instantiations, filter_nullary, get_inst_name, get_unit_name,
    get_value_name,
};
use super::enode::LLHDENode;
use super::{LLHDArg, LLHDInst, LLHDNet};

type NameUnitMap = HashMap<String, UnitId>;
type UnitNameMap = HashMap<UnitId, String>;
type NameInstMap = HashMap<(UnitId, String), Inst>;
type InstNameMap = HashMap<LLHDInst, String>;
type ArgNameMap = HashMap<LLHDNet, String>;
type NameArgMap = HashMap<(UnitId, String), LLHDNet>;

/// `NewType` Wrapper for an LLHD Module
pub struct LModule {
    module: Module,
    name_unit_map: NameUnitMap,
    unit_name_map: UnitNameMap,
    name_arg_map: NameArgMap,
    arg_name_map: ArgNameMap,
    name_inst_map: NameInstMap,
    inst_name_map: InstNameMap,
}

impl LModule {
    pub(crate) fn new(module: Module) -> Self {
        let init = Self {
            module,
            ..Default::default()
        };
        let mut name_unit_map = NameUnitMap::default();
        let mut unit_name_map = UnitNameMap::default();
        let mut name_arg_map = NameArgMap::default();
        let mut arg_name_map = ArgNameMap::default();
        let mut name_inst_map = NameInstMap::default();
        let mut inst_name_map = InstNameMap::default();
        init.module.units().for_each(|scoped_unit| {
            let unit_id = scoped_unit.id();
            let unit_name = get_unit_name(&scoped_unit);
            name_unit_map.insert(unit_name.to_owned(), unit_id);
            unit_name_map.insert(unit_id, unit_name);
            init.all_insts(unit_id)
                .into_iter()
                .filter(|inst| matches!(scoped_unit[*inst], InstData::Call { .. }))
                .for_each(|inst| {
                    let net_name = get_inst_name(&init.module, &scoped_unit, inst);
                    name_inst_map.insert((unit_id, net_name.to_owned()), inst);
                    inst_name_map.insert((unit_id, inst), net_name);
                });
        });
        init.module.units().for_each(|scoped_unit| {
            let unit_id = scoped_unit.id();
            scoped_unit.args().for_each(|arg| {
                let arg_name = &get_value_name(&init.module, &scoped_unit, arg);
                name_arg_map.insert((unit_id, arg_name.to_owned()), (unit_id, arg));
                arg_name_map.insert((unit_id, arg), arg_name.to_owned());
            });
        });
        Self {
            module: init.module,
            name_unit_map,
            unit_name_map,
            name_arg_map,
            arg_name_map,
            name_inst_map,
            inst_name_map,
        }
    }

    pub(crate) fn unit_names(&self) -> usize {
        self.name_unit_map.len()
    }

    pub(crate) fn inst_names(&self) -> usize {
        self.name_inst_map.len()
    }

    pub(crate) fn module(&self) -> &Module {
        &self.module
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
        self.unit_name_map[&unit_id].to_owned()
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
                LinkedUnit::Def(ext_unit_id) => Some(ext_unit_id),
                LinkedUnit::Decl(_) => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_unit_arg(&self, inst: LLHDArg) -> LLHDNet {
        let owned_unit_id = inst.0;
        let inst_id = inst.1;
        let value_id = inst.2;
        if let InstData::Call { unit, args, .. } = &self.module().unit(owned_unit_id)[inst_id] {
            if let LinkedUnit::Def(ext_unit_id) = self
                .module()
                .lookup_ext_unit(*unit, owned_unit_id)
                .expect("ExtUnit not linked")
            {
                let instantiation_arg_idx = args
                    .into_iter()
                    .position(|arg| *arg == value_id)
                    .expect("Arg Values don't match Searched Value.");
                let arg_id = self
                    .module()
                    .unit(ext_unit_id)
                    .input_arg(instantiation_arg_idx);
                (ext_unit_id, arg_id)
            } else {
                panic!("Not a Unit Definition.")
            }
        } else {
            panic!("Not an instantiation instruction.")
        }
        // let value_refs = self.module().unit(value.0).uses(value.1);
        // let instantiation_refs = value_refs.iter().filter_map(|inst| {
        //     let inst_data = self.module.unit(value.0)[inst.to_owned()].to_owned();
        //     match inst_data {
        //         InstData::Call { unit, args, .. } => Some((unit, args)),
        //         _ => None
        //     }
        // });
    }

    pub(crate) fn get_value_name(&self, value: LLHDNet) -> String {
        let scope_unit_id = value.0;
        let value_id = value.1;
        let scope_unit = &self.module.unit(scope_unit_id);
        get_value_name(self.module(), scope_unit, value_id)
    }

    pub(crate) fn get_inst_name(&self, llhd_inst: LLHDInst) -> String {
        let unit_id = llhd_inst.0;
        let inst_id = llhd_inst.1;
        self.inst_name_map[&(unit_id, inst_id)].to_owned()
    }

    pub(crate) fn get_arg_name(&self, unit_arg_id: LLHDNet) -> String {
        self.arg_name_map[&unit_arg_id].to_owned()
    }

    pub(crate) fn get_arg(&self, unit_id: UnitId, name: &str) -> LLHDNet {
        self.name_arg_map[&(unit_id, name.to_owned())]
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
                id: unit
                    .get_inst_result(inst)
                    .unwrap_or_else(|| Value::new(usize::max_value())),
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
                        unit.get_inst_result(inst)
                            .unwrap_or_else(|| Value::new(usize::max_value())),
                    )
                }),
        )
    }

    pub(crate) fn iter_inst(&self, unit_id: UnitId) -> impl Iterator<Item = LLHDInst> + '_ {
        let unit = self.module.unit(unit_id);
        unit.all_insts()
            .filter(move |inst| filter_instantiations(&unit, *inst))
            .map(move |inst| (unit.id(), inst))
    }

    pub(crate) fn iter_unit_dependencies(
        &self,
        unit_id: UnitId,
    ) -> impl Iterator<Item = UnitId> + '_ {
        let scope_unit = self.module.unit(unit_id);
        let mut seen_units = HashSet::new();
        scope_unit
            .all_insts()
            .filter(move |inst| filter_instantiations(&scope_unit, *inst))
            .filter_map(move |inst| {
                let inst_data = &scope_unit[inst];
                if let InstData::Call { unit, .. } = inst_data {
                    Some(*unit)
                } else {
                    None
                }
            })
            .map(move |ext_unit_id| {
                let ext_unit_data = scope_unit[ext_unit_id].to_owned();
                let unit_name = ext_unit_data
                    .name
                    .get_name()
                    .expect("ExtUnit's UnitName does not resolve to String.");
                let dependency_unit_id = self.get_unit_id(&unit_name);
                dependency_unit_id
            })
            .filter(move |dependency_unit_id| seen_units.insert(*dependency_unit_id))
    }

    pub(crate) fn iter_dependent_units(
        &self,
        unit_id: UnitId,
    ) -> impl Iterator<Item = UnitId> + '_ {
        let mut dependent_units = HashSet::new();
        self.module
            .units()
            .flat_map(|unit| {
                let inner_unit_id = unit.id();
                unit.extern_units()
                    .map(move |(_, ext_unit_data)| (inner_unit_id, ext_unit_data))
            })
            .map(|(inner_unit_id, ext_unit_data)| {
                let ext_unit_name = ext_unit_data
                    .name
                    .get_name()
                    .expect("UnitName does not resolve to a String.");
                let ext_unit_id = self.get_unit_id(ext_unit_name);
                (inner_unit_id, ext_unit_id)
            })
            .filter(move |(_, ext_unit_id)| unit_id == *ext_unit_id)
            .map(|(inner_unit_id, _)| inner_unit_id)
            .filter(move |inner_unit_id| dependent_units.insert(*inner_unit_id))
    }

    pub(crate) fn iter_unit_instantiations(
        &self,
        unit_id: UnitId,
    ) -> impl Iterator<Item = LLHDInst> + '_ {
        let unit = self.module.unit(unit_id);
        unit.all_insts()
            .map(move |inst| (unit, unit_id, inst))
            .filter_map(
                move |(unit, inner_unit_id, inst)| match unit[inst].get_ext_unit() {
                    Some(_) => Some((inner_unit_id, inst)),
                    None => None,
                },
            )
    }

    pub(crate) fn iter_unit_references(
        &self,
        unit_id: UnitId,
    ) -> impl Iterator<Item = LLHDInst> + '_ {
        self.module
            .units()
            .flat_map(|unit| {
                let inner_unit_id = unit.id();
                self.iter_unit_instantiations(inner_unit_id)
            })
            .filter(move |(inner_unit_id, inst)| {
                let ext_unit_id = self
                    .get_unit_id_from_inst((*inner_unit_id, *inst))
                    .expect("Inst should have a corresponding Def.");
                unit_id == ext_unit_id
            })
    }

    pub(crate) fn add_unit(&mut self, unit_name: &str) -> UnitId {
        let kind = UnitKind::Entity;
        let name = build_unit_name(unit_name);
        let sig = Signature::default();
        let unit_data = UnitData::new(kind, name, sig);
        let global_unit_name = unit_data.name.to_owned().to_string();
        let unit_id = self.module.add_unit(unit_data);
        self.name_unit_map
            .insert(global_unit_name.to_owned(), unit_id);
        self.unit_name_map.insert(unit_id, global_unit_name);
        unit_id
    }

    pub(crate) fn remove_unit(&mut self, unit_id: UnitId) {
        let unit_name = &self.unit_name_map[&unit_id];
        self.module.remove_unit(unit_id);
        if let Some(_) = self.name_unit_map.remove(unit_name) {
            self.unit_name_map.remove(&unit_id);
        }
    }

    pub(crate) fn add_instantiation(
        &mut self,
        scoped_unit: UnitId,
        template_unit: UnitId,
        name: Option<&str>,
    ) -> Inst {
        let unit_name = self.module.unit(template_unit).name().clone();
        let sig = self.module.unit(template_unit).sig().clone();
        let mut unit = self.module.unit_mut(scoped_unit);
        let ext_unit_id = unit.add_extern(unit_name, sig);
        let inputs: Vec<Value> = vec![];
        let outputs: Vec<Value> = vec![];
        let inst_id = unit.ins().inst(ext_unit_id, inputs, outputs);
        self.module.link();
        if let Some(inst_name) = name {
            self.name_inst_map
                .insert((scoped_unit, inst_name.to_owned()), inst_id);
            self.inst_name_map
                .insert((scoped_unit, inst_id), inst_name.to_owned());
        }
        inst_id
    }

    pub(crate) fn remove_instantiation(&mut self, inst: LLHDInst) {
        let unit_id = inst.0;
        let inst_id = inst.1;
        let inst_name = &self.inst_name_map[&(unit_id, inst_id)];
        if let Some(_) = self.name_inst_map.remove(&(unit_id, inst_name.to_owned())) {
            self.inst_name_map.remove(&(unit_id, inst_id));
            let mut unit = self.module.unit_mut(unit_id);
            unit.delete_inst(inst_id);
        }
    }

    pub(crate) fn rename_unit(&mut self, unit_id: UnitId, name: &str) {
        let old_unit_name = &self.unit_name_map[&unit_id];
        let new_unit_name = name.to_owned();
        let mut unit = self.module.unit_mut(unit_id);
        match unit.data().name {
            UnitName::Local(_) => {
                unit.data().name = UnitName::local(new_unit_name.to_owned());
            }
            UnitName::Global(_) => {
                unit.data().name = UnitName::global(new_unit_name.to_owned());
            }
            _ => (),
        }
        self.name_unit_map.remove(old_unit_name);
        self.name_unit_map.insert(new_unit_name.to_owned(), unit_id);
        self.unit_name_map.remove(&unit_id);
        self.unit_name_map.insert(unit_id, new_unit_name);
    }

    pub(crate) fn rename_inst(&mut self, inst: LLHDInst, name: &str) {
        let unit_id = inst.0;
        let inst_id = inst.1;
        let old_inst_name = &self.inst_name_map[&(unit_id, inst_id)];
        let new_inst_name = name.to_owned();
        if let Some(_) = self
            .name_inst_map
            .remove(&(unit_id, old_inst_name.to_owned()))
        {
            self.inst_name_map.remove(&(unit_id, inst_id));
            self.name_inst_map
                .insert((unit_id, new_inst_name.to_owned()), inst_id);
            self.inst_name_map.insert((unit_id, inst_id), new_inst_name);
        }
    }

    pub fn declare(&mut self, name: UnitName, sig: Signature) -> DeclId {
        self.module.declare(name, sig)
    }

    pub fn add_decl(&mut self, data: DeclData) -> DeclId {
        self.module.add_decl(data)
    }

    pub fn remove_decl(&mut self, decl: DeclId) {
        self.module.remove_decl(decl);
    }

    pub fn units_mut<'a>(&'a mut self) -> impl Iterator<Item = UnitBuilder<'a>> + 'a {
        self.module.units_mut()
    }

    pub fn par_units_mut<'a>(&'a mut self) -> impl ParallelIterator<Item = UnitBuilder<'a>> + 'a {
        self.module.par_units_mut()
    }

    pub fn unit_mut(&mut self, unit: UnitId) -> UnitBuilder {
        self.module.unit_mut(unit)
    }

    pub fn link(&mut self) {
        self.module.link();
    }

    pub fn set_location_hint(&mut self, mod_unit: UnitId, loc: usize) {
        self.module.set_location_hint(mod_unit, loc);
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
            unit_name_map: HashMap::new(),
            name_arg_map: HashMap::new(),
            arg_name_map: HashMap::new(),
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
        let unit_id = llhd_module.module().units().next().unwrap().id();
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
        let unit_id = llhd_module.module().units().next().unwrap().id();
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
        let unit_id = llhd_module.module().units().next().unwrap().id();
        let unit_name = llhd_module.get_unit_name(unit_id);
        assert_eq!("ent2", unit_name, "Unit name does not match stored name.");
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
        let unit_name = "ent2".to_owned();
        let unit_id = llhd_module.get_unit_id(&unit_name);
        assert_eq!(
            UnitId::new(0),
            unit_id,
            "Unit name does not match stored name."
        );
    }

    #[test]
    #[should_panic]
    fn llhd_module_get_inst_name() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.module().units().next().unwrap().id();
        let and_inst_id = llhd_module
            .module()
            .unit(unit_id)
            .all_insts()
            .nth(1)
            .unwrap();
        let inst_id = (unit_id, and_inst_id);
        let inst_name = llhd_module.get_inst_name(inst_id);
        assert_eq!("ent2.and.v3", inst_name, "Inst name does not match");
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
            .module()
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
        assert_eq!("top.top.and.i7", and_inst_name, "Inst name does not match");
    }

    #[test]
    #[should_panic]
    fn llhd_module_get_inst_id() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let unit_id = llhd_module.module().units().next().unwrap().id();
        let inst_id = llhd_module.get_inst(unit_id, "@ent2.and.v3");
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
            .module()
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
        let always_ff_parent_unit_name = "top.always_ff.227.0".to_owned();
        let initial_parent_unit_name = "top.initial.228.0".to_owned();
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

    #[test]
    fn llhd_module_iter_unit_insts() {
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
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                %top_input13 = const i1 0
                %in13 = sig i1 %top_input13
                %top_input23 = const i1 1
                %in23 = sig i1 %top_input23
                %top_out13 = const i1 0
                %out13 = sig i1 %top_out13

                %top_input14 = const i1 0
                %in14 = sig i1 %top_input14
                %top_input24 = const i1 1
                %in24 = sig i1 %top_input24
                %top_out14 = const i1 0
                %out14 = sig i1 %top_out14

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
                inst %top.and (i1$ %in13, i1$ %in23) -> (i1$ %out13)
                inst %top.and (i1$ %in14, i1$ %in24) -> (i1$ %out14)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let module_binding = llhd_module.module();
        let units: Vec<_> = module_binding.units().collect();
        let top_unit = units[1];
        assert_eq!(
            "top",
            llhd_module.get_unit_name(top_unit.id()),
            "Unit should be 'top' unit."
        );
        let inst_count = llhd_module.iter_inst(top_unit.id()).count();
        assert_eq!(
            4, inst_count,
            "There should be 4 Instantiation instructions present in Unit."
        );
    }

    #[test]
    fn llhd_module_unit_get_dependencies() {
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

            proc %top.or (i1$ %in1, i1$ %in2) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %or1 = or i1 %in1_prb, %in2_prb
                drv i1$ %out1, %or1, %epsilon
                wait %init for %epsilon
            }

            proc %top.or_unused (i1$ %in1, i1$ %in2) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %or1 = or i1 %in1_prb, %in2_prb
                drv i1$ %out1, %or1, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                %top_input13 = const i1 0
                %in13 = sig i1 %top_input13
                %top_input23 = const i1 1
                %in23 = sig i1 %top_input23
                %top_out13 = const i1 0
                %out13 = sig i1 %top_out13

                %top_input14 = const i1 0
                %in14 = sig i1 %top_input14
                %top_input24 = const i1 1
                %in24 = sig i1 %top_input24
                %top_out14 = const i1 0
                %out14 = sig i1 %top_out14

                %top_input15 = const i1 0
                %in15 = sig i1 %top_input15
                %top_input25 = const i1 1
                %in25 = sig i1 %top_input25
                %top_out15 = const i1 0
                %out15 = sig i1 %top_out15

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
                inst %top.and (i1$ %in13, i1$ %in23) -> (i1$ %out13)
                inst %top.and (i1$ %in14, i1$ %in24) -> (i1$ %out14)
                inst %top.or (i1$ %in15, i1$ %in25) -> (i1$ %out15)
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let module_binding = llhd_module.module();
        let units: Vec<_> = module_binding.units().collect();
        let top_unit = units[3];
        assert_eq!(
            "top",
            llhd_module.get_unit_name(top_unit.id()),
            "Unit should be 'top' unit."
        );
        let unit_dependencies: Vec<_> = llhd_module
            .iter_unit_dependencies(top_unit.id())
            .map(|unit_id| llhd_module.module.unit(unit_id))
            .collect();
        assert_eq!(
            2,
            unit_dependencies.len(),
            "There are 2 Unit dependencies for @top: %top.and and %top.or"
        );
        let unit_dependency_names: HashSet<_> = unit_dependencies
            .into_iter()
            .map(|unit| llhd_module.get_unit_name(unit.id()))
            .collect();
        assert!(
            unit_dependency_names.contains("top.and"),
            "top.and is a dependent_unit."
        );
        assert!(
            unit_dependency_names.contains("top.or"),
            "top.or is a dependent_unit."
        );
        assert!(
            !unit_dependency_names.contains("top.or_unused"),
            "top.or_unused is a dependent_unit."
        );
    }

    #[test]
    fn llhd_module_unit_get_dependent_units() {
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

            proc %top.or (i1$ %in1, i1$ %in2) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %or1 = or i1 %in1_prb, %in2_prb
                drv i1$ %out1, %or1, %epsilon
                wait %init for %epsilon
            }

            entity @second () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }

            entity @third () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
            }

            entity @fourth () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }

            entity @top () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let module_binding = llhd_module.module();
        let units: Vec<_> = module_binding.units().collect();
        let and_unit = units[0];
        assert_eq!(
            "top.and",
            llhd_module.get_unit_name(and_unit.id()),
            "Unit should be 'and' unit."
        );
        let unit_dependencies: Vec<_> = llhd_module
            .iter_dependent_units(and_unit.id())
            .map(|unit_id| llhd_module.module.unit(unit_id))
            .collect();
        assert_eq!(
            4,
            unit_dependencies.len(),
            "There are 4 Units dependent on %top.and: @top, @second, @third, @fourth."
        );
        let dependent_unit_names: HashSet<_> = unit_dependencies
            .into_iter()
            .map(|unit| llhd_module.get_unit_name(unit.id()))
            .collect();
        assert!(
            dependent_unit_names.contains("top"),
            "@top is a dependent unit."
        );
        assert!(
            dependent_unit_names.contains("second"),
            "@second is a dependent unit."
        );
        assert!(
            dependent_unit_names.contains("third"),
            "@third is a dependent unit."
        );
        assert!(
            dependent_unit_names.contains("fourth"),
            "@fourth is a dependent unit."
        );
        assert!(
            !dependent_unit_names.contains("top.or"),
            "%top.or not is a dependent unit."
        );

        let or_unit = units[1];
        assert_eq!(
            "top.or",
            llhd_module.get_unit_name(or_unit.id()),
            "Unit should be 'or' unit."
        );
        let or_unit_dependencies: Vec<_> = llhd_module
            .iter_dependent_units(or_unit.id())
            .map(|unit_id| llhd_module.module.unit(unit_id))
            .collect();
        assert_eq!(
            0,
            or_unit_dependencies.len(),
            "There are 0 Units dependent on %top.or."
        );
    }

    #[test]
    fn llhd_module_unit_get_references() {
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

            entity @second () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }

            entity @third () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
            }

            entity @fourth () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }

            entity @top () -> () {
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11

                %top_input12 = const i1 0
                %in12 = sig i1 %top_input12
                %top_input22 = const i1 1
                %in22 = sig i1 %top_input22
                %top_out12 = const i1 0
                %out12 = sig i1 %top_out12

                inst %top.and (i1$ %in11, i1$ %in21) -> (i1$ %out11)
                inst %top.and (i1$ %in12, i1$ %in22) -> (i1$ %out12)
            }
        "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);

        let module_binding = llhd_module.module();
        let units: Vec<_> = module_binding.units().collect();
        let and_unit = units[0];
        assert_eq!(
            "top.and",
            llhd_module.get_unit_name(and_unit.id()),
            "Unit should be 'and' unit."
        );
        let unit_references: Vec<_> = llhd_module
            .iter_unit_references(and_unit.id())
            .map(|(unit_id, inst)| (llhd_module.module.unit(unit_id), inst))
            .collect();
        assert_eq!(
            7,
            unit_references.len(),
            "There are 7 references to %top.and: @top.%top.and, @second.%top.and, \
             @third.%top.and, @fourth.%top.and."
        );
        let unit_reference_names: HashSet<_> = unit_references
            .into_iter()
            .map(|(unit, inst)| llhd_module.get_inst_name((unit.id(), inst)))
            .collect();
        assert!(
            unit_reference_names.contains("top.top.and.i13"),
            "@top has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("top.top.and.i14"),
            "@top has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("second.top.and.i13"),
            "@second has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("second.top.and.i14"),
            "@second has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("fourth.top.and.i13"),
            "@fourth has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("fourth.top.and.i14"),
            "@fourth has 2 references to %top.and."
        );
        assert!(
            unit_reference_names.contains("third.top.and.i7"),
            "@third has a reference to %top.and."
        );
    }

    #[test]
    fn llhd_module_add_unit() {
        let input = indoc::indoc! {"
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_module = LModule::from(module);
        let top_unit_id = llhd_module.add_unit("top");
        assert!(
            !llhd_module.get_unit_name(UnitId::new(0)).is_empty(),
            "Unit should be present in UnitNameMap."
        );
        assert!(
            !llhd_module.get_unit_name(UnitId::new(1)).is_empty(),
            "Unit should be present in UnitNameMap."
        );
        let top_unit = llhd_module.module().unit(top_unit_id);
        assert_eq!(
            "@top",
            top_unit.name().to_string(),
            "Unit name should match."
        );
    }

    #[test]
    fn llhd_module_remove_unit() {
        let input = indoc::indoc! {"
            entity @top (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
            entity @ent2 (i1 %in1, i1 %in2, i1 %in3) -> () {
                %and1 = and i1 %in1, %in2
                %or1 = or i1 %and1, %in3
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_module = LModule::from(module);
        let top_unit_id = UnitId::new(0);
        llhd_module.remove_unit(top_unit_id);
        assert_eq!(
            1,
            llhd_module.unit_names(),
            "There should be 1 Unit present in Module."
        );
        assert_eq!(
            1,
            llhd_module.module().units().count(),
            "There should be 1 Unit present in Module."
        );
        let ent2_id = UnitId::new(1);
        let ent2_unit = llhd_module.module().unit(ent2_id);
        assert_eq!(
            "@ent2",
            ent2_unit.name().to_string(),
            "@ent2 should be the remaining unit."
        );
    }

    #[test]
    fn llhd_module_add_instantiation() {
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
                %top_input11 = const i1 0
                %in11 = sig i1 %top_input11
                %top_input21 = const i1 1
                %in21 = sig i1 %top_input21
                %top_out11 = const i1 0
                %out11 = sig i1 %top_out11
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_module = LModule::from(module);
        let top_unit_id = UnitId::new(1);
        let and_unit_id = UnitId::new(0);
        assert_eq!(
            0,
            llhd_module.iter_unit_instantiations(top_unit_id).count(),
            "There should be no %top.and instantiations yet."
        );
        let _instantiation_inst = llhd_module.add_instantiation(top_unit_id, and_unit_id, None);
        let and_unit = llhd_module.module().units().next().unwrap();
        assert_eq!(
            "top.and",
            get_unit_name(&and_unit),
            "Unit should be 'and' unit."
        );
        assert_eq!(
            1,
            llhd_module.iter_unit_instantiations(top_unit_id).count(),
            "There is 1 reference to %top.and: @top.%top.and."
        );
        let unit_references: Vec<_> = llhd_module
            .iter_unit_references(and_unit.id())
            .map(|(unit_id, inst)| (llhd_module.module.unit(unit_id), inst))
            .collect();
        assert_eq!(
            1,
            unit_references.len(),
            "There is 1 reference to %top.and: @top.%top.and."
        );
    }

    #[test]
    fn llhd_module_remove_instantiation() {
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
        let mut llhd_module = LModule::from(module);
        let top_unit_id = UnitId::new(1);
        assert_eq!(
            1,
            llhd_module.inst_names(),
            "There should be 1 Instantiation Inst's present in Unit."
        );
        let inst_count_before = llhd_module
            .module()
            .units()
            .nth(1)
            .unwrap()
            .all_insts()
            .count();
        assert_eq!(
            8, inst_count_before,
            "There should be 8 total Inst's present in Unit."
        );
        llhd_module.remove_instantiation((top_unit_id, Inst::new(7)));
        let inst_count_after = llhd_module
            .module()
            .units()
            .nth(1)
            .unwrap()
            .all_insts()
            .count();
        assert_eq!(
            7, inst_count_after,
            "There should be 7 total Inst's present in Unit."
        );
        assert_eq!(
            0,
            llhd_module.inst_names(),
            "There should be 0 Instantiation Inst's present in Unit."
        );
    }

    #[test]
    fn llhd_module_rename_unit() {
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
        let mut llhd_module = LModule::from(module);
        let and_unit_id = UnitId::new(0);
        let and_unit_name = llhd_module.get_unit_name(and_unit_id);
        assert_eq!("top.and", and_unit_name, "Unit should be named '%top.and'.");
        llhd_module.rename_unit(and_unit_id, "ent.and");
        let and_unit_name_updated = llhd_module.get_unit_name(and_unit_id);
        assert_eq!(
            "ent.and", and_unit_name_updated,
            "Unit should be named '%ent.and'."
        );
    }

    #[test]
    fn llhd_module_rename_instantiation() {
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
        let mut llhd_module = LModule::from(module);
        let top_unit_id = UnitId::new(1);
        let and_unit_id = UnitId::new(0);
        let and_inst_id = Inst::new(7);
        let and_unit_name = llhd_module.get_unit_name(and_unit_id);
        assert_eq!("top.and", and_unit_name, "Unit should be named 'top.and'.");
        let and_inst_name = llhd_module.get_inst_name((top_unit_id, and_inst_id));
        assert_eq!(
            "top.top.and.i7", and_inst_name,
            "Inst should be named '%top.and'."
        );
        llhd_module.rename_inst((top_unit_id, and_inst_id), "ent.and");
        let and_inst_name_updated = llhd_module.get_inst_name((top_unit_id, and_inst_id));
        assert_eq!(
            "ent.and", and_inst_name_updated,
            "Inst should be named '%ent.and'."
        );
    }

    #[test]
    fn llhd_module_get_pin_val_from_inst_val() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let top_unit_id = UnitId::new(1);
        let and_inst_arg1_id = (top_unit_id, Inst::new(9), Value::new(1));
        let and_unit_arg1_id = llhd_module.get_unit_arg(and_inst_arg1_id);
        let and_unit_arg1_name = llhd_module.get_value_name(and_unit_arg1_id);
        assert_eq!(
            "top.and.v0", and_unit_arg1_name,
            "Unit Arg should be named 'top.and.v0'."
        );
        let and_inst_arg2_id = (top_unit_id, Inst::new(9), Value::new(3));
        let and_unit_arg2_id = llhd_module.get_unit_arg(and_inst_arg2_id);
        let and_unit_arg2_name = llhd_module.get_value_name(and_unit_arg2_id);
        assert_eq!(
            "top.and.v1", and_unit_arg2_name,
            "Unit Arg should be named 'top.and.v1'."
        );
        let and_inst_arg3_id = (top_unit_id, Inst::new(9), Value::new(5));
        let and_unit_arg3_id = llhd_module.get_unit_arg(and_inst_arg3_id);
        let and_unit_arg3_name = llhd_module.get_value_name(and_unit_arg3_id);
        assert_eq!(
            "top.and.v2", and_unit_arg3_name,
            "Unit Arg should be named 'top.and.v2'."
        );
    }

    #[test]
    fn llhd_module_get_pin_name() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let llhd_module = LModule::from(module);
        let and_unit_id = UnitId::new(0);
        let and_unit_arg1_id = (and_unit_id, Value::new(0));
        let and_unit_arg1_name = llhd_module.get_arg_name(and_unit_arg1_id);
        assert_eq!(
            "top.and.v0", and_unit_arg1_name,
            "Unit Arg should be named 'top.and.v0'."
        );
        let and_unit_arg1_value = llhd_module.get_arg(and_unit_id, "top.and.v0");
        assert_eq!(
            (and_unit_id, Value::new(0)),
            and_unit_arg1_value,
            "Unit Arg should match for 'top.and.v0'."
        );
        let and_unit_arg2_value = llhd_module.get_arg(and_unit_id, "top.and.v1");
        assert_eq!(
            (and_unit_id, Value::new(1)),
            and_unit_arg2_value,
            "Unit Arg should match for 'top.and.v1'."
        );
        let and_unit_arg3_value = llhd_module.get_arg(and_unit_id, "top.and.v2");
        assert_eq!(
            (and_unit_id, Value::new(2)),
            and_unit_arg3_value,
            "Unit Arg should match for 'top.and.v2'."
        );
    }
}

// use libreda_db::prelude::*;
// use llhd::ir::prelude::*;
//
// use crate::index::Syndex;
// use crate::llhd::LLHDInst;
//
// impl HierarchyBase for Syndex {
//     type NameType = String;
//     type CellId = UnitId;
//     type CellInstId = LLHDInst;
//
//     fn cell_by_name(&self, name: &str) -> Option<Self::CellId> {
//         // Chip::circuit_by_name(self, name)
//         todo!()
//     }
//
//     fn cell_instance_by_name(
//         &self,
//         parent_circuit: &Self::CellId,
//         name: &str,
//     ) -> Option<Self::CellInstId> {
//         // self.circuit(parent_circuit).instances_by_name.get(name).copied()
//         todo!()
//     }
//
//     fn cell_name(&self, circuit: &Self::CellId) -> Self::NameType {
//         // self.circuit(circuit).name.clone()
//         todo!()
//     }
//
//     fn cell_instance_name(&self, circuit_inst: &Self::CellInstId) -> Option<Self::NameType> {
//         // self.circuit_inst(circuit_inst).name.clone()
//         todo!()
//     }
//
//     fn parent_cell(&self, circuit_instance: &Self::CellInstId) -> Self::CellId {
//         // self.circuit_inst(circuit_instance).parent_circuit_id
//         todo!()
//     }
//
//     fn template_cell(&self, circuit_instance: &Self::CellInstId) -> Self::CellId {
//         // self.circuit_inst(circuit_instance).template_circuit_id
//         todo!()
//     }
//
//     fn for_each_cell<F>(&self, f: F)
//     where
//         F: FnMut(Self::CellId) -> (),
//     {
//         // self.circuits.keys().copied().for_each(f)
//         todo!()
//     }
//
//     fn each_cell(&self) -> Box<dyn Iterator<Item = Self::CellId> + '_> {
//         // Box::new(self.circuits.keys().copied())
//         todo!()
//     }
//
//     fn for_each_cell_instance<F>(&self, circuit: &Self::CellId, f: F)
//     where
//         F: FnMut(Self::CellInstId) -> (),
//     {
//         // self.circuit(circuit).instances.iter()
//         //     .copied().for_each(f)
//         todo!()
//     }
//
//     fn each_cell_instance(
//         &self,
//         circuit: &Self::CellId,
//     ) -> Box<dyn Iterator<Item = Self::CellInstId> + '_> {
//         // Box::new(self.circuit(circuit).instances.iter().copied())
//         todo!()
//     }
//
//     fn for_each_cell_dependency<F>(&self, circuit: &Self::CellId, f: F)
//     where
//         F: FnMut(Self::CellId) -> (),
//     {
//         // self.circuit(circuit).dependencies.keys().copied().for_each(f);
//         todo!()
//     }
//
//     fn each_cell_dependency(
//         &self,
//         circuit: &Self::CellId,
//     ) -> Box<dyn Iterator<Item = Self::CellId> + '_> {
//         // Box::new(self.circuit(circuit).dependencies.keys().copied())
//         todo!()
//     }
//
//     fn num_cell_dependencies(&self, cell: &Self::CellId) -> usize {
//         // self.circuit(cell).dependencies.len()
//         todo!()
//     }
//
//     fn for_each_dependent_cell<F>(&self, circuit: &Self::CellId, f: F)
//     where
//         F: FnMut(Self::CellId) -> (),
//     {
//         // self.circuit(circuit).dependent_circuits.keys().copied().for_each(f);
//         todo!()
//     }
//
//     fn each_dependent_cell(
//         &self,
//         circuit: &Self::CellId,
//     ) -> Box<dyn Iterator<Item = Self::CellId> + '_> {
//         // Box::new(self.circuit(circuit).dependent_circuits.keys().copied())
//         todo!()
//     }
//
//     fn num_dependent_cells(&self, cell: &Self::CellId) -> usize {
//         // self.circuit(cell).dependent_circuits.len()
//         todo!()
//     }
//
//     fn for_each_cell_reference<F>(&self, circuit: &Self::CellId, f: F)
//     where
//         F: FnMut(Self::CellInstId) -> (),
//     {
//         // self.circuit(circuit).references.iter().copied().for_each(f)
//         todo!()
//     }
//
//     fn each_cell_reference(
//         &self,
//         circuit: &Self::CellId,
//     ) -> Box<dyn Iterator<Item = Self::CellInstId> + '_> {
//         // Box::new(self.circuit(circuit).references.iter().copied())
//         todo!()
//     }
//
//     fn num_cell_references(&self, cell: &Self::CellId) -> usize {
//         // self.circuit(cell).references.len()
//         todo!()
//     }
//
//     fn num_child_instances(&self, cell: &Self::CellId) -> usize {
//         // self.circuit(cell).instances.len()
//         todo!()
//     }
//
//     fn num_cells(&self) -> usize {
//         // self.circuits.len()
//         todo!()
//     }
//
//     fn get_chip_property(&self, key: &Self::NameType) -> Option<PropertyValue> {
//         // self.properties.get(key).cloned()
//         todo!()
//     }
//
//     fn get_cell_property(
//         &self,
//         cell: &Self::CellId,
//         key: &Self::NameType,
//     ) -> Option<PropertyValue> {
//         // self.circuit(cell).properties.get(key).cloned()
//         todo!()
//     }
//
//     fn get_cell_instance_property(
//         &self,
//         inst: &Self::CellInstId,
//         key: &Self::NameType,
//     ) -> Option<PropertyValue> {
//         // self.circuit_inst(inst).properties.get(key).cloned()
//         todo!()
//     }
// }

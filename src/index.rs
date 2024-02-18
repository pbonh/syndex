use crate::llhd::{LLHDNet, LModule};
use rayon::prelude::*;
use llhd::ir::{UnitId, UnitData, UnitName, Signature, DeclId, DeclData, UnitBuilder};
use crate::world::LWorld;
use flecs::EntityId;
use std::collections::HashMap;

type LinkedMap = HashMap<LLHDNet, EntityId>;

/// Synthesis Database
#[derive(Debug,Default)]
pub struct Syndex {
    module: LModule,
    world: LWorld,
    map: LinkedMap,
}

impl Syndex {
    #[must_use]
    pub const fn module(&self) -> &LModule {
        &self.module
    }

    pub fn add_unit(&mut self, data: UnitData) -> UnitId {
        self.module.add_unit(data)
    }

    pub fn remove_unit(&mut self, unit: UnitId) {
        self.module.remove_unit(unit);
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

    #[must_use]
    pub const fn world(&self) -> &LWorld {
        &self.world
    }
}

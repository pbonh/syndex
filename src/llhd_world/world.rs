use crate::{llhd::module::LLHDModule, world::LWorld};

#[derive(Debug,Default)]
pub struct LLHDWorld {
    module: LLHDModule,
    world: LWorld,
}

impl LLHDWorld {
    fn new() -> Self {
        Self {
            module: LLHDModule::default(),
            world: LWorld::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_llhd_world() {
        let _ = LLHDWorld::default();
    }
}

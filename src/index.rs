use crate::llhd::LModule;
use flecs::World;

/// Database
#[derive(Debug)]
struct Syndex {
    llhd: LModule,
    ecs: World,
}


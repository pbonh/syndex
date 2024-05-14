use std::ops::{Deref, DerefMut};

use flecs::World;

/// New-Type Wrapper for an LLHD Module
#[derive(Debug,Default)]
pub struct LWorld(World);

impl Deref for LWorld {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<World> AsRef<World> for LWorld
where
    World: ?Sized,
    <LWorld as Deref>::Target: AsRef<World>,
{
    fn as_ref(&self) -> &World {
        self.deref().as_ref()
    }
}

impl<World> AsMut<World> for LWorld
where
    <LWorld as Deref>::Target: AsMut<World>,
{
    fn as_mut(&mut self) -> &mut World {
        self.deref_mut().as_mut()
    }
}

impl From<World> for LWorld {
    fn from(module: World) -> Self {
        Self(module)
    }
}

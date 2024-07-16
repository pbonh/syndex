use std::ops::{Deref, DerefMut};

use layout21::gds21::GdsLibrary;

#[derive(Debug, Clone, Default)]
pub struct LGdsLibrary(GdsLibrary);

impl Deref for LGdsLibrary {
    type Target = GdsLibrary;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LGdsLibrary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<GdsLibrary> AsRef<GdsLibrary> for LGdsLibrary
where
    GdsLibrary: ?Sized,
    <Self as Deref>::Target: AsRef<GdsLibrary>,
{
    fn as_ref(&self) -> &GdsLibrary {
        self.deref().as_ref()
    }
}

impl<GdsLibrary> AsMut<GdsLibrary> for LGdsLibrary
where
    <Self as Deref>::Target: AsMut<GdsLibrary>,
{
    fn as_mut(&mut self) -> &mut GdsLibrary {
        self.deref_mut().as_mut()
    }
}

impl From<GdsLibrary> for LGdsLibrary {
    fn from(library: GdsLibrary) -> Self {
        Self(library)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn gds_library_default() {
        let _ = LGdsLibrary::default();
    }

    #[test]
    fn gds_sky130pdk_cell_example() {
        let mut gds_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        gds_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a2111o/\
             sky130_fd_sc_ls__a2111o_1.gds",
        );
        assert!(
            GdsLibrary::open(gds_path).is_ok(),
            "Skywater 130nm PDK Cell 'a2111o' should successfully load."
        );
    }
}

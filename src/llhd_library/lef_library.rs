use std::ops::{Deref, DerefMut};

use layout21::lef21::LefLibrary;

#[derive(Debug, Clone, Default)]
pub struct LLefLibrary(LefLibrary);

impl Deref for LLefLibrary {
    type Target = LefLibrary;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LLefLibrary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<LefLibrary> AsRef<LefLibrary> for LLefLibrary
where
    LefLibrary: ?Sized,
    <Self as Deref>::Target: AsRef<LefLibrary>,
{
    fn as_ref(&self) -> &LefLibrary {
        self.deref().as_ref()
    }
}

impl<LefLibrary> AsMut<LefLibrary> for LLefLibrary
where
    <Self as Deref>::Target: AsMut<LefLibrary>,
{
    fn as_mut(&mut self) -> &mut LefLibrary {
        self.deref_mut().as_mut()
    }
}

impl From<LefLibrary> for LLefLibrary {
    fn from(library: LefLibrary) -> Self {
        Self(library)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn lef_library_default() {
        let _ = LLefLibrary::default();
    }

    #[test]
    #[should_panic]
    fn lef_sky130pdk_example() {
        let mut tech_lef_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        tech_lef_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/tech/sky130_fd_sc_ls.tlef",
        );
        assert!(
            LefLibrary::open(tech_lef_path).is_ok(),
            "Skywater 130nm PDK Tech LEF should successfully load."
        );
    }

    #[test]
    #[should_panic]
    fn lef_spec_example() {
        let mut tech_lef_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        tech_lef_path.push("resources/lef_examples/lef_def_lang_reference_example.lef");
        assert!(
            LefLibrary::open(tech_lef_path).is_ok(),
            "Spec Example Tech LEF should successfully load."
        );
    }

    #[test]
    fn lef_sky130pdk_a211o_example() {
        let mut tech_lef_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        tech_lef_path.push(
            "resources/libraries_no_liberty/sky130_fd_sc_ls/latest/cells/a211o/\
             sky130_fd_sc_ls__a211o_2.magic.lef",
        );
        assert!(
            LefLibrary::open(tech_lef_path).is_ok(),
            "Skywater 130nm PDK Libary(a211o) LEF should successfully load."
        );
    }
}

//! Project-level documentation.

#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]
// clippy WARN level lints
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::map_err_ignore,
    clippy::panic,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unreachable
)]
#![allow(clippy::multiple_crate_versions)]
// clippy WARN level lints, that can be upgraded to DENY if preferred
#![warn(
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::modulo_arithmetic,
    clippy::as_conversions,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::pattern_type_mismatch,
    clippy::string_slice,
    clippy::try_err
)]
// clippy DENY level lints, they always have a quick fix that should be preferred
#![deny(
    clippy::wildcard_imports,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::separated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

mod config;

/// Build A Synthesis Flow via a State Machine
///
/// 1) Start with a Digital Design(LLHD Module)
/// 2) Constrain the Synthesis Flow to a Technology
/// 3) Apply Synthesis Rules to Design
///
/// ```rust
/// # use syndex::{Flow, DesignState, TechnologyState, SynthesisState};
/// # let input = indoc::indoc! {"
/// #         entity @test_entity (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
/// #             %null = const time 0s 1e
/// #             %and1 = and i1 %in1, %in2
/// #             %and2 = and i1 %in3, %in4
/// #             %or1 = or i1 %and1, %and2
/// #             drv i1$ %out1, %or1, %null
/// #         }
/// #     "};
///
/// # let module = llhd::assembly::parse_module(input).unwrap();
/// let _technology_flow = Flow::load(module.into());
/// ```
pub mod synthesis_state;
pub use synthesis_state::builder::{DesignState, Flow, SynthesisState, TechnologyState};

pub mod synthesis;

/// Library Technology Representation
///
/// Build a representation of the underlying technology
///
/// Inputs
/// 1) LLHDModule -> Cells/Memories are LLHD Units with digital signal declarations
/// 2) LCircuit -> Analog Circuit Representation of all Library Units
/// 3) LLefLibrary -> Abstract Representation of Library Units
/// 4) LGdsLibrary -> Physical Representation of Library Units
///
/// Build A Technology Flow via a State Machine
///
/// 1) Load an Abstract View(LEF)
/// 2) Build Analog Circuits
/// 3) Load Layout Information
/// 4) Build LLHD Unit Representation
///
pub mod llhd_library;

/// Datastore for Design
pub mod llhd_world;

/// Index/Database for Design, driven by Datalog Relation Tables
///
/// LLHD Module
/// |
/// |--> Unit(UnitId)
///      |
///      |--> DFG Node
///      |    |
///      |    |--> DesignDGate(UnitId, Inst, InstData)
///      |    |
///      |    |--> DesignDNet(UnitId, Inst, Value)
///      |
///      |--> CFG Node
///           |
///           |--> (UnitId, Inst, InstData)
///           |
///           |--> (UnitId, Inst, Block)
///
pub mod index;

/// Types & Utilities for Managing LLHD Modules
pub(crate) mod llhd;

/// Types & Utilities for Managing FLECS Worlds
pub(crate) mod world;

/// Analog Circuit Data Structure
pub(crate) mod circuit;

/// `LibrEDA` Trait Implementations
mod libreda;

/// DB Rewrites via the Ascent Datalog Engine
mod engine;

pub use config::CONFIG;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

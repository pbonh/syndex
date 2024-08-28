#[derive(Debug)]
pub(crate) struct LLHDUtils;

#[derive(Debug)]
pub(crate) struct LLHDEGraph;

/// LLHD Inst `ENode` Type
pub mod enode;

/// Helper Functions for LLHD Types
pub mod common;

/// World Component Data for LLHD
pub mod components;

/// World Component Data for LLHD Nets/Instructions
pub mod inst;

/// LLHD Module Type Wrapper
pub mod module;

/// LLHD Unit Data
pub mod unit;

/// LLHD Module LibrEDA Trait Implementation
pub mod libreda_module;

use llhd::ir::{Inst, UnitId, Value};

/// `Net/Value` Identifier within LLHD `Unit`
pub type LLHDUnitArg = (UnitId, Value);

/// `Inst` Identifier within LLHD `Unit`
pub type LLHDInst = (UnitId, Inst);

/// `Value` Identifier within LLHD `Unit`
pub type LLHDValue = (UnitId, Inst, Value);

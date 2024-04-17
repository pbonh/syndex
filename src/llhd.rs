/// LLHD Inst `ENode` Type
pub mod enode;

/// Helper Functions for LLHD Types
pub mod common;

/// World Component Data for LLHD Unit
pub mod unit;

/// World Component Data for LLHD Nets/Instructions
pub mod inst;

/// LLHD Module Type Wrapper
pub mod module;

/// LLHD Module LibrEDA Trait Implementation
pub mod libreda_module;

use llhd::ir::{Inst, UnitId, Value};

/// `Net/Value` Identifier within LLHD `Unit`
pub type LLHDNet = (UnitId, Value);

/// `Inst` Identifier within LLHD `Unit`
pub type LLHDInst = (UnitId, Inst);

/// `Pin/Value` Identifier within LLHD `Unit`
pub type LLHDArg = (UnitId, Inst, Value);

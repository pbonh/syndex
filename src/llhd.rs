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

use llhd::ir::{Opcode, UnitId, Value};

/// `Net` Identifier within LLHD `Unit`
pub type LLHDNet = (UnitId, Value, Opcode);

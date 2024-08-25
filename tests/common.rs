use std::fs;
use std::path::PathBuf;

use llhd::ir::prelude::*;

#[cfg(test)]
#[allow(dead_code)]
fn load_llhd_module(filename: &str) -> Module {
    let mut llhd_module_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    llhd_module_file_path.push("resources/llhd");
    llhd_module_file_path.push(filename);
    let llhd_module_str: String = fs::read_to_string(llhd_module_file_path).unwrap();
    llhd::assembly::parse_module(llhd_module_str)
        .expect(&format!("Error loading module: {}", filename))
}

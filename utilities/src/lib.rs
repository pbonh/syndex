use std::fs;
use std::path::PathBuf;

use egglog::EGraph;
use llhd::ir::prelude::*;

pub fn load_egraph(filename: &str) -> (EGraph, Vec<String>) {
    let mut egglog_program_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    egglog_program_file_path.push("../resources/egglog");
    egglog_program_file_path.push(filename);
    let egglog_program_str: String = fs::read_to_string(egglog_program_file_path).unwrap();
    let mut egraph = EGraph::default();
    let msgs = egraph
        .parse_and_run_program(&egglog_program_str)
        .expect("Failure to run program on egraph.");
    (egraph, msgs)
}

pub fn load_llhd_module(filename: &str) -> Module {
    let mut llhd_module_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    llhd_module_file_path.push("../resources/llhd");
    llhd_module_file_path.push(filename);
    let llhd_module_str: String = fs::read_to_string(llhd_module_file_path).unwrap();
    llhd::assembly::parse_module(llhd_module_str)
        .expect(&format!("Error loading module: {}", filename))
}

pub fn trim_whitespace(s: &str) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

pub fn build_entity(name: UnitName) -> UnitData {
    let mut sig = Signature::new();
    let _clk = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
    let _rst = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
    let inp = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
    let _oup = sig.add_output(llhd::signal_ty(llhd::int_ty(32)));
    let mut ent = UnitData::new(UnitKind::Entity, name, sig);
    {
        let mut builder = UnitBuilder::new_anonymous(&mut ent);
        let v1 = builder.ins().const_int((1, 0));
        let v2 = builder.ins().const_int((1, 1));
        let v3 = builder.ins().add(v1, v2);
        let inp = builder.unit().arg_value(inp);
        let inp = builder.ins().prb(inp);
        builder.ins().add(v3, inp);
    }
    Unit::new_anonymous(&ent).verify();
    ent
}

use std::fs;
use std::path::PathBuf;

use egglog::{EGraph, Error};
use itertools::Itertools;
use llhd::ir::prelude::*;
use llhd::TimeValue;

pub fn load_egraph(filename: &str) -> (EGraph, Vec<String>) {
    let mut egglog_program_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    egglog_program_file_path.push("../resources/egglog");
    egglog_program_file_path.push(filename);
    let egglog_program_str: String = fs::read_to_string(egglog_program_file_path).unwrap();
    let mut egraph = EGraph::default();
    let msgs = egraph
        .parse_and_run_program(None, &egglog_program_str)
        .expect("Failure to run program on egraph.");
    (egraph, msgs)
}

pub fn get_egglog_commands(filename: &str) -> String {
    let mut egglog_program_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    egglog_program_file_path.push("../resources/egglog");
    egglog_program_file_path.push(filename);
    fs::read_to_string(egglog_program_file_path).unwrap()
}

pub fn load_egraph_rewrite_rules(
    rewrite_filename: &str,
    egraph: &mut EGraph,
) -> Result<Vec<String>, Error> {
    let mut egglog_program_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    egglog_program_file_path.push("../resources/egglog");
    egglog_program_file_path.push(rewrite_filename);
    let egglog_program_str: String = fs::read_to_string(egglog_program_file_path).unwrap();
    egraph.parse_and_run_program(None, &egglog_program_str)
}

pub fn trim_expr_whitespace(expr_str: &str) -> String {
    let newline_stripped_expr = expr_str.replace('\n', "");
    let words = newline_stripped_expr.split_whitespace().collect_vec();
    words.join(" ")
}

pub fn load_llhd_module(filename: &str) -> Module {
    let mut llhd_module_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    llhd_module_file_path.push("../resources/llhd");
    llhd_module_file_path.push(filename);
    let llhd_module_str: String = fs::read_to_string(llhd_module_file_path).unwrap();
    llhd::assembly::parse_module(llhd_module_str)
        .expect(&format!("Error loading module: {}", filename))
}

pub fn build_entity_alpha(name: UnitName) -> UnitData {
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

pub fn build_entity_2and_1or_common(name: UnitName) -> UnitData {
    let mut sig = Signature::new();
    let in1 = sig.add_input(llhd::int_ty(1));
    let in2 = sig.add_input(llhd::int_ty(1));
    let in3 = sig.add_input(llhd::int_ty(1));
    let _in4 = sig.add_input(llhd::int_ty(1));
    let out1 = sig.add_output(llhd::signal_ty(llhd::int_ty(1)));
    let mut ent = UnitData::new(UnitKind::Entity, name, sig);
    {
        let mut builder = UnitBuilder::new_anonymous(&mut ent);
        let in1_val = builder.unit().arg_value(in1);
        let in2_val = builder.unit().arg_value(in2);
        let in3_val = builder.unit().arg_value(in3);
        let out1_val = builder.unit().arg_value(out1);
        let null_time = builder.ins().const_time(TimeValue::zero());
        let and1 = builder.ins().and(in1_val, in2_val);
        let and2 = builder.ins().and(in3_val, in2_val);
        let or1 = builder.ins().or(and1, and2);
        builder.ins().drv(out1_val, or1, null_time);
    }
    Unit::new_anonymous(&ent).verify();
    ent
}

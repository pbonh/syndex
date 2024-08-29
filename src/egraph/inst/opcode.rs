use std::collections::HashMap;

use egglog::ast::Symbol;
use lazy_static::lazy_static;
use llhd::ir::Opcode;

type LLHDOpcodeSymbolLookup = HashMap<Symbol, Opcode>;

lazy_static! {
    pub(in crate::egraph) static ref OPCODESYMBOLMAP: LLHDOpcodeSymbolLookup = {
        let mut opcode_symbol_map = HashMap::new();
        opcode_symbol_map.insert(opcode_symbol(Opcode::ConstInt), Opcode::ConstInt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ConstTime), Opcode::ConstTime);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Alias), Opcode::Alias);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ArrayUniform), Opcode::ArrayUniform);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Array), Opcode::Array);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Struct), Opcode::Struct);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Not), Opcode::Not);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Neg), Opcode::Neg);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Add), Opcode::Add);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sub), Opcode::Sub);
        opcode_symbol_map.insert(opcode_symbol(Opcode::And), Opcode::And);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Or), Opcode::Or);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Xor), Opcode::Xor);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Smul), Opcode::Smul);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sdiv), Opcode::Sdiv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Smod), Opcode::Smod);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Srem), Opcode::Srem);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Umul), Opcode::Umul);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Udiv), Opcode::Udiv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Umod), Opcode::Umod);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Urem), Opcode::Urem);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Eq), Opcode::Eq);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Neq), Opcode::Neq);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Slt), Opcode::Slt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sgt), Opcode::Sgt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sle), Opcode::Sle);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sge), Opcode::Sge);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ult), Opcode::Ult);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ugt), Opcode::Ugt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ule), Opcode::Ule);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Uge), Opcode::Uge);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Shl), Opcode::Shl);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Shr), Opcode::Shr);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Mux), Opcode::Mux);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Reg), Opcode::Reg);
        opcode_symbol_map.insert(opcode_symbol(Opcode::InsField), Opcode::InsField);
        opcode_symbol_map.insert(opcode_symbol(Opcode::InsSlice), Opcode::InsSlice);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ExtField), Opcode::ExtField);
        opcode_symbol_map.insert(opcode_symbol(Opcode::ExtSlice), Opcode::ExtSlice);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Con), Opcode::Con);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Del), Opcode::Del);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Call), Opcode::Call);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Inst), Opcode::Inst);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Sig), Opcode::Sig);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Prb), Opcode::Prb);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Drv), Opcode::Drv);
        opcode_symbol_map.insert(opcode_symbol(Opcode::DrvCond), Opcode::DrvCond);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Var), Opcode::Var);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ld), Opcode::Ld);
        opcode_symbol_map.insert(opcode_symbol(Opcode::St), Opcode::St);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Halt), Opcode::Halt);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Ret), Opcode::Ret);
        opcode_symbol_map.insert(opcode_symbol(Opcode::RetValue), Opcode::RetValue);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Phi), Opcode::Phi);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Br), Opcode::Br);
        opcode_symbol_map.insert(opcode_symbol(Opcode::BrCond), Opcode::BrCond);
        opcode_symbol_map.insert(opcode_symbol(Opcode::Wait), Opcode::Wait);
        opcode_symbol_map.insert(opcode_symbol(Opcode::WaitTime), Opcode::WaitTime);
        opcode_symbol_map
    };
    pub(in crate::egraph) static ref OPCODESYMBOLMAP_COUNT: usize = OPCODESYMBOLMAP.len();
}

fn uppercase_first_letter(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

pub(in crate::egraph) fn opcode_symbol(opcode: Opcode) -> Symbol {
    let mut opcode_str = opcode.to_string();
    match opcode {
        Opcode::ConstTime => opcode_str.push_str("Time"),
        Opcode::ConstInt => opcode_str.push_str("Int"),
        Opcode::DrvCond => opcode_str.push_str("Cond"),
        _ => (),
    }
    uppercase_first_letter(&mut opcode_str);
    Symbol::new(opcode_str)
}

pub(in crate::egraph) fn get_symbol_opcode(symbol: &Symbol) -> Option<Opcode> {
    OPCODESYMBOLMAP.get(symbol).copied()
}

pub(in crate::egraph) fn symbol_opcode(symbol: Symbol) -> Opcode {
    OPCODESYMBOLMAP[&symbol]
}

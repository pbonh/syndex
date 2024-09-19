pub(crate) mod egglog_names;
pub mod facts;
#[macro_use]
pub mod macros;
pub mod builder;
pub mod rules;
pub mod schedule;
pub mod sorts;
pub use builder::*;
use egglog::ast::Command;

pub(crate) type EgglogCommandList = Vec<Command>;

#[cfg(test)]
mod tests {

    #[test]
    fn egglog_type_macros() {
        use egglog::ast::*;

        // let set_option_cmd = cmd!(SetOption {
        //     name: "node_limit",
        //     value: 1000,
        // });

        let _datatype_cmd: Command = cmd!(Datatype {
            span: span!(),
            name: "Math",
            variants: vec![
                variant!("Num", ["i64"]),
                variant!("Var", ["String"]),
                variant!("Add", ["Math", "Math"]),
                variant!("Mul", ["Math", "Math"]),
            ],
        });

        // let function_cmd = cmd!(
        //     Function(function_decl!(
        //         "Add",
        //         inputs = ["Math", "Math"],
        //         output = "Math"
        //     ))
        // );
        // Optional fields can be added here
        // default = expr!(0),
        // cost = Some(1),

        // let print_function_cmd = cmd!(PrintFunction(span!(), "Add", 20));

        // let rewrite_cmd = cmd!(Rewrite(
        //     symbol!("commute_add"),
        //     GenericRewrite {
        //         span: span!(),
        //         lhs: expr!("Add", var "a", var "b"),
        //         rhs: expr!("Add", var "b", var "a"),
        //         conditions: vec![],
        //     },
        //     false,
        // ));
        //
        // let run_schedule_cmd = cmd!(
        //     RunSchedule(
        //         schedule!(sequence [
        //             saturate run "my_ruleset_1",
        //             run "my_ruleset_2", until = [("eq", [expr!(var "x"), 0])]
        // ])));
        //
        // let check_cmd = cmd!(
        //     Check(
        //         span!(),
        //         facts = [
        //             eq [expr!(var "x"), 0],
        //             expr!("greater_than", var "y", 10)
        //         ]
        //     )
        // );
    }

    #[test]
    const fn egglog_syntax_macros() {
        use egglog_syntax::egglog_expr_str;

        let _llhd_dfg_egglog_expr = egglog_expr_str!(
            r#"
            (datatype LLHDValue (Value u64)) (sort LLHDVecValue (Vec u64))
            (datatype LLHDBlock (Block u64)) (sort LLHDVecBlock (Vec LLHDBlock))
            (datatype LLHDExtUnit (ExtUnit u64))
            (datatype LLHDRegMode
                (Low)
                (High)
                (Rise)
                (Fall)
                (Both))
            (sort LLHDVecRegMode (Vec LLHDRegMode))
            (datatype LLHDDFG
                (ValueRef u64)
                (ConstInt String)
                (ConstTime String)
                (Alias LLHDDFG)
                (ArrayUniform u64 LLHDDFG)
                (Array LLHDVecValue)
                (Struct LLHDVecValue)
                (Not LLHDDFG)
                (Neg LLHDDFG)
                (Add LLHDDFG LLHDDFG)
                (Sub LLHDDFG LLHDDFG)
                (And LLHDDFG LLHDDFG)
                (Or LLHDDFG LLHDDFG)
                (Xor LLHDDFG LLHDDFG)
                (Smul LLHDDFG LLHDDFG)
                (Sdiv LLHDDFG LLHDDFG)
                (Smod LLHDDFG LLHDDFG)
                (Srem LLHDDFG LLHDDFG)
                (Umul LLHDDFG LLHDDFG)
                (Udiv LLHDDFG LLHDDFG)
                (Umod LLHDDFG LLHDDFG)
                (Urem LLHDDFG LLHDDFG)
                (Eq LLHDDFG LLHDDFG)
                (Neq LLHDDFG LLHDDFG)
                (Slt LLHDDFG LLHDDFG)
                (Sgt LLHDDFG LLHDDFG)
                (Sle LLHDDFG LLHDDFG)
                (Sge LLHDDFG LLHDDFG)
                (Ult LLHDDFG LLHDDFG)
                (Ugt LLHDDFG LLHDDFG)
                (Ule LLHDDFG LLHDDFG)
                (Uge LLHDDFG LLHDDFG)
                (Shl LLHDDFG LLHDDFG LLHDDFG)
                (Shr LLHDDFG LLHDDFG LLHDDFG)
                (Mux LLHDDFG LLHDDFG)
                (Reg LLHDVecValue LLHDVecRegMode)
                (InsField LLHDDFG LLHDDFG u64 u64)
                (InsSlice LLHDDFG LLHDDFG u64 u64)
                (ExtField LLHDDFG LLHDDFG u64 u64)
                (ExtSlice LLHDDFG LLHDDFG u64 u64)
                (Con LLHDDFG LLHDDFG)
                (Del LLHDDFG LLHDDFG LLHDDFG)
                (Call LLHDExtUnit u64 LLHDVecValue)
                (Inst LLHDExtUnit u64 LLHDVecValue)
                (Sig LLHDDFG)
                (Prb LLHDDFG)
                (Drv LLHDDFG LLHDDFG LLHDDFG)
                (DrvCond LLHDDFG LLHDDFG LLHDDFG LLHDDFG)
                (Var LLHDDFG)
                (Ld LLHDDFG)
                (St LLHDDFG LLHDDFG)
                (Halt)
                (Ret)
                (RetValue LLHDDFG)
                (Phi LLHDVecValue LLHDVecBlock)
                (Br LLHDBlock)
                (BrCond LLHDDFG LLHDBlock LLHDBlock)
                (Wait LLHDBlock LLHDVecValue)
                (WaitTime LLHDBlock LLHDVecValue)
                (LLHDUnit LLHDDFG)
            )
        "#
        );
    }
}

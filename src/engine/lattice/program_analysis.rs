// use ascent::ascent;
//
// struct Stmt {
//     x: String,
//     y: String,
// }
//
// struct Prog(Vec<Stmt>); // Assuming Prog is a list of Stmt
//
// struct Pt {
//     todo: Prog,
//     hist: Prog, // Assuming Ctx is the same type as Prog
// }
//
// type Run = String;
// type Value = i32;
//
// // Assuming $Move in Datalog is a function to create a Stmt
// fn move_stmt(x: &str, y: &str) -> Stmt {
//     Stmt {
//         x: x.to_string(),
//         y: y.to_string(),
//     }
// }
//
// ascent! {
//     relation var_store(Run, Pt, String, Value);
//
//     var_store("run0", Pt { todo: Prog(vec![move_stmt("x", "y"), move_stmt("y", "z")]), hist: Prog(vec![]) }, "x", 3);
//     var_store("run0", Pt { todo: Prog(vec![move_stmt("x", "y"), move_stmt("y", "z")]), hist: Prog(vec![]) }, "y", 4);
//     // ... Add more var_store entries as per the Datalog example ...
//
//     var_store(run, Pt { todo, hist }, x, v) <--
//         var_store(run, Pt { todo: Prog(vec![stmt.clone()]), hist }, x, v),
//         let stmt = move_stmt(y, _z),
//         x != y;
//
//     var_store(run, Pt { todo, hist }, x, v) <--
//         var_store(run, pt, x, _),
//         let pt = Pt { todo: Prog(vec![stmt.clone()]), hist },
//         let stmt = move_stmt(x, z),
//         var_store(run, pt, z, v);
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_var_store() {
//         let mut prog = AscentProgram::default();
//         // Populate the `edge` relation as needed
//         prog.run();
//
//         // Assertions to validate the state of `var_store` relation
//         assert!(prog.var_store.contains(&("run0".to_string(), Pt { ... }, "x".to_string(), 3)));
//     }
// }

// use ascent::ascent;
//
// #[derive(Clone, PartialEq, Eq, Hash)]
// struct Stmt {
//     x: String,
//     y: String,
// }
//
// #[derive(Clone, PartialEq, Eq, Hash)]
// struct Prog(Vec<Stmt>); // Assuming Prog is a list of Stmt
//
// #[derive(Clone, PartialEq, Eq, Hash)]
// struct Pt {
//     todo: Prog,
//     hist: Prog, // Assuming Ctx is the same type as Prog
// }
//
// type Run = String;
// type Value = i32;
//
// struct VarStore {
//     run: Run,
//     pt: Pt,
//     x: String,
//     v: Value,
// }
//
// ascent! {
//     relation var_store(Run, Pt, String, Value);
//
//     // Seed data for var_store
//     var_store("run0", Pt { todo: Prog(vec![Stmt { x: "x".to_string(), y: "y".to_string() }, Stmt { x: "y".to_string(), y: "z".to_string() }]), hist: Prog(vec![]) }, "x", 3);
//     var_store("run0", Pt { todo: Prog(vec![Stmt { x: "x".to_string(), y: "y".to_string() }, Stmt { x: "y".to_string(), y: "z".to_string() }]), hist: Prog(vec![]) }, "y", 4);
//     // ... Add more var_store entries as per the Datalog example ...
//
//     var_store(run, pt, x, v) <--
//         var_store(run, Pt { todo: Prog(vec![stmt.clone()]), hist }, x, v),
//         let Stmt { x: y, y: _z } = stmt,
//         if x != y;
//
//     // var_store(run, pt, x, v) <--
//     //     var_store(run, pt_old, x, _),
//     //     let pt_old = Pt { todo: Prog(vec![stmt.clone()]), hist },
//     //     let Stmt { x, y: z } = stmt,
//     //     if let Some(v) = var_store(run, pt_old, z, ?v) {
//     //         var_store(run, pt, x, v);
//     //     };
//     // var_store(run, pt, x, v) <--
//     //     var_store(run, pt_old, x, _),
//     //     let pt_old = Pt { todo: Prog(vec![stmt.clone()]), hist },
//     //     let Stmt { x, y: z } = stmt,
//     //     for (run, _, _, v) in var_store {
//     //         if pt_old == pt && z == x {
//     //             var_store(run, pt, x, v);
//     //         }
//     //     };
//     // var_store(run, pt, x, v) <--
//     //     var_store(run, pt_old, x, _),
//     //     let pt_old = Pt { todo: Prog(vec![stmt.clone()]), hist },
//     //     let Stmt { x: x_from_stmt, y: z } = stmt,
//     //     var_store(run, pt_old, z, ?v),
//     //     if pt_old == pt && x_from_stmt == x;
//     //
//     var_store(run, pt, x, v) <--
//         var_store(run, pt_old, x, _),
//         let cloned_stmt = stmt.clone(),
//         let new_pt = Pt { todo: Prog(vec![cloned_stmt]), hist },
//         let Stmt { x: x_from_stmt, y: z } = stmt,
//         var_store(run, new_pt, z, ?v),
//         if new_pt == pt && x_from_stmt == x;
//
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_var_store() {
//         let mut prog = AscentProgram::default();
//         // Populate the `var_store` relation as needed
//         prog.run();
//
//         // Assertions to validate the state of `var_store` relation
//         // Example:
//         // assert!(prog.var_store.contains(&("run0".to_string(), Pt { ... }, "x".to_string(), 3)));
//     }
// }

// use ascent::ascent;
//
// ascent! {
//     // Define relations and lattices
//     relation instruction(i32, String, Vec<String>);  // (Id, Operator, Operands)
//     relation common_subexpr(i32, i32);  // (Id1, Id2)
//
//     // Rule to find common subexpressions
//     common_subexpr(Id1, Id2) <--
//         instruction(Id1, Op.clone(), Operands.clone()),
//         instruction(Id2, Op, Operands),
//         Id1 != Id2;
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_cse_example() {
//         let mut prog = AscentProgram {
//             instruction: vec![
//                 (1, "add".to_string(), vec!["a".to_string(), "b".to_string()]),
//                 (2, "mul".to_string(), vec!["1".to_string(), "c".to_string()]),
//                 (3, "add".to_string(), vec!["a".to_string(), "b".to_string()]),
//             ],
//             ..Default::default()
//         };
//         prog.run();
//
//         // Check if the common subexpression (1, 3) is found
//         assert!(
//             prog.common_subexpr.contains(&(1, 3)),
//             "Common Subexpression (1, 3) should be present in program."
//         );
//     }
// }

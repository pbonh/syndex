extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn egglog_expr_str(input: TokenStream) -> TokenStream {
    let input_lit = parse_macro_input!(input as LitStr);
    let input_str = input_lit.value();

    fn split_sexpressions(expr_str: &str) -> Result<Vec<&str>, String> {
        let mut exprs = Vec::new();
        let mut paren_level = 0;
        let mut start = None;
        let mut chars = expr_str.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            match c {
                '(' => {
                    if paren_level == 0 {
                        start = Some(i);
                    }
                    paren_level += 1;
                }
                ')' => {
                    paren_level -= 1;
                    if paren_level < 0 {
                        return Err(format!(
                            "Unmatched closing parenthesis at byte position {}",
                            i
                        ));
                    }
                    if paren_level == 0 {
                        let end = chars.peek().map_or(expr_str.len(), |&(next_i, _)| next_i);
                        let expr = &expr_str[start.unwrap()..end];
                        exprs.push(expr.trim());
                        start = None;
                    }
                }
                _ => {}
            }
        }

        if paren_level != 0 {
            return Err("Unmatched opening parenthesis".to_string());
        }

        Ok(exprs)
    }

    match split_sexpressions(&input_str) {
        Ok(exprs) => {
            for (idx, expr) in exprs.iter().enumerate() {
                match sexp::parse(expr) {
                    Ok(_sexpr) => {}
                    Err(e) => {
                        let err_msg =
                            format!("Invalid s-expression at expression {}: {}", idx + 1, e);
                        return quote!(compile_error!(#err_msg);).into();
                    }
                }
            }
            quote!(#input_lit).into()
        }
        Err(e) => {
            let err_msg = format!("Error parsing s-expressions: {}", e);
            quote!(compile_error!(#err_msg);).into()
        }
    }
}

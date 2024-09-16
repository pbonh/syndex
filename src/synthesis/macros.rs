// macro_rules! synthesize {
//     ($chip:expr, $($func:expr),+) => {{
//         let mut synth_chip = clift($chip);
//         $(
//             synth_chip = synthesis_compose(synth_chip, $func);
//         )+
//         synth_chip
//     }};
// }

// macro_rules! synthesize {
//     ($f:expr, $g:expr $(, $rest:expr)*) => {{
//         let mut composed = synthesis_compose($f, $g);
//         $(
//             composed = synthesis_compose(composed, $rest);
//         )*
//         composed
//     }};
// }

// macro_rules! synthesize {
//     ($func:expr, $($rest:expr),+) => {{
//         let mut composed_func = $func;
//         $(
//             composed_func = synthesis_compose(composed_func, $rest);
//         )+
//         composed_func
//     }};
// }

// macro_rules! synthesize {
//     ( $f:expr ) => {
//         $f
//     };
//     ( $first:expr, $($rest:expr),+ ) => {
//         {
//             let rest = synthesize!($($rest),+);
//             move |x| rest($first(x))
//         }
//     };
// }

macro_rules! synthesize {
    ( $f:expr ) => { $f };
    ( $f:expr, $g:expr ) => {
        synthesis_compose($f, $g)
    };
    ( $f:expr, $($rest:tt)+ ) => {
        synthesis_compose($f, synthesize!($($rest)+))
    };
}

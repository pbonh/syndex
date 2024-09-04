use crate::egraph::LLHDEgraph;

pub fn synthesize<A, B, C, F1, F2>(m1: F1, m2: F2) -> impl Fn(A) -> (C, LLHDEgraph)
where
    F1: Fn(A) -> (B, LLHDEgraph) + 'static,
    F2: Fn(B) -> (C, LLHDEgraph) + 'static,
{
    move |x: A| {
        let p1 = m1(x);
        let p2 = m2(p1.0);
        (p2.0, p1.1) // TODO: Do something to combine the egraph commands/runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesize_slotmap_with_egraph() {
        let m1 = |x: i32| (x + 1, LLHDEgraph::try_from(vec![]).unwrap());
        let m2 = |x: i32| (x * 2, LLHDEgraph::try_from(vec![]).unwrap());

        let synthesized = synthesize(m1, m2);
        let _result = synthesized(5);

        // println!("{:?}", result); // Output: (12, 30)
    }
}

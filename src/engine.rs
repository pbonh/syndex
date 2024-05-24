use ascent::ascent;

mod lattice;

ascent! {
    relation edge(i32, i32);
    relation path(i32, i32);

    path(x, y) <-- edge(x, y);
    path(x, z) <-- edge(x, y), path(y, z);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascent_example() {
        // let mut prog = AscentProgram::default();
        let mut prog = AscentProgram {
            edge: vec![(1, 2), (2, 3)],
            ..Default::default()
        };
        prog.run();
        assert!(
            prog.path.contains(&(1, 3)),
            "Path (1, 3) should be present in program."
        );
    }
}

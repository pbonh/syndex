use mhgl::*;

#[derive(Debug)]
struct Foo(u8);

#[derive(Debug)]
struct Bar(u32);

type ConGraphExample = ConGraph;
type HGraphExample = HGraph<Foo, Bar>;

#[cfg(test)]
mod tests {
    use mhgl::HyperGraph;

    use super::*;

    #[test]
    fn mhgl_doc_example() {
        let mut cg = ConGraphExample::new();
        let nodes = cg.add_nodes(5);
        let mut edges = Vec::new();
        for ix in 1..nodes.len() {
            let edge = cg.add_edge(&nodes[0..=ix]).unwrap();
            edges.push(edge);
        }
        let maxs_of_edge = cg.maximal_edges(&edges[0]);
        let maxs_of_nodes = cg.maximal_edges_of_nodes([0, 1, 2]);

        assert_eq!(maxs_of_edge[0], edges[edges.len() - 1]);
        assert_eq!(maxs_of_nodes[0], edges[edges.len() - 1]);
        assert_eq!(cg.boundary_up(&edges[0]), vec![edges[1]]);

        let mut hg = HGraphExample::new();
        let n0 = hg.add_node(Foo(1));
        let n1 = hg.add_node(Foo(2));
        let e = hg.add_edge(&[n0, n1], Bar(42)).unwrap();
        let e_mut = hg.borrow_edge_mut(&e).unwrap();
        e_mut.0 = 12;
        let bar = hg.remove_edge(e).unwrap();
        assert_eq!(bar.0, 12);
    }
}

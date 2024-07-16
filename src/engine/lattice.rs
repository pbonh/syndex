mod program_analysis;

mod lattice_ascent_doc_example {

    use ascent::{ascent, Dual};

    ascent! {
        lattice shortest_path(i32, i32, Dual<u32>);
        relation edge(i32, i32, u32);

        shortest_path(x, y, Dual(*w)) <-- edge(x, y, w);

        shortest_path(x, z, Dual(w + l)) <--
            edge(x, y, w),
            shortest_path(y, z, ?Dual(l));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ascent_lattice_example() {
            let mut prog = AscentProgram {
                edge: vec![(1, 2, 1), (2, 3, 1), (1, 3, 5)],
                ..Default::default()
            };
            prog.run();
            assert!(
                prog.shortest_path.contains(&(1, 3, Dual(2))),
                "Shortest Path (1, 3, 2) should be present in program."
            );
        }
    }
}

mod clause_ascent_doc_example {
    use std::rc::Rc;

    use ascent::ascent;

    ascent! {
        relation node(i32, Rc<Vec<i32>>);
        relation edge(i32, i32);

        edge(x, y) <--
            node(x, neighbors),
            for y in neighbors.iter(),
            if x != y;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ascent_clause_example() {
            let node1 = (1, Rc::new(vec![2, 3]));
            let node2 = (2, Rc::new(vec![3]));
            let node3 = (3, Rc::new(vec![1, 2]));

            let node4 = (4, Rc::new(vec![5]));
            let node5 = (5, Rc::new(vec![4]));
            let mut prog = AscentProgram {
                node: vec![node1, node2, node3, node4, node5],
                ..Default::default()
            };
            prog.run();
            assert!(
                prog.edge.contains(&(1, 2)),
                "Edge(1,2) should exist in graph."
            );
            assert!(
                prog.edge.contains(&(1, 3)),
                "Edge(1,3) should exist in graph."
            );
            assert!(
                prog.edge.contains(&(2, 3)),
                "Edge(2,3) should exist in graph."
            );
            assert!(
                prog.edge.contains(&(3, 1)),
                "Edge(3,1) should exist in graph."
            );
            assert!(
                prog.edge.contains(&(3, 2)),
                "Edge(3,2) should exist in graph."
            );

            assert!(
                !prog.edge.contains(&(2, 1)),
                "Edge(2,1) should not exist in graph, since only non-reflexive edges should exist."
            );
            assert!(
                !prog.edge.contains(&(2, 4)),
                "Edge(2,4) should not exist in graph, since they are not neighbors."
            );
            assert!(
                !prog.edge.contains(&(3, 4)),
                "Edge(3,4) should not exist in graph, since they are not neighbors."
            );
        }
    }
}

mod chatterjee_brayton_physical_synthesis_example {
    use ascent::ascent;

    type SortedRectEdgeList = Vec<(i32, TopBottom)>;
    type OptimumRegionCost = (i32, i32, i32);

    #[allow(dead_code)]
    #[derive(Clone, Copy, Hash, Eq, PartialEq)]
    enum TopBottom {
        Top,
        Bottom,
    }

    #[allow(dead_code)]
    fn calculate_optimum_region(sorted_regions: &[(i32, TopBottom)]) -> OptimumRegionCost {
        let middle = sorted_regions.len() / 2;
        let mut optimum_region: OptimumRegionCost = (0, 0, 0);
        for (ii, region_node) in sorted_regions.iter().enumerate() {
            let idx = ii + 1;
            let yi = region_node.0;
            let sidei = region_node.1;
            if idx <= middle && sidei == TopBottom::Bottom {
                optimum_region.2 += -yi;
            } else if idx > middle && sidei == TopBottom::Top {
                optimum_region.2 += yi;
            }
            if idx == middle {
                optimum_region.0 = yi;
            } else if idx == middle + 1 {
                optimum_region.1 = yi;
            }
        }
        optimum_region
    }

    ascent! {
        relation sorted_rect_edge_list(SortedRectEdgeList);
        relation optimum_region(i32,i32,i32);

        optimum_region(y1, y2, cost) <--
            sorted_rect_edge_list(region_nodes),
            let (y1, y2, cost) = calculate_optimum_region(region_nodes);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_chatterjee_brayton_paper_example() {
            let example_divisor_region_list: SortedRectEdgeList = vec![
                (0, TopBottom::Top),
                (0, TopBottom::Top),
                (2, TopBottom::Top),
                (4, TopBottom::Bottom),
                (4, TopBottom::Bottom),
                (12, TopBottom::Top),
                (12, TopBottom::Top),
                (14, TopBottom::Bottom),
                (14, TopBottom::Bottom),
                (18, TopBottom::Top),
                (20, TopBottom::Bottom),
                (20, TopBottom::Bottom),
            ];
            let mut prog = AscentProgram {
                sorted_rect_edge_list: vec![(example_divisor_region_list,)],
                ..Default::default()
            };
            prog.run();
            assert!(
                !prog.optimum_region.is_empty(),
                "The input regions of this example should produce an optimum region."
            );
            assert!(
                !prog.optimum_region.contains(&(0, 0, 0)),
                "The input regions of this example should produce an optimum region, not a null \
                 value(0, 0, 0)."
            );
            assert!(
                prog.optimum_region.contains(&(12, 12, 22)),
                "Optimal region should be (12, 12) between edges 6 and 7 with Cost = -(4+4) + \
                 (12+18) = 22."
            );
        }
    }
}

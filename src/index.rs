pub mod macros;

use euclid::Box2D;
use hypergraph::VertexIndex;
use llhd::ir::prelude::*;
use std::collections::BTreeSet;
use std::hash::Hash;

pub trait FlatIndex: Clone + PartialEq + Eq + Hash {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignUnitIndex(UnitId, BTreeSet<VertexIndex>, Box2D<usize, ()>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignNodeIndex(
    UnitId,
    Block,
    Inst,
    Value,
    BTreeSet<VertexIndex>,
    Box2D<usize, ()>,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ascent::*;
    use euclid::Point2D;
    use llhd::table::TableKey;
    use std::rc::Rc;

    #[test]
    fn ascent_column_compatability_design_unit_index() {
        ascent! {
           relation node(DesignUnitIndex, Rc<Vec<DesignUnitIndex>>);
           relation edge(DesignUnitIndex, DesignUnitIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let unit1_nets = VertexIndex::from(1);
        let unit_loc = Point2D::zero();
        let unit1 = DesignUnitIndex(UnitId::new(1), BTreeSet::from([unit1_nets]), Box2D::new(unit_loc, unit_loc));
        let unit2_nets = VertexIndex::from(2);
        let unit2 = DesignUnitIndex(UnitId::new(2), BTreeSet::from([unit2_nets]), Box2D::new(unit_loc, unit_loc));
        let mut prog = AscentProgram::default();
        prog.edge = vec![(unit1, unit2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_node_index() {
        ascent! {
           relation node(DesignNodeIndex, Rc<Vec<DesignNodeIndex>>);
           relation edge(DesignNodeIndex, DesignNodeIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = VertexIndex::from(1);
        let node_loc = Point2D::zero();
        let node1 = DesignNodeIndex(UnitId::new(1), Block::new(0), Inst::new(1), Value::new(usize::max_value()), BTreeSet::from([node1_nets]), Box2D::new(node_loc, node_loc));
        let node2_nets = VertexIndex::from(2);
        let node2 = DesignNodeIndex(UnitId::new(1), Block::new(0), Inst::new(2), Value::new(usize::max_value()), BTreeSet::from([node2_nets]), Box2D::new(node_loc, node_loc));
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }
}

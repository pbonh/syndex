pub mod macros;

use std::collections::BTreeSet;
use std::hash::Hash;

use euclid::default::Box2D;
use llhd::ir::prelude::*;
use llhd::ir::InstData;

use crate::circuit::graph::LCircuitNodeID;

/// Type Constraint for Use in a Datalog Relation Column
pub trait FlatIndex: Clone + PartialEq + Eq + Hash {}

/// `FlatIndex` for Design Units
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignUnitIndex(UnitId, BTreeSet<LCircuitNodeID>, Box2D<usize>);

/// `FlatIndex` for Design Gates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignDGateIndex(
    UnitId,
    Inst,
    InstData,
    BTreeSet<LCircuitNodeID>,
    Box2D<usize>,
);

/// `FlatIndex` for Design Nets
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DesignDNetIndex(UnitId, Inst, Value, BTreeSet<LCircuitNodeID>, Box2D<usize>);

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use ascent::*;
    use euclid::Point2D;
    use llhd::table::TableKey;

    use super::*;

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
        let unit1_nets = 1;
        let unit_loc = Point2D::zero();
        let unit1 = DesignUnitIndex(
            UnitId::new(1),
            BTreeSet::from([unit1_nets]),
            Box2D::new(unit_loc, unit_loc),
        );
        let unit2_nets = 2;
        let unit2 = DesignUnitIndex(
            UnitId::new(2),
            BTreeSet::from([unit2_nets]),
            Box2D::new(unit_loc, unit_loc),
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(unit1, unit2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_gate_index() {
        ascent! {
           relation node(DesignDGateIndex, Rc<Vec<DesignDGateIndex>>);
           relation edge(DesignDGateIndex, DesignDGateIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = 1;
        let node_loc = Point2D::zero();
        let node1_data = InstData::default();
        let node1 = DesignDGateIndex(
            UnitId::new(1),
            Inst::new(1),
            node1_data,
            BTreeSet::from([node1_nets]),
            Box2D::new(node_loc, node_loc),
        );
        let node2_nets = 2;
        let node2_data = InstData::default();
        let node2 = DesignDGateIndex(
            UnitId::new(1),
            Inst::new(2),
            node2_data,
            BTreeSet::from([node2_nets]),
            Box2D::new(node_loc, node_loc),
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }

    #[test]
    fn ascent_column_compatability_design_net_index() {
        ascent! {
           relation node(DesignDNetIndex, Rc<Vec<DesignDNetIndex>>);
           relation edge(DesignDNetIndex, DesignDNetIndex);

           edge(x, y) <--
              node(x, neighbors),
              for y in neighbors.iter(),
              if *x != *y;
        }
        let node1_nets = 1;
        let node_loc = Point2D::zero();
        let node1_net = Value::new(0);
        let node1 = DesignDNetIndex(
            UnitId::new(1),
            Inst::new(1),
            node1_net,
            BTreeSet::from([node1_nets]),
            Box2D::new(node_loc, node_loc),
        );
        let node2_nets = 2;
        let node2_net = Value::new(0);
        let node2 = DesignDNetIndex(
            UnitId::new(1),
            Inst::new(2),
            node2_net,
            BTreeSet::from([node2_nets]),
            Box2D::new(node_loc, node_loc),
        );
        let mut prog = AscentProgram::default();
        prog.edge = vec![(node1, node2)];
        prog.run();
    }
}

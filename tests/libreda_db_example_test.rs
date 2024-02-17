// Copyright (c) 2020-2021 Thomas Kramer.
// SPDX-FileCopyrightText: 2022 Thomas Kramer
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for the `Chip` data structure.

use itertools::Itertools;
use libreda_db::chip::Chip;
use libreda_db::prelude::*;

#[test]
fn test_create_circuit() {
    let mut chip = Chip::new();
    assert_eq!(chip.num_cells(), 0);
    let a = chip.create_cell("A".to_string());
    assert_eq!(chip.num_cells(), 1);
    let b = chip.create_cell("B".to_string());
    assert_eq!(chip.num_cells(), 2);
    assert_eq!(chip.cell_name(&a), "A");
    assert_eq!(chip.cell_name(&b), "B");
    chip.remove_cell(&a);
    assert_eq!(chip.num_cells(), 1);
}

#[test]
fn test_get_cell_by_name() {
    // Find cells by name.
    let mut chip = Chip::new();
    let a = chip.create_cell("A".to_string());
    let b = chip.create_cell("B".to_string());
    assert_eq!(chip.cell_by_name("A"), Some(a));
    assert_eq!(chip.cell_by_name("B"), Some(b));
    assert_eq!(chip.cell_by_name("C"), None);
}

#[test]
fn test_create_sub_circuit() {
    let mut chip = Chip::new();
    let a = chip.create_cell("A".into());
    let b = chip.create_cell("B".into());

    // Create an instance of a in b.
    let inst_a = chip.create_cell_instance(&b, &a, Some("inst_a".into()));
    assert_eq!(chip.num_child_instances(&b), 1);
    assert_eq!(chip.num_cell_references(&a), 1);
    assert_eq!(chip.num_cell_references(&b), 0);

    // Check template and parent relation.
    assert_eq!(chip.template_cell(&inst_a), a);
    assert_eq!(chip.parent_cell(&inst_a), b);

    assert_eq!(chip.each_cell_instance(&b).collect_vec(), vec![inst_a]);
    assert_eq!(chip.each_cell_instance(&a).count(), 0);
    assert_eq!(chip.each_cell_reference(&a).collect_vec(), vec![inst_a]);
    assert_eq!(chip.each_cell_reference(&b).count(), 0);

    // Check dependency relations.
    assert_eq!(chip.num_dependent_cells(&a), 1);
    assert_eq!(chip.num_dependent_cells(&b), 0);
    assert_eq!(chip.num_cell_dependencies(&a), 0);
    assert_eq!(chip.num_cell_dependencies(&b), 1);

    assert_eq!(chip.each_dependent_cell(&a).collect_vec(), vec![b]);
    assert_eq!(chip.each_dependent_cell(&b).collect_vec(), vec![]);
    assert_eq!(chip.each_cell_dependency(&a).collect_vec(), vec![]);
    assert_eq!(chip.each_cell_dependency(&b).collect_vec(), vec![a]);
}

#[test]
fn test_get_sub_circuit_by_name() {
    let mut chip = Chip::new();
    let a = chip.create_cell("A".into());
    let b = chip.create_cell("B".into());

    // Create an instance of a in b.
    let inst1 = chip.create_cell_instance(&b, &a, Some("a1".into()));
    let inst2 = chip.create_cell_instance(&b, &a, Some("a2".into()));

    assert_eq!(chip.cell_instance_by_name(&b, "a1"), Some(inst1));
    assert_eq!(chip.cell_instance_by_name(&b, "a2"), Some(inst2));
    assert_eq!(chip.cell_instance_by_name(&b, "a3"), None);
}

#[test]
fn test_create_pins() {
    let mut chip = Chip::new();
    let a = chip.create_cell("A".into());
    let b = chip.create_cell("B".into());

    let inst1 = chip.create_cell_instance(&b, &a, Some("a1".into()));

    let pin1 = chip.create_pin(&a, "pin1".into(), Direction::Input);
    assert_eq!(chip.num_pins(&a), 1);

    assert_eq!(chip.each_pin_instance(&inst1).count(), 1);
    for pin_inst in chip.each_pin_instance(&inst1) {
        assert_eq!(&chip.template_pin(&pin_inst), &pin1);
    }

    // Find a pin by its name.
    assert_eq!(chip.pin_by_name(&a, "pin1").as_ref(), Some(&pin1));
}

#[test]
fn test_create_nets() {
    let mut chip = Chip::new();
    let a = chip.create_cell("A".into());
    let b = chip.create_cell("B".into());

    let a_net1 = chip.create_net(&a, Some("net1".into()));
    assert_eq!(chip.num_internal_nets(&a), 1 + 2); // TODO: Somehow handle '0' and '1' special nets differently.
    let a_net2 = chip.create_net(&a, Some("net2".into()));
    let b_net2 = chip.create_net(&b, Some("net2".into()));
    assert_eq!(chip.num_internal_nets(&a), 2 + 2);
    assert_eq!(chip.net_by_name(&a, "net1").as_ref(), Some(&a_net1));
    assert_eq!(chip.net_by_name(&a, "net2").as_ref(), Some(&a_net2));
    assert_eq!(chip.net_by_name(&b, "net1").as_ref(), None);
    assert_eq!(chip.net_by_name(&b, "net2").as_ref(), Some(&b_net2));
}

#[test]
fn test_connect_nets() {
    #![allow(unused_variables)]
    let mut chip = Chip::new();
    let a = chip.create_cell("A".into());
    let top = chip.create_cell("TOP".into());

    let a_pin1 = chip.create_pin(&a, "pin1".into(), Direction::Input);
    let a_pin2 = chip.create_pin(&a, "pin2".into(), Direction::Output);
    let a_clk = chip.create_pin(&a, "clk".into(), Direction::Clock);

    let top_pin_clk = chip.create_pin(&top, "clk".into(), Direction::Clock);

    // Create an instance of a in b.
    let inst1 = chip.create_cell_instance(&top, &a, Some("a1".into()));
    let inst2 = chip.create_cell_instance(&top, &a, Some("a2".into()));

    let top_net1 = chip.create_net(&top, Some("net1".into()));
    let top_net2 = chip.create_net(&top, Some("net2".into()));
    let top_clk = chip.create_net(&top, Some("clk".into()));

    // Connect the clock net.
    assert_eq!(chip.connect_pin(&top_pin_clk, Some(top_clk)), None);
    assert_eq!(chip.num_net_terminals(&top_clk), 1);
    let inst1_clk_pin = chip.pin_instance(&inst1, &a_clk);
    let inst2_clk_pin = chip.pin_instance(&inst2, &a_clk);
    assert_eq!(
        chip.connect_pin_instance(&inst1_clk_pin, Some(top_clk)),
        None
    );
    assert_eq!(
        chip.connect_pin_instance(&inst2_clk_pin, Some(top_clk)),
        None
    );
    assert_eq!(chip.num_net_terminals(&top_clk), 3);

    let top_clk_terminals = chip.each_terminal_of_net(&top_clk).collect_vec();

    assert!(top_clk_terminals.contains(&TerminalId::PinId(top_pin_clk)));
    assert!(top_clk_terminals.contains(&TerminalId::PinInstId(inst1_clk_pin)));
    assert!(top_clk_terminals.contains(&TerminalId::PinInstId(inst2_clk_pin)));

    // chip.connect_pin_instance(, Some(top_net1.clone()));
    // chip.connect_pin_instance(, Some(top_net2.clone()));
}

// Check if creating recursive circuits leads to an error.
#[test]
#[should_panic(expected = "Cannot create recursive instances.")]
fn test_circuit_no_recursion_1() {
    let mut chip = Chip::new();
    let top = chip.create_cell("top".into());
    // This should fail:
    let _top_inst = chip.create_cell_instance(&top, &top, Some("top_inst".into()));
}

#[test]
#[should_panic(expected = "Cannot create recursive instances.")]
fn test_circuit_no_recursion_2() {
    let mut chip = Chip::new();
    let top = chip.create_cell("top".into());
    let sub = chip.create_cell("sub".into());
    let _sub_inst = chip.create_cell_instance(&top, &sub, Some("sub_inst".into()));
    // This should fail:
    let _top_inst = chip.create_cell_instance(&sub, &top, Some("recursive_inst".into()));
}

// #[test]
// fn test_simple_net() {
//     let mut netlist = RcNetlist::new();
//     let top = netlist.create_circuit("top", vec![Pin::new_input("A")]);
//     let a = netlist.create_circuit("a", vec![Pin::new_input("A")]);
//     let b = netlist.create_circuit("b", vec![Pin::new_input("A")]);
//     let a_inst = top.create_circuit_instance(&a, Some("a_inst"));
//     let b_inst = top.create_circuit_instance(&b, Some("b_inst"));
//
//     let net1 = top.create_net(Some("Net1"));
//     assert_eq!(net1.parent_circuit().upgrade(), Some(top.clone()));
//
//     assert_eq!(Some(net1.clone()), top.net_by_name("Net1"));
//
//     top.connect_pin_by_id(0, Some(net1.clone()));
//     a_inst.connect_pin_by_id(0, Some(net1.clone()));
//     b_inst.connect_pin_by_id(0, Some(net1.clone()));
//
//     assert_eq!(net1.num_terminals(), 3);
//     assert_eq!(net1.each_terminal().count(), 3);
//
//     assert_eq!(net1.each_terminal()
//                    .filter_map(|t| match t {
//                        TerminalRef::Pin(p) => Some(p),
//                        _ => None
//                    })
//                    .count(), 1, "Number of connections to `Pin`s is wrong.");
//
//     assert_eq!(net1.each_terminal()
//                    .filter_map(|t| match t {
//                        TerminalRef::PinInstance(p) => Some(p),
//                        _ => None
//                    })
//                    .count(), 2, "Number of connections to `PinInstance`s is wrong.");
//
//     assert_eq!(net1.each_instance().unique().count(), 2);
// }

#[test]
fn test_rename_net() {
    let mut chip = Chip::new();
    let top = chip.create_cell("top".into());

    let net1 = chip.create_net(&top, Some("Net1".into()));
    assert_eq!(Some(&net1), chip.net_by_name(&top, "Net1").as_ref());

    // Change name.
    chip.rename_net(&net1, Some("NewName".into()));
    assert_eq!(Some(&net1), chip.net_by_name(&top, "NewName").as_ref());

    // Change back to original.
    chip.rename_net(&net1, Some("Net1".into()));
    assert_eq!(Some(&net1), chip.net_by_name(&top, "Net1").as_ref());

    // No name.
    chip.rename_net(&net1, None);
    assert_eq!(None, chip.net_by_name(&top, "Net1"));
}

#[test]
fn test_flatten_circuit_instance() {
    let mut chip = Chip::new();

    // Create cells.
    let top = chip.create_cell("top".into());
    let top_pin_a = chip.create_pin(&top, "A".into(), Direction::InOut);

    let a = chip.create_cell("a".into());
    let a_pin_a = chip.create_pin(&a, "A".into(), Direction::InOut);
    let b = chip.create_cell("b".into());
    let _b_pin_a = chip.create_pin(&b, "A".into(), Direction::InOut);

    // Create cell instances.
    let a_inst = chip.create_cell_instance(&top, &a, Some("a_inst".into()));
    let b_inst = chip.create_cell_instance(&a, &b, Some("b_inst".into()));

    let net1 = chip.create_net(&top, Some("Net1".into()));
    chip.connect_pin(&top_pin_a, Some(net1));
    let a_pin_inst_a = chip.each_pin_instance_vec(&a_inst);
    chip.connect_pin_instance(&a_pin_inst_a[0], Some(net1));
    assert_eq!(chip.num_net_terminals(&net1), 2);

    let net2 = chip.create_net(&a, Some("Net2".into()));
    let b_pin_inst_a = chip.each_pin_instance_vec(&b_inst);
    chip.connect_pin_instance(&b_pin_inst_a[0], Some(net2));
    chip.connect_pin(&a_pin_a, Some(net2));
    assert_eq!(chip.num_net_terminals(&net2), 2);

    // Flatten the middle circuit.
    chip.flatten_circuit_instance(&a_inst);
    assert_eq!(chip.num_child_instances(&top), 1);
    assert_eq!(chip.cell_instance_by_name(&top, "a_inst"), None);
    assert!(chip.cell_instance_by_name(&top, "a_inst:b_inst").is_some());

    assert_eq!(chip.num_net_terminals(&net1), 2);
}

// Does not work yet. Kept as a reminder to eventually support trait objects.
// #[test]
// fn test_hierarchy_trait_object() {
//
//     // Test that the HierarchyBase trait can be used with trait objects.
//
//     let mut chip = Chip::new();
//
//     fn function_with_trait_object(h: &dyn HierarchyBase<NameType=String, CellId=CellId, CellInstId=CellInstId>) {
//         assert!(true)
//     }
//
//     function_with_trait_object(&chip);
// }

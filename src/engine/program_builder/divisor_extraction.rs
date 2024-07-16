use ascent::ascent_run;
use bevy_ecs::prelude::Component;
use itertools::Itertools;
use llhd::ir::prelude::*;

use crate::llhd_world::world::LLHDWorld;

#[derive(Clone, PartialEq, Eq, Hash, Component)]
struct LLHDInstLocation(usize, usize);

type LLHDProgramWithInstLocation = Vec<(Value, Value, Value, LLHDInstLocation)>;
type LLHDProgramWithInstOpAndLocation = Vec<(Opcode, Value, Value, Value, LLHDInstLocation)>;

fn get_all_llhd_insts(llhd_world: &LLHDWorld, unit_id: UnitId) -> LLHDProgramWithInstOpAndLocation {
    llhd_world
        .unit_program_inst::<LLHDInstLocation>(unit_id)
        .map(|(_inst_idx, inst_component, inst_data)| {
            let opcode = inst_component.data.opcode();
            let inst_value = inst_component.value.unwrap();
            let args = inst_component.data.args();
            let input1 = args[0];
            let input2 = args[1];
            (opcode, inst_value, input1, input2, inst_data)
        })
        .collect()
}

fn get_llhd_insts(
    llhd_world: &LLHDWorld,
    unit_id: UnitId,
    opcode: Opcode,
) -> LLHDProgramWithInstLocation {
    llhd_world
        .unit_program_inst::<LLHDInstLocation>(unit_id)
        .filter(|((_unit_id, _inst_id), inst_component, _inst_data)| {
            opcode == inst_component.data.opcode()
        })
        .map(|(_inst_idx, inst_component, inst_data)| {
            let inst_value = inst_component.value.unwrap();
            let args = inst_component.data.args();
            let input1 = args[0];
            let input2 = args[1];
            (inst_value, input1, input2, inst_data)
        })
        .collect()
}

fn run_divisor_extraction(
    llhd_world: &LLHDWorld,
    unit_id: UnitId,
) -> (
    LLHDProgramWithInstLocation,
    LLHDProgramWithInstOpAndLocation,
) {
    let unit_program_and_inst = get_llhd_insts(llhd_world, unit_id, Opcode::And);
    let unit_program_or_inst = get_llhd_insts(llhd_world, unit_id, Opcode::Or);
    let unit_program_not_inst = get_llhd_insts(llhd_world, unit_id, Opcode::Not);
    let unit_program_all_inst = get_all_llhd_insts(llhd_world, unit_id);

    // Replace a*b + a*c
    // with    a*(b + c)
    let ascent_program = ascent_run! {
        relation llhd_inst(Opcode, Value, Value, Value, LLHDInstLocation) = unit_program_all_inst;
        relation div_llhd_inst(Opcode, Value, Value, Value, LLHDInstLocation);
        relation andi(Value, Value, Value, LLHDInstLocation) = unit_program_and_inst;
        relation ori(Value, Value, Value, LLHDInstLocation) = unit_program_or_inst;
        relation noti(Value, Value, Value, LLHDInstLocation) = unit_program_not_inst;
        relation div_andi(Value, Value, Value, LLHDInstLocation);
        relation div_ori(Value, Value, Value, LLHDInstLocation);
        relation div_noti(Value, Value, Value, LLHDInstLocation);

        div_andi(out_idx, a, and1_idx, and1_loc),
        div_ori(and1_idx, b, c, or_loc)
        <-- ori(out_idx, and1_idx, and2_idx, or_loc),
            andi(and1_idx, a, b, and1_loc),
            andi(and2_idx, a, c, and2_loc);

        div_llhd_inst(Opcode::And, out_idx, a, and1_idx, and1_loc),
        div_llhd_inst(Opcode::Or, and1_idx, b, c, or_loc)
        <-- llhd_inst(Opcode::Or, out_idx, and1_idx, and2_idx, or_loc),
            llhd_inst(Opcode::And, and1_idx, a, b, and1_loc),
            llhd_inst(Opcode::And, and2_idx, a, c, and2_loc);
    };

    (
        ascent_program
            .div_noti
            .into_iter()
            .chain(ascent_program.div_andi)
            .chain(ascent_program.div_ori)
            .collect_vec(),
        ascent_program.div_llhd_inst.into_iter().collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    use std::usize;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::llhd::module::LLHDModule;

    /// Evenly spaces `count` points on a 2D grid of specified dimensions `(width, height)`.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the 2D grid.
    /// * `height` - The height of the 2D grid.
    /// * `count` - The number of points to be placed on the grid.
    ///
    /// # Returns
    ///
    /// A `Vec<(usize, usize)>` containing the coordinates of the points in the order they are placed.
    fn space_to_grid(width: usize, height: usize, count: usize) -> Vec<(usize, usize)> {
        let mut points = Vec::with_capacity(count);

        if count == 0 || width == 0 || height == 0 {
            return points;
        }

        // Calculate approximate spacing between points
        let rows = (count as f64).sqrt().ceil() as usize;
        let cols = if rows == 0 {
            1
        } else {
            (count as f64 / rows as f64).ceil() as usize
        };

        // Adjust rows and columns if necessary
        let rows = if rows > height { height } else { rows };
        let cols = if cols > width { width } else { cols };

        for row in 0..rows {
            for col in 0..cols {
                if points.len() < count {
                    let x = col * (width / cols);
                    let y = row * (height / rows);
                    points.push((x, y));
                }
            }
        }

        points
    }

    #[test]
    fn test_space_to_grid() {
        let width = 10;
        let height = 10;
        let count = 10;
        let points = space_to_grid(width, height, count);

        let expected_points = vec![
            (0, 0),
            (3, 0),
            (6, 0),
            (0, 2),
            (3, 2),
            (6, 2),
            (0, 4),
            (3, 4),
            (6, 4),
            (0, 6),
        ];
        assert_eq!(
            expected_points, points,
            "Placed points don't match expected values."
        );
    }

    // fn space_to_grid(
    //     bb: usize,
    //     inputs: usize,
    //     outputs: usize,
    //     count: usize,
    // ) -> Vec<(usize, usize)> {
    //     let mut positions: Vec<(usize, usize)> = Vec::with_capacity(count);
    //     let half_perimeter = inputs + outputs;
    //     let spacing = bb.div_euclid(count);
    //     let inst_spacings = (0..count)
    //         .enumerate()
    //         .map(|(ii, inst)| (inst, ii * spacing));
    //     positions
    // }
    //
    // #[test]
    // fn space_20_3_1_3_to_grid() {
    //     let expected_positions = vec![(0, 0), (10, 10), (20, 20)];
    //     let actual_positions = space_to_grid(20, 3, 1, 3);
    //     assert_eq!(
    //         expected_positions, actual_positions,
    //         "Position Vectors don't match."
    //     );
    // }

    fn initialize_llhd_unit_relative_locations(
        llhd_world: &mut LLHDWorld,
        unit_id: UnitId,
        bb: (usize, usize),
    ) {
        let unit = llhd_world.module().unit(unit_id);
        let unit_insts = llhd_world
            .module()
            .unit(unit_id)
            .all_insts()
            .filter(|inst| unit.has_result(*inst))
            .filter(|inst| {
                let inst_value = unit.inst_result(*inst);
                unit.get_const(inst_value).is_none()
            })
            .collect_vec();
        // let inputs = unit.input_args().count();
        // let outputs = unit.output_args().count();
        let positions = space_to_grid(bb.0, bb.1, unit_insts.len());
        unit_insts.iter().enumerate().for_each(|(ii, inst)| {
            let position = LLHDInstLocation(positions[ii].0, positions[ii].1);
            llhd_world.set_inst::<LLHDInstLocation>(unit_id, *inst, position)
        });
    }

    #[test]
    fn test_llhd_unit_placement() {
        let input = indoc::indoc! {"
                entity @test_extraction (i1 %in1, i1 %in2, i1 %in3, i1 %in4) -> (i1$ %out1) {
                    %instant = const time 0s 1e
                    %and1 = and i1 %in1, %in2
                    %and2 = and i1 %in3, %in4
                    %or1 = or i1 %and1, %and2
                    drv i1$ %out1, %or1, %instant
                }
            "};
        let expected_locations = vec![(0, 0), (2, 0), (0, 2)];

        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = llhd_world.module().units().next().unwrap().id();
        initialize_llhd_unit_relative_locations(&mut llhd_world, unit_id, (5, 5));
        let unit = llhd_world.module().unit(unit_id);
        let unit_insts = llhd_world
            .module()
            .unit(unit_id)
            .all_insts()
            .filter(|inst| unit.has_result(*inst))
            .filter(|inst| {
                let inst_value = unit.inst_result(*inst);
                unit.get_const(inst_value).is_none()
            })
            .collect_vec();
        assert_eq!(
            expected_locations.len(),
            unit_insts.len(),
            "Expecgted Locations count mismatches Inst count."
        );
        let actual_locations = unit_insts
            .iter()
            .map(|inst| {
                let inst_rel_location = llhd_world
                    .get_inst::<LLHDInstLocation>(unit_id, *inst)
                    .unwrap();
                (inst_rel_location.0, inst_rel_location.1)
            })
            .collect_vec();
        assert_eq!(
            expected_locations, actual_locations,
            "Incorrect Placements."
        );
    }

    #[test]
    fn test_divisor_extraction() {
        // Replace a*b + a*c
        // with    a*(b + c)
        let input = indoc::indoc! {"
                entity @test_extraction (i1 %in1, i1 %in2, i1 %in3) -> (i1$ %out1) {
                    %instant = const time 0s 1e
                    %and1 = and i1 %in1, %in2
                    %and2 = and i1 %in1, %in3
                    %or1 = or i1 %and1, %and2
                    drv i1$ %out1, %or1, %instant
                }
            "};

        let module = llhd::assembly::parse_module(input).unwrap();
        let mut llhd_world = LLHDWorld::new(LLHDModule::from(module));
        let unit_id = llhd_world.module().units().next().unwrap().id();
        initialize_llhd_unit_relative_locations(&mut llhd_world, unit_id, (5, 5));
        let unit_insts = llhd_world.module().unit(unit_id).all_insts().collect_vec();
        assert_eq!(
            6,
            unit_insts.len(),
            "There should be 6 Instructions in original Unit."
        );

        let extracted_divisor_program_all = run_divisor_extraction(&llhd_world, unit_id);
        let extracted_divisor_program_opcode_types = extracted_divisor_program_all.0;
        assert_eq!(
            2,
            extracted_divisor_program_opcode_types.len(),
            "There should be 2 instructions remaining in the extracted program(`a*b + a*c` -> \
             `a*(b + c)`)."
        );

        let extracted_divisor_program_plain = extracted_divisor_program_all.1;
        assert_eq!(
            2,
            extracted_divisor_program_plain.len(),
            "There should be 2 instructions remaining in the extracted program(`a*b + a*c` -> \
             `a*(b + c)`)."
        );
    }
}

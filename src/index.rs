pub mod category;
#[macro_use]
pub mod macros;
pub mod unit;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use euclid::default::Box2D;

    use crate::llhd::LLHDIndex;

    // Usage of the macro
    create_map_struct!(
        OneMapManySubmaps,
        String,           // Key type (K)
        i32,              // Main map value type (V)
        secondary_map1: f32,   // Secondary map 1 type (SecV1)
        secondary_map2: bool   // Secondary map 2 type (SecV2)
    );

    #[test]
    fn create_example_hashmap() {
        // Create an instance of the generated struct
        let mut my_maps = OneMapManySubmaps::new();

        // Inserting values into the main and secondary maps
        my_maps.main_map.insert("key1".to_string(), 42);
        my_maps.secondary_map1.insert("key1".to_string(), 3.14);
        my_maps.secondary_map2.insert("key1".to_string(), true);

        println!("Main map: {:?}", my_maps.main_map);
        println!("Secondary map 1: {:?}", my_maps.secondary_map1);
        println!("Secondary map 2: {:?}", my_maps.secondary_map2);
    }

    define_syn_map!(
        LLHDSlotMapWBoundingBox,
        LLHDKey,
        LLHDIndex,
        bb: Box2D<usize>,
    );

    #[test]
    fn default_llhd_slotmap_example() {
        let empty_llhd_slotmap = LLHDSlotMapWBoundingBox::default();
        let default_llhd_map = empty_llhd_slotmap.llhd_map();
        assert!(default_llhd_map.is_empty());
        let default_bb_map = empty_llhd_slotmap.bb();
        assert!(default_bb_map.is_empty());
    }
}

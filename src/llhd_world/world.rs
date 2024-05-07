use crate::{llhd::module::LLHDModule, world::LWorld};

#[derive(Debug,Default)]
pub struct LLHDWorld {
    pub(crate) module: LLHDModule,
    pub(crate) world: LWorld,
}

impl LLHDWorld {
    pub(crate) fn new() -> Self {
        Self {
            module: LLHDModule::default(),
            world: LWorld::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::create_llhd_world;

    use super::*;

    #[derive(Default, Debug, PartialEq)]
    struct TimingNode {
        name: String,
        delay: f64,
    }

    #[derive(Default, Debug, PartialEq)]
    struct TimingEdge {
        delay: f64,
    }

    #[test]
    fn create_default_llhd_world() {
        let _ = LLHDWorld::default();
    }

    #[test]
    fn create_empty_llhd_world_via_macro() {
        let _llhd_world = create_llhd_world!();
    }

    #[test]
    fn create_llhd_world_via_macro() {
        let input = indoc::indoc! {"
            proc %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1) {
            %init:
                %epsilon = const time 0s 1e
                %in1_prb = prb i1$ %in1
                %in2_prb = prb i1$ %in2
                %in3_prb = prb i1$ %in2
                %and1 = and i1 %in1_prb, %in2_prb
                %and2 = and i1 %in3_prb, %and1
                drv i1$ %out1, %and2, %epsilon
                wait %init for %epsilon
            }

            entity @top () -> () {
                %top_input1 = const i1 0
                %in1 = sig i1 %top_input1
                %top_input2 = const i1 1
                %in2 = sig i1 %top_input2
                %top_input3 = const i1 1
                %in3 = sig i1 %top_input3
                %top_out1 = const i1 0
                %out1 = sig i1 %top_out1
                inst %top.and (i1$ %in1, i1$ %in2, i1$ %in3) -> (i1$ %out1)
            }
        "};
        let module = llhd::assembly::parse_module(input).unwrap();
        let _llhd_world = create_llhd_world!(&module, TimingNode, TimingEdge);
    }
}

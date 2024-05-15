use bevy_ecs::prelude::*;
use llhd::ir::prelude::*;
use llhd::ir::BlockData;

#[derive(Debug, Clone, Default, Component)]
pub struct LLHDBlockComponent {
    pub(crate) id: Option<Block>,
    pub(crate) data: BlockData,
}

impl From<&(Block, BlockData)> for LLHDBlockComponent {
    fn from(inst: &(Block, BlockData)) -> Self {
        Self {
            id: Some(inst.0),
            data: inst.1.clone(),
        }
    }
}

impl PartialEq for LLHDBlockComponent {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id.unwrap() == other.id.unwrap()
        } else {
            false
        }
    }
}

impl Eq for LLHDBlockComponent {}

#[cfg(test)]
mod tests {
    use super::*;
    use llhd::table::TableKey;

    fn build_function(name: UnitName) -> UnitData {
        let mut sig = Signature::new();
        let arg1 = sig.add_input(llhd::int_ty(32));
        let arg2 = sig.add_input(llhd::int_ty(32));
        sig.set_return_type(llhd::int_ty(32));
        let mut func = UnitData::new(UnitKind::Function, name, sig);
        {
            let mut builder = UnitBuilder::new_anonymous(&mut func);
            let arg1 = builder.unit().arg_value(arg1);
            let arg2 = builder.unit().arg_value(arg2);
            let bb1 = builder.block();
            let bb2 = builder.block();
            builder.append_to(bb1);
            let v1 = builder.ins().const_int((32, 4));
            let v2 = builder.ins().const_int((32, 5));
            let v3 = builder.ins().add(v1, v2);
            let v8 = builder.ins().umul(arg1, v3);
            let v9 = builder.ins().not(v8);
            let _v9 = builder.ins().neg(v9);
            builder.ins().br(bb2);
            builder.append_to(bb2);
            let v4 = builder.ins().const_int((32, 1));
            let v5 = builder.ins().add(v3, v4);
            let v6 = builder.ins().add(v5, arg1);
            let v7 = builder.ins().add(arg2, v6);
            builder.ins().ult(v3, v4);
            builder.ins().ugt(v3, v4);
            builder.ins().ule(v3, v4);
            builder.ins().uge(v3, v4);
            builder.ins().ret_value(v7);
        }
        Unit::new_anonymous(&func).verify();
        func
    }

    #[test]
    fn create_inst_component_default() {
        let _unit_component = LLHDBlockComponent::default();
    }

    #[test]
    fn create_block_component() {
        let unit_data = build_function(UnitName::anonymous(0));
        let unit = Unit::new(UnitId::new(0), &unit_data);
        let mut block_components: Vec<LLHDBlockComponent> = Default::default();
        unit.blocks().for_each(|block| {
            let block_data = unit[block].clone();
            block_components.push(LLHDBlockComponent::from(&(block, block_data)));
        });
        assert_eq!(
            2,
            block_components.len(),
            "There should be 2 Blocks defined in Unit."
        );
        assert_eq!(
            Block::new(0),
            block_components[0].id.unwrap(),
            "First Id should be Block with Id: 0"
        );
        assert_eq!(
            Block::new(1),
            block_components[1].id.unwrap(),
            "Second Id should be Block with Id: 1"
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::*;
    use bevy_hierarchy::BuildWorldChildren;
    use llhd::ir::prelude::*;
    use llhd::ir::ValueData;

    #[test]
    fn bevy_entity_basic() {
        #[derive(Component)]
        struct Position {
            x: i32,
            y: i32,
        }
        #[derive(Component)]
        struct Velocity {
            x: i32,
            y: i32,
        }

        let mut world = World::new();

        let entity = world
            .spawn((Position { x: 0, y: 0 }, Velocity { x: 1, y: 0 }))
            .id();

        let entity_ref = world.entity(entity);
        let position = entity_ref.get::<Position>().unwrap();
        let velocity = entity_ref.get::<Velocity>().unwrap();

        assert_eq!(0, position.x);
        assert_eq!(0, position.y);
        assert_eq!(1, velocity.x);
        assert_eq!(0, velocity.y);
    }

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

    fn build_process(name: UnitName) -> UnitData {
        let mut sig = Signature::new();
        let clk = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let inp = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let oup = sig.add_output(llhd::signal_ty(llhd::int_ty(32)));
        let mut prok = UnitData::new(UnitKind::Process, name, sig);
        {
            let mut builder = UnitBuilder::new_anonymous(&mut prok);
            let clk = builder.unit().arg_value(clk);
            let inp = builder.unit().arg_value(inp);
            let _oup = builder.unit().arg_value(oup);
            let entry_bb = builder.block();
            builder.append_to(entry_bb);
            builder.ins().add(clk, inp);
            builder.ins().eq(clk, inp);
            builder.ins().neq(clk, inp);
            builder.ins().halt();
        }
        Unit::new_anonymous(&prok).verify();
        prok
    }

    fn build_entity(name: UnitName) -> UnitData {
        let mut sig = Signature::new();
        let _clk = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let _rst = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let inp = sig.add_input(llhd::signal_ty(llhd::int_ty(1)));
        let _oup = sig.add_output(llhd::signal_ty(llhd::int_ty(32)));
        let mut ent = UnitData::new(UnitKind::Entity, name, sig);
        {
            let mut builder = UnitBuilder::new_anonymous(&mut ent);
            let v1 = builder.ins().const_int((1, 0));
            let v2 = builder.ins().const_int((1, 1));
            let v3 = builder.ins().add(v1, v2);
            let inp = builder.unit().arg_value(inp);
            let inp = builder.ins().prb(inp);
            builder.ins().add(v3, inp);
        }
        Unit::new_anonymous(&ent).verify();
        ent
    }

    #[derive(Debug, Clone, Eq, PartialEq, Component)]
    struct LLHDUnitComponent(Option<UnitId>, UnitName, UnitKind);

    #[derive(Debug, Clone, Component)]
    struct LLHDValueComponent(Option<Value>, ValueData);

    impl Default for LLHDUnitComponent {
        fn default() -> Self {
            Self(None, UnitName::anonymous(0), llhd::ir::UnitKind::Entity)
        }
    }

    impl PartialEq for LLHDValueComponent {
        fn eq(&self, other: &Self) -> bool {
            if self.0.is_some() && other.0.is_some() {
                self.0.unwrap() == other.0.unwrap()
            } else {
                false
            }
        }
    }

    impl Eq for LLHDValueComponent {}

    #[test]
    fn bevy_llhd_example() {
        let mut world = World::new();
        let llhd_entity_data = build_entity(UnitName::Global("top".to_owned()));
        let llhd_entity = Unit::new_anonymous(&llhd_entity_data);
        let llhd_entity_id = llhd_entity.id();
        let llhd_entity_name = llhd_entity.name().to_owned();
        let llhd_entity_kind = llhd_entity.kind();
        let llhd_entity_arg1 = llhd_entity.input_arg(0);
        let llhd_entity_arg1_data = &llhd_entity[llhd_entity_arg1];
        let llhd_value_component =
            LLHDValueComponent(Some(llhd_entity_arg1), llhd_entity_arg1_data.to_owned());
        let _ecs_entity = world
            .spawn(LLHDUnitComponent(
                Some(llhd_entity_id),
                llhd_entity_name,
                llhd_entity_kind,
            ))
            .with_children(|parent_unit| {
                parent_unit.spawn(llhd_value_component);
            });
    }
}

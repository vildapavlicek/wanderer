pub mod scorers;

pub mod actions {
    use bevy::ecs::entity::Entity;
    use bevy::ecs::prelude::*;
    use big_brain::prelude::*;

    // #[derive(Debug, Clone, Copy)]
    // pub struct Attack;
    //
    // impl Attack {
    //     pub fn build() -> AttackBuilder {
    //         AttackBuilder
    //     }
    // }
    //
    // #[derive(Debug, Clone, Copy)]
    // pub struct AttackBuilder;
    //
    // impl ActionBuilder for AttackBuilder {
    //     fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
    //         cmd.entity(action).insert(Attack);
    //     }
    // }

    #[derive(Debug, Clone, Copy, Component, ActionBuilder)]
    pub struct Move;

    impl Move {
        pub fn build() -> MoveBuilder {
            MoveBuilder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct MoveBuilder;

    impl ActionBuilder for MoveBuilder {
        fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
            cmd.entity(action).insert(Move);
        }
    }

    #[derive(Debug, Clone, Copy, Component, ActionBuilder)]
    pub struct Idle;

    impl Idle {
        pub fn build() -> IdleBuilder {
            IdleBuilder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct IdleBuilder;

    impl ActionBuilder for IdleBuilder {
        fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
            cmd.entity(action).insert(Idle);
        }
    }
}

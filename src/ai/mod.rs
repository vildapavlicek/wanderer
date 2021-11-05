pub mod pickers;
pub mod scorers;

pub mod actions {
    use bevy::ecs::entity::Entity;
    use bevy::ecs::prelude::Commands;
    use big_brain::prelude::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Attack;

    impl Attack {
        pub fn build() -> AttackBuilder {
            AttackBuilder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct AttackBuilder;

    impl ActionBuilder for AttackBuilder {
        fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
            cmd.entity(action).insert(Attack);
        }
    }

    #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug, Clone, Copy)]
    pub struct Wander;

    impl Wander {
        pub fn build() -> WanderBuilder {
            WanderBuilder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct WanderBuilder;

    impl ActionBuilder for WanderBuilder {
        fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
            cmd.entity(action).insert(Wander);
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Skip;

    impl Skip {
        pub fn build() -> SkipBuilder {
            SkipBuilder
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SkipBuilder;

    impl ActionBuilder for SkipBuilder {
        fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
            cmd.entity(action).insert(Skip);
        }
    }
}

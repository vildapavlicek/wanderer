use bevy::prelude::Bundle;
use rand::Rng;

#[derive(Bundle)]
pub struct Npc {
    name: super::Name,
    race: super::Race,
    level: super::Level,
    blocking: super::Blocking,
}

#[derive(Bundle)]
pub struct MeeleeEnemy {
    health: super::Health,
    name: super::Name,
    race: super::Race,
    // probably won't need level, or just internally as it should be player level + monster strength
    level: super::Level,
    monster_strength: MonsterStrength,
    blocking: super::Blocking,
    _h: super::Enemy,

    #[bundle]
    stats: super::Stats,
}

impl MeeleeEnemy {
    pub fn new(
        name: String,
        max_health: usize,
        race: super::Race,
        level: usize,
        stats: super::Stats,
    ) -> Self {
        MeeleeEnemy {
            name: super::Name(name),
            health: super::Health::new(max_health as i32),
            race,
            level: super::Level(level as i32),
            monster_strength: MonsterStrength::random(),
            blocking: super::Blocking::enemy(),
            _h: super::Enemy,
            stats,
        }
    }
}

pub enum MonsterStrength {
    Weak,
    Normal,
    Strong,
    Elite,
    Veteran,
    Leader, //todo rename?
    Boss,
}

impl MonsterStrength {
    pub fn get_monster_strength(n: f32) -> Self {
        assert!(n <= 1.);

        match n {
            _ if n > 0.9 => MonsterStrength::Boss,
            // _ if n > 0.8 => MonsterStrength::Leader,
            _ if n > 0.8 => MonsterStrength::Veteran,
            _ if n > 0.7 => MonsterStrength::Elite,
            _ if n > 0.6 => MonsterStrength::Strong,
            _ if n > 0.3 => MonsterStrength::Normal,
            _ => MonsterStrength::Weak,
        }
    }

    pub fn get_level_bonus(&self) -> i32 {
        match self {
            MonsterStrength::Weak => -1,
            MonsterStrength::Normal | MonsterStrength::Leader => 0,
            MonsterStrength::Strong => 1,
            MonsterStrength::Elite => 2,
            MonsterStrength::Veteran => 3,
            MonsterStrength::Boss => 5,
        }
    }

    pub fn random() -> Self {
        let rng = rand::thread_rng().gen_range(0.0..1.0) as f32;
        Self::get_monster_strength(rng)
    }
}

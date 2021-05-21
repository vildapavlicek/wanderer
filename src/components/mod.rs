pub struct Player;
pub struct PlayerCamera;

pub struct Blocking {
    pub blocking_type: BlockingType,
}

impl Blocking {
    pub fn wall() -> Self {
        Self {
            blocking_type: BlockingType::Wall,
        }
    }

    pub fn obstacle() -> Self {
        Self {
            blocking_type: BlockingType::Obstacle,
        }
    }

    pub fn enemy() -> Self {
        Self {
            blocking_type: BlockingType::Enemy,
        }
    }

    pub fn player() -> Self {
        Self {
            blocking_type: BlockingType::Player,
        }
    }

    pub fn is_attackable(&self) -> bool {
        match self.blocking_type {
            BlockingType::Enemy => true,
            _ => false,
        }
    }
}

pub enum BlockingType {
    Wall,
    Obstacle,
    Enemy,
    Player,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Debug)]
pub struct Health {
    pub current: i32,
    max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Health { current: max, max }
    }
}

pub struct Enemy;

pub struct Player;

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
}

pub enum BlockingType {
    Wall,
    Obstacle,
    Enemy,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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

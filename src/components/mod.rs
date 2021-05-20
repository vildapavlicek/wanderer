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

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position { x, y }
    }
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
        Health {
            // todo we should fix this
            current: max,
            max,
        }
    }
}

pub struct Enemy;

use ggez::graphics;

pub struct Head {
    sprite: graphics::Image,
    sight_range: f32,
}

pub struct Body {
    sprite: graphics::Image,
    health: f32,
}

pub struct Arms {
    sprite: graphics::Image,
    damage: f32,
}

pub struct Legs {
    sprite: graphics::Image,
    speed: f32,
}

pub struct BuilderState {
    curr_choices: [Option<usize>; 4],
    possible_heads: Vec<Head>,
    possible_bodies: Vec<Body>,
    possible_arms: Vec<Arms>,
    possible_legs: Vec<Legs>,
    curr_hover: usize,
}

impl BuilderState {
    pub fn new(possible_heads: Vec<Head>, possible_bodies: Vec<Body>, possible_arms: Vec<Arms>, possible_legs: Vec<Legs>) -> Self {
        BuilderState {
            curr_choices: [None, None, None, None],
            possible_heads,
            possible_bodies,
            possible_arms,
            possible_legs,
            curr_hover: 0,
        }
    }
}

use crate::monster_build::{Head, Body, Arms, Legs};
use ggez::graphics::Image;

pub struct Monster {
    head: Head,
    body: Body,
    arms: Arms,
    legs: Legs,
}

impl Monster {
    fn new(head: Head, body: Body, arms: Arms, legs: Legs) -> Self { Self { head, body, arms, legs } }
}

pub struct Human {
    sprite_index: usize,
    tilt: f32,
    pos: mint::Point2<f32>,
    speed: f32,
    range: f32,
    hp: f32,
}

pub struct AttackState {
    human_sprites: Vec<Image>,
    monsters: Vec<Monster>,
    humans: Vec<Human>
}

impl AttackState {
    pub fn new(human_sprites: Vec<Image>, monsters: Vec<Monster>, humans: Vec<Human>) -> Self { Self { human_sprites, monsters, humans } }

    pub fn add_monster(&mut self, head: Head, body: Body, arms: Arms, legs: Legs) {
        self.monsters.push(Monster::new(head, body, arms, legs));
    }
}

// TODO: Humans vs Monsters!

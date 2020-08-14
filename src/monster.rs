use crate::monster_build::{Head, Body, Arms, Legs};
use ggez::{Context, graphics::Image};

// NOTE: I'm prob gonna assume that the game world is also just 800x600

pub struct Monster {
    head: Head,
    body: Body,
    arms: Arms,
    legs: Legs,
    pos: mint::Point2<f32>,
}

impl Monster {
    pub fn new(head: Head, body: Body, arms: Arms, legs: Legs, pos: mint::Point2<f32>) -> Self { Self { head, body, arms, legs, pos } }
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
    pub fn new(human_sprites: Vec<Image>) -> Self {
        Self { human_sprites, monsters: Vec::new(), humans: Vec::new() }
    }

    pub fn add_monster(&mut self, head: Head, body: Body, arms: Arms, legs: Legs) {
        self.monsters.push(Monster::new(head, body, arms, legs,
                                        mint::Point2 {
                                            x: (0.0),
                                            y: (64.0*self.monsters.len() as f32)
                                        })); // NOTE: ^monsters are close together^
    }

    pub fn generate_humans(&mut self, day: u16) {
        // TODO
    }

    pub fn draw(&self, ctx: &mut Context) {
        // TODO: draw humans and monsters
    }

    pub fn update_state(&mut self, ctx: &mut Context) {
        // TODO
    }
}

// TODO: Humans vs Monsters!

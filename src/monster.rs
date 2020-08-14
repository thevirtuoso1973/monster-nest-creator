use crate::monster_build::{Head, Body, Arms, Legs};

struct Monster {
    head: Head,
    body: Body,
    arms: Arms,
    legs: Legs,
}

impl Monster {
    fn new(head: Head, body: Body, arms: Arms, legs: Legs) -> Self { Self { head, body, arms, legs } }
}


pub struct AttackState {
    monsters: Vec<Monster>,
}

impl AttackState {
    pub fn new() -> Self {
        AttackState {monsters: Vec::new()}
    }

    pub fn add_monster(&mut self, head: Head, body: Body, arms: Arms, legs: Legs) {
        self.monsters.push(Monster::new(head, body, arms, legs));
    }
}

// TODO: Humans vs Monsters!

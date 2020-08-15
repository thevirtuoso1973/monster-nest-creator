use crate::monster_build::{Arms, Body, Head, Legs, Sprite};
use crate::SCREEN_SIZE;
use ggez::{
    graphics::{self, Image},
    Context, GameResult,
};
use std::f32::consts::PI;

// NOTE: I'm prob gonna assume that the game world is also just 800x600

pub struct Monster {
    head: Head,
    body: Body,
    arms: Arms,
    legs: Legs,
    pos: mint::Point2<f32>,
    hp: f32,
}

impl Monster {
    pub fn new(head: Head, body: Body, arms: Arms, legs: Legs, pos: mint::Point2<f32>) -> Self {
        let hp = body.get_health();
        Self {
            head,
            body,
            arms,
            legs,
            pos,
            hp,
        }
    }
}

pub struct Human {
    sprite_index: usize,
    tilt: f32,
    pos: mint::Point2<f32>,
    speed: f32,
    range: f32,
    hp: f32,
    damage: f32,
}

impl Human {
    pub fn new(
        sprite_index: usize,
        tilt: f32,
        pos: mint::Point2<f32>,
        speed: f32,
        range: f32,
        hp: f32,
        damage: f32,
    ) -> Self {
        Self {
            sprite_index,
            tilt,
            pos,
            speed,
            range,
            hp,
            damage,
        }
    }

    pub fn look_towards(&mut self, target_tilt: f32) {
        if target_tilt > self.tilt {
            self.tilt = target_tilt.min(self.tilt + (PI / 4.0));
        } else if target_tilt < self.tilt {
            self.tilt = target_tilt.max(self.tilt - (PI / 4.0));
        }
    }

    pub fn move_along_tilt(&mut self) {
        self.pos = mint::Point2 {
            x: self.pos.x - (self.speed * self.tilt.cos()),
            y: self.pos.y - (self.speed * self.tilt.sin()),
        }
    }
}

pub struct AttackState {
    human_sprites: Vec<Image>,
    monsters: Vec<Monster>,
    humans: Vec<Human>,
}

impl AttackState {
    pub fn new(human_sprites: Vec<Image>) -> Self {
        Self {
            human_sprites,
            monsters: Vec::new(),
            humans: Vec::new(),
        }
    }

    pub fn move_monster_right(&mut self) {
        // NOTE: for debugging
        let monster = &mut self.monsters[0];
        monster.pos = mint::Point2 {
            x: monster.pos.x + monster.legs.get_speed(),
            y: monster.pos.y,
        };
    }

    pub fn move_monster_down(&mut self) {
        // NOTE: for debugging
        let monster = &mut self.monsters[0];
        monster.pos = mint::Point2 {
            x: monster.pos.x,
            y: monster.pos.y + monster.legs.get_speed(),
        };
    }

    pub fn add_monster(&mut self, head: Head, body: Body, arms: Arms, legs: Legs) {
        self.monsters.push(Monster::new(
            head,
            body,
            arms,
            legs,
            mint::Point2 {
                x: (0.0),
                y: (64.0 * self.monsters.len() as f32),
            },
        )); // NOTE: ^monsters are close together^
    }

    pub fn generate_humans(&mut self, day: u16) {
        self.humans.clear();
        for i in 0..day {
            let new_pos = mint::Point2 {
                x: SCREEN_SIZE.0 - 32.0,
                y: SCREEN_SIZE.1 - (64.0 * (i as f32 + 1.0)),
            };
            self.humans
                .push(Human::new(0, 0.0, new_pos, 2.0, 100.0, 10.0, 10.0)); // TODO actually randomise this
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for monster in &self.monsters {
            self.draw_monster(ctx, monster, 0.5)?;
        }
        for human in &self.humans {
            self.draw_human(ctx, human)?;
        }
        Ok(())
    }

    fn draw_monster(&self, ctx: &mut Context, monster: &Monster, scaling: f32) -> GameResult {
        let body_part_side_length = monster.head.get_image().width() as f32 * scaling;
        let scale_vector = mint::Point2 {
            x: scaling,
            y: scaling,
        };

        graphics::draw(
            ctx,
            monster.head.get_image(),
            graphics::DrawParam::from((monster.pos,)).scale(scale_vector),
        )?;
        graphics::draw(
            ctx,
            monster.body.get_image(),
            graphics::DrawParam::from((mint::Point2 {
                x: monster.pos.x,
                y: monster.pos.y + body_part_side_length,
            },))
            .scale(scale_vector),
        )?;
        graphics::draw(
            ctx,
            monster.arms.get_image(),
            graphics::DrawParam::from((mint::Point2 {
                x: monster.pos.x - body_part_side_length,
                y: monster.pos.y + body_part_side_length,
            },))
            .scale(scale_vector),
        )?;
        graphics::draw(
            ctx,
            monster.arms.get_image(),
            graphics::DrawParam::from((mint::Point2 {
                x: monster.pos.x + (body_part_side_length * 2.0),
                y: monster.pos.y + body_part_side_length,
            },))
            .scale(mint::Point2 {
                x: -scale_vector.x,
                y: scale_vector.y,
            }),
        )?;
        graphics::draw(
            ctx,
            monster.legs.get_image(),
            graphics::DrawParam::from((mint::Point2 {
                x: monster.pos.x,
                y: monster.pos.y + (body_part_side_length * 2.0),
            },))
            .scale(scale_vector),
        )?;
        Ok(())
    }

    fn draw_human(&self, ctx: &mut Context, human: &Human) -> GameResult {
        let human_sprite = &self.human_sprites[human.sprite_index];

        graphics::draw(
            ctx,
            human_sprite,
            graphics::DrawParam::from((human.pos,)).rotation(human.tilt),
        )?;

        Ok(())
    }

    /// optionally returns if true if monster won, else false
    pub fn update_state(&mut self) -> Option<bool> {
        for i in 0..self.humans.len() {
            if self.monsters.is_empty() {
                break;
            }
            let (target_index, distance) = self.get_closest_monster(&self.humans[i]);
            let target = &mut self.monsters[target_index];
            let acute_tilt = get_acute_tilt(target.pos, self.humans[i].pos);
            // HACK: acute_tilt is correct, but actual_tilt is rather unintuitive:
            let actual_tilt = if target.pos.x >= self.humans[i].pos.x
                && target.pos.y <= self.humans[i].pos.y
            {
                PI / 2.0 + ((PI / 2.0) - acute_tilt) //PI+acute_tilt
            } else if target.pos.x >= self.humans[i].pos.x {
                -((PI / 2.0) + ((PI / 2.0) - acute_tilt))
            } else if target.pos.x < self.humans[i].pos.x && target.pos.y < self.humans[i].pos.y {
                acute_tilt
            } else {
                -acute_tilt //PI+(PI/2.0)+((PI/2.0)-acute_tilt)
            };

            self.humans[i].look_towards(actual_tilt); // update tilt

            if distance >= self.humans[i].range {
                // move if not in range
                self.humans[i].move_along_tilt();
            } else if (self.humans[i].tilt - actual_tilt).abs() < 0.01 {
                target.hp -= self.humans[i].damage;
                if target.hp <= 0.0 {
                    self.monsters.remove(target_index);
                }
            }
        }

        // TODO: update monsters:

        return if self.monsters.is_empty() || self.humans.is_empty() {
            Some(self.humans.is_empty())
        } else { None }
    }

    fn get_closest_monster(&self, human: &Human) -> (usize, f32) {
        let mut curr_monster_index = 0;
        let mut curr_min = f32::INFINITY;
        for (i, monster) in self.monsters.iter().enumerate() {
            let temp = get_euclid_distance(human.pos, monster.pos);
            if temp < curr_min {
                curr_min = temp;
                curr_monster_index = i;
            }
        }
        (curr_monster_index, curr_min)
    }
}

fn get_acute_tilt(point1: mint::Point2<f32>, point2: mint::Point2<f32>) -> f32 {
    ((point1.y - point2.y).abs() / (point1.x - point2.x).abs()).atan()
}

fn get_euclid_distance(point1: mint::Point2<f32>, point2: mint::Point2<f32>) -> f32 {
    (point1.x - point2.x).hypot(point1.y - point2.y)
}

use crate::monster_build::{Arms, Body, Head, Legs, Sprite};
use crate::SCREEN_SIZE;
use audio::SoundSource;
use ggez::{
    audio,
    graphics::{self, Image},
    Context, GameResult,
};
use rand::Rng;
use std::f32::consts::PI;

// NOTE: I'm prob gonna assume that the game world is also just 800x600

pub struct Monster {
    head: Head,
    body: Body,
    arms: Arms,
    legs: Legs,
    pos: mint::Point2<f32>,
    hp: f32,
    cooldown: u32,
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
            cooldown: 0,
        }
    }
}

pub struct Human {
    sprite_index: usize,
    tilt: f32,
    pos: mint::Point2<f32>,
    speed: f32,
    range: f32,
    total_hp: f32,
    hp: f32,
    damage: f32,
    // the number of frames left until attack is available (game runs at ~70fps):
    cooldown: u32,
}

impl Human {
    pub fn new(
        sprite_index: usize,
        pos: mint::Point2<f32>,
        speed: f32,
        range: f32,
        hp: f32,
        damage: f32,
    ) -> Self {
        Self {
            sprite_index,
            tilt: 0.0,
            pos,
            speed,
            range,
            total_hp: hp,
            hp,
            damage,
            cooldown: 0,
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
    gunshot_sound: audio::Source,
    hit_sounds: Vec<audio::Source>,
    thread_rng: rand::rngs::ThreadRng,
    tree_sprite_batch: graphics::spritebatch::SpriteBatch,
}

impl AttackState {
    pub fn new(
        human_sprites: Vec<Image>,
        gunshot_sound: audio::Source,
        hit_sounds: Vec<audio::Source>,
        tree_sprite: graphics::Image,
    ) -> Self {
        let tree_sprite_batch = graphics::spritebatch::SpriteBatch::new(tree_sprite.clone());
        Self {
            human_sprites,
            monsters: Vec::new(),
            humans: Vec::new(),
            gunshot_sound,
            hit_sounds,
            thread_rng: rand::thread_rng(),
            tree_sprite_batch,
        }
    }

    pub fn move_monster_left(&mut self) {
        // NOTE: for debugging
        let monster = &mut self.monsters[0];
        monster.pos = mint::Point2 {
            x: monster.pos.x - monster.legs.get_speed(),
            y: monster.pos.y,
        };
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

    pub fn move_monster_up(&mut self) {
        // NOTE: for debugging
        let monster = &mut self.monsters[0];
        monster.pos = mint::Point2 {
            x: monster.pos.x,
            y: monster.pos.y - monster.legs.get_speed(),
        };
    }

    pub fn add_monster(&mut self, head: Head, body: Body, arms: Arms, legs: Legs) {
        let new_point = mint::Point2 { x: 0.0, y: self.thread_rng.gen_range(0.0, SCREEN_SIZE.1-96.0) };
        self.monsters.push(Monster::new(
            head,
            body,
            arms,
            legs,
            new_point,
        )); // NOTE: ^monsters are close together^
    }

    pub fn generate_humans(&mut self, day: u16) {
        self.humans.clear();
        for i in 0..day {
            let rnggen = &mut self.thread_rng;
            let new_pos = mint::Point2 {
                x: SCREEN_SIZE.0 - 32.0,
                y: rnggen.gen_range(0.0, SCREEN_SIZE.1-32.0),
            };
            self.humans.push(Human::new(
                rnggen.gen_range(0, self.human_sprites.len()), // index
                new_pos,
                rnggen.gen_range(2.0, (i+2) as f32*2.0), // speed
                rnggen.gen_range(50.0, (i+2) as f32*50.0), // range
                rnggen.gen_range(20.0, (i+2) as f32*20.0), // hp
                rnggen.gen_range(10.0, (i+2) as f32*10.0), // damage
            ));
        }
    }

    pub fn generate_scenery(&mut self) {
        self.tree_sprite_batch.clear();

        let y = 0.0;
        let mut x = 0.0;
        while x < SCREEN_SIZE.0 {
            let point = mint::Point2 { x, y };
            self.tree_sprite_batch.add((point,));

            x += self.thread_rng.gen_range(50.0, 100.0);
        }

        let y = SCREEN_SIZE.1 - 32.0;
        let mut x = 0.0;
        while x < SCREEN_SIZE.0 {
            let point = mint::Point2 { x, y };
            self.tree_sprite_batch.add((point,));

            x += self.thread_rng.gen_range(50.0, 100.0);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_scenery(ctx)?;

        for monster in &self.monsters {
            self.draw_monster(ctx, monster, 0.5)?;
        }
        for human in &self.humans {
            self.draw_human(ctx, human)?;
        }
        Ok(())
    }

    /// just draws the currently saved scenery
    fn draw_scenery(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.tree_sprite_batch, graphics::DrawParam::default())?;
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

        let health_height = 10;
        let health_rect = graphics::Image::solid(ctx, health_height, graphics::Color::from_rgb(0, 255, 0))?;
        let actual_length = (32.0*(monster.hp/monster.body.get_health())) as u16;
        let x_scaling = actual_length as f32/health_height as f32; // health_height is also width
        let pos = mint::Point2 { x: monster.pos.x, y: monster.pos.y+96.0 };
        graphics::draw(ctx, &health_rect, graphics::DrawParam::from((pos,)).scale([x_scaling, 1.0]))?;

        Ok(())
    }

    fn draw_human(&self, ctx: &mut Context, human: &Human) -> GameResult {
        let human_sprite = &self.human_sprites[human.sprite_index];
        let mut params = graphics::DrawParam::from((human.pos,)).rotation(human.tilt);
        if human.tilt.abs() > PI / 2.0 {
            params = params.scale([1.0, -1.0])
        }
        if human.hp < human.total_hp*0.75 { // change color of human depending on health
            let new_red = (255.0*(human.hp/human.total_hp)) as u8;
            params = params.color(graphics::Color::from_rgb(new_red, 20, 20));
        }

        graphics::draw(ctx, human_sprite, params)?;

        Ok(())
    }

    /// optionally returns if true if monster won, else false
    pub fn update_state(&mut self) -> Option<bool> {
        self.update_humans();
        self.update_monsters();

        return if self.monsters.is_empty() || self.humans.is_empty() {
            Some(self.humans.is_empty())
        } else {
            None
        };
    }

    fn update_humans(&mut self) {
        for i in 0..self.humans.len() {
            if self.monsters.is_empty() {
                break;
            }
            let (target_index, distance) = self.get_closest_monster(&self.humans[i].pos);
            let target = &mut self.monsters[target_index];
            let acute_tilt = get_acute_tilt(&target.pos, &self.humans[i].pos);
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
            } else if (self.humans[i].tilt - actual_tilt).abs() < 0.01
                && self.humans[i].cooldown == 0
            {
                if let Err(error) = self.gunshot_sound.play() {
                    // play gunshot
                    eprintln!("{}", error);
                }
                target.hp -= self.humans[i].damage;
                if target.hp <= 0.0 {
                    self.monsters.remove(target_index);
                }
                self.humans[i].cooldown = 75;
            }
            if self.humans[i].cooldown > 0 {
                self.humans[i].cooldown -= 1;
            }
        }
    }

    fn update_monsters(&mut self) {
        for i in 0..self.monsters.len() {
            if self.humans.is_empty() {
                break;
            }
            // center_point assumes half scaling
            let monster_center_point = mint::Point2 {
                x: self.monsters[i].pos.x + 16.0,
                y: self.monsters[i].pos.y + 48.0,
            };
            let (possible_target_index, distance) = self.get_closest_human(&monster_center_point);
            let possible_target = &mut self.humans[possible_target_index];
            let sight_range = self.monsters[i].head.get_sight_range();
            if distance <= sight_range / 2.0 {
                // NOTE: attack range is 1/4 of sight
                if self.monsters[i].cooldown == 0 {
                    let choice = self.thread_rng.gen_range(0, self.hit_sounds.len());
                    if let Err(error) = self.hit_sounds[choice].play() {
                        // play gunshot
                        eprintln!("{}", error);
                    }

                    possible_target.hp -= self.monsters[i].arms.get_damage();
                    if possible_target.hp <= 0.0 {
                        self.humans.remove(possible_target_index);
                    }
                    self.monsters[i].cooldown = 40;
                }
            } else if distance <= sight_range {
                let acute_angle = get_acute_tilt(&possible_target.pos, &self.monsters[i].pos);
                let (curr_x, curr_y) = (self.monsters[i].pos.x, self.monsters[i].pos.y);
                let speed = self.monsters[i].legs.get_speed();

                self.monsters[i].pos = mint::Point2 {
                    x: curr_x
                        + if curr_x <= possible_target.pos.x {
                            speed * acute_angle.cos()
                        } else {
                            -speed * acute_angle.cos()
                        },
                    y: curr_y
                        + if curr_y <= possible_target.pos.y {
                            speed * acute_angle.sin()
                        } else {
                            -speed * acute_angle.sin()
                        },
                };
            } else {
                let choice: i8 = self.thread_rng.gen_range(0, 4);
                let speed = self.monsters[i].legs.get_speed() / 4.0;
                let new_pos = match choice {
                    0 => mint::Point2 {
                        x: self.monsters[i].pos.x,
                        y: 0.0f32.max(self.monsters[i].pos.y - speed),
                    },
                    1 => mint::Point2 {
                        x: SCREEN_SIZE.0.min(self.monsters[i].pos.x + speed),
                        y: self.monsters[i].pos.y,
                    },
                    2 => mint::Point2 {
                        x: self.monsters[i].pos.x,
                        y: SCREEN_SIZE.1.min(self.monsters[i].pos.y + speed),
                    },
                    3 => mint::Point2 {
                        x: 0.0f32.max(self.monsters[i].pos.x - speed),
                        y: self.monsters[i].pos.y,
                    },
                    _ => panic!(),
                };
                self.monsters[i].pos = new_pos;
            }

            if self.monsters[i].cooldown > 0 {
                self.monsters[i].cooldown -= 1;
            }
        }
    }

    fn get_closest_monster(&self, human_pos: &mint::Point2<f32>) -> (usize, f32) {
        let mut curr_monster_index = 0;
        let mut curr_min = f32::INFINITY;
        for (i, monster) in self.monsters.iter().enumerate() {
            let temp = get_euclid_distance(human_pos, &monster.pos);
            if temp < curr_min {
                curr_min = temp;
                curr_monster_index = i;
            }
        }
        (curr_monster_index, curr_min)
    }

    fn get_closest_human(&self, monster_pos: &mint::Point2<f32>) -> (usize, f32) {
        let mut curr_human_index = 0;
        let mut curr_min = f32::INFINITY;
        for (i, human) in self.humans.iter().enumerate() {
            let temp = get_euclid_distance(monster_pos, &human.pos);
            if temp < curr_min {
                curr_min = temp;
                curr_human_index = i;
            }
        }
        (curr_human_index, curr_min)
    }
}

fn get_acute_tilt(point1: &mint::Point2<f32>, point2: &mint::Point2<f32>) -> f32 {
    ((point1.y - point2.y).abs() / (point1.x - point2.x).abs()).atan()
}

fn get_euclid_distance(point1: &mint::Point2<f32>, point2: &mint::Point2<f32>) -> f32 {
    (point1.x - point2.x).hypot(point1.y - point2.y)
}

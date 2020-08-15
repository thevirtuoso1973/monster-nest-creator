use crate::monster_build::{Sprite, Head, Body, Arms, Legs};
use crate::SCREEN_SIZE;
use ggez::{Context, graphics::{self, Image}, GameResult};

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

impl Human {
    pub fn new(sprite_index: usize, tilt: f32, pos: mint::Point2<f32>, speed: f32, range: f32, hp: f32) -> Self { Self { sprite_index, tilt, pos, speed, range, hp } }
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
        self.humans.clear();
        for i in 0..day {
            let new_pos = mint::Point2 { x: SCREEN_SIZE.0-32.0, y: SCREEN_SIZE.1-(64.0*(i as f32+1.0)) };
            self.humans.push(Human::new(0, 0.0, new_pos, 10.0, 10.0, 10.0));
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

    fn draw_monster(&self, ctx: &mut Context, monster: &Monster, scaling: f32)
                        -> GameResult {
        let body_part_side_length = monster.head.get_image().width() as f32*scaling;
        let scale_vector = mint::Point2 { x: scaling, y: scaling };

        graphics::draw(ctx, monster.head.get_image(),
                       graphics::DrawParam::from((monster.pos,)).scale(scale_vector))?;
        graphics::draw(ctx, monster.body.get_image(),
                       graphics::DrawParam::from((mint::Point2{
                           x: monster.pos.x, y: monster.pos.y+body_part_side_length
                       },)).scale(scale_vector))?;
        graphics::draw(ctx, monster.arms.get_image(),
                       graphics::DrawParam::from((mint::Point2{
                           x: monster.pos.x-body_part_side_length, y: monster.pos.y+body_part_side_length
                       },)).scale(scale_vector))?;
        graphics::draw(ctx, monster.arms.get_image(),
                       graphics::DrawParam::from((mint::Point2{
                           x: monster.pos.x+(body_part_side_length*2.0), y: monster.pos.y+body_part_side_length
                       },)).scale(mint::Point2 { x: -scale_vector.x, y: scale_vector.y  }))?;
        graphics::draw(ctx, monster.legs.get_image(),
                       graphics::DrawParam::from((mint::Point2{
                           x: monster.pos.x, y: monster.pos.y+(body_part_side_length*2.0)
                       },)).scale(scale_vector))?;
        Ok(())
    }

    fn draw_human(&self, ctx: &mut Context, human: &Human) -> GameResult {
        let human_sprite = &self.human_sprites[human.sprite_index];

        graphics::draw(ctx, human_sprite, graphics::DrawParam::from((human.pos,)).rotation(human.tilt))?;

        Ok(())
    }

    pub fn update_state(&mut self, ctx: &mut Context) {
        // TODO: Update monsters & update humans
    }
}

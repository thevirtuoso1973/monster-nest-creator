use ggez::graphics;
use ggez::{Context, GameResult};
use crate::SCREEN_SIZE;

trait Sprite {
    fn get_image(&self) -> &graphics::Image;
}

pub struct Head {
    sprite: graphics::Image,
    sight_range: f32,
}

impl Head {
    pub fn new(sprite: graphics::Image, sight_range: f32) -> Self { Self { sprite, sight_range } }
}

impl Sprite for Head {
    fn get_image(&self) -> &graphics::Image {
        &self.sprite
    }
}

pub struct Body {
    sprite: graphics::Image,
    health: f32,
}

impl Body {
    pub fn new(sprite: graphics::Image, health: f32) -> Self { Self { sprite, health } }
}

impl Sprite for Body {
    fn get_image(&self) -> &graphics::Image {
        &self.sprite
    }
}

pub struct Arms {
    sprite: graphics::Image,
    damage: f32,
}

impl Arms {
    pub fn new(sprite: graphics::Image, damage: f32) -> Self { Self { sprite, damage } }
}

impl Sprite for Arms {
    fn get_image(&self) -> &graphics::Image {
        &self.sprite
    }
}

pub struct Legs {
    sprite: graphics::Image,
    speed: f32,
}

impl Legs {
    pub fn new(sprite: graphics::Image, speed: f32) -> Self { Self { sprite, speed } }
}

impl Sprite for Legs {
    fn get_image(&self) -> &graphics::Image {
        &self.sprite
    }
}

pub struct BuilderState {
    curr_choices: [Option<usize>; 4], // choice for head, body, arms, and legs
    possible_heads: Vec<Head>,
    possible_bodies: Vec<Body>,
    possible_arms: Vec<Arms>,
    possible_legs: Vec<Legs>,
    curr_hover: usize, // index of option to highlight
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

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        if self.curr_choices[0].is_none() {
            self.draw_options(&self.possible_heads, ctx)?;
        } else if self.curr_choices[1].is_none() {
            self.draw_options(&self.possible_bodies, ctx)?;
        } else if self.curr_choices[2].is_none() {
            self.draw_options(&self.possible_arms, ctx)?;
        } else {
            self.draw_options(&self.possible_legs, ctx)?;
        }

        // TODO: draw the curr monster
        Ok(())
    }

    fn draw_options<T: Sprite>(&self, parts: &Vec<T>, ctx: &mut Context) -> GameResult {
        // TODO: test moving the highlight
        for (i, part) in parts.iter().enumerate() {
            let img = part.get_image();
            let new_point = mint::Point2 { x: i as f32*64.0, y: SCREEN_SIZE.1-64.0 };
            graphics::draw(ctx, img, (new_point,))?;
        }

        let outline_rect = graphics::Rect::new(self.curr_hover as f32*64.0, SCREEN_SIZE.1-64.0, 64.0, 64.0);
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(2.0), outline_rect, graphics::BLACK)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        Ok(())
    }

    pub fn move_option(&mut self, next: bool) {
        if self.curr_choices[0].is_none() {
            if next && self.curr_hover < self.possible_heads.len()-1 {
                self.curr_hover += 1
            } else if !next && self.curr_hover > 0 {
                self.curr_hover += 1
            }
        } else if self.curr_choices[1].is_none() {
            if self.curr_hover < self.possible_bodies.len()-1 {
                self.curr_hover += 1
            } else if !next && self.curr_hover > 0 {
                self.curr_hover += 1
            }
        } else if self.curr_choices[2].is_none() {
            if self.curr_hover < self.possible_arms.len()-1 {
                self.curr_hover += 1
            } else if !next && self.curr_hover > 0 {
                self.curr_hover += 1
            }
        } else {
            if self.curr_hover < self.possible_legs.len()-1 {
                self.curr_hover += 1
            } else if !next && self.curr_hover > 0 {
                self.curr_hover += 1
            }
        }
    }

    pub fn choose_current(&mut self) {
        if self.curr_choices[0].is_none() {
            self.curr_choices[0] = Some(self.curr_hover);
        } else if self.curr_choices[1].is_none() {
            self.curr_choices[1] = Some(self.curr_hover);
        } else if self.curr_choices[2].is_none() {
            self.curr_choices[2] = Some(self.curr_hover);
        } else {
            self.curr_choices[3] = Some(self.curr_hover);
        }
    }

    pub fn is_fully_selected(&self) -> bool {
        return self.curr_choices[3].is_some();
    }
}

use crate::monster_build::{Arms, Body, Head, Legs};
use ggez::{graphics, Context};

pub fn get_heads(ctx: &mut Context) -> Vec<Head> {
    let head1 = graphics::Image::new(ctx, "/sprites/googly-eyes.png").unwrap();
    let head2 = graphics::Image::new(ctx, "/sprites/longeyes.png").unwrap();

    vec![Head::new(head1, 100.0), Head::new(head2, 150.0)]
}

pub fn get_bodies(ctx: &mut Context) -> Vec<Body> {
    let body1 = graphics::Image::new(ctx, "/sprites/round-body.png").unwrap();

    vec![Body::new(body1, 100.0)]
}

pub fn get_arms(ctx: &mut Context) -> Vec<Arms> {
    let arms1 = graphics::Image::new(ctx, "/sprites/small-arms.png").unwrap();

    vec![Arms::new(arms1, 10.0)]
}

pub fn get_legs(ctx: &mut Context) -> Vec<Legs> {
    let legs1 = graphics::Image::new(ctx, "/sprites/blob-legs.png").unwrap();

    vec![Legs::new(legs1, 10.0)]
}

pub fn get_human_sprites(ctx: &mut Context) -> Vec<graphics::Image> {
    let human1 = graphics::Image::new(ctx, "/sprites/gun-human.png").unwrap();

    vec![human1]
}

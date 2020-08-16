use crate::monster_build::{Arms, Body, Head, Legs};
use ggez::{graphics, Context};

pub fn get_heads(ctx: &mut Context) -> Vec<Head> {
    let head1 = graphics::Image::new(ctx, "/sprites/googly-eyes.png").unwrap();
    let head2 = graphics::Image::new(ctx, "/sprites/longeyes.png").unwrap();
    let head3 = graphics::Image::new(ctx, "/sprites/thanos-head.png").unwrap();

    vec![Head::new(head1, 130.0), Head::new(head2, 180.0), Head::new(head3, 105.0)]
}

pub fn get_bodies(ctx: &mut Context) -> Vec<Body> {
    let body1 = graphics::Image::new(ctx, "/sprites/round-body.png").unwrap();
    let body2 = graphics::Image::new(ctx, "/sprites/muscle-body.png").unwrap();
    let body3 = graphics::Image::new(ctx, "/sprites/gingerbread-body.png").unwrap();

    vec![Body::new(body1, 100.0), Body::new(body2, 80.0), Body::new(body3, 60.0)]
}

pub fn get_arms(ctx: &mut Context) -> Vec<Arms> {
    let arms1 = graphics::Image::new(ctx, "/sprites/small-arms.png").unwrap();
    let arms2 = graphics::Image::new(ctx, "/sprites/muscle-arms.png").unwrap();
    let arms3 = graphics::Image::new(ctx, "/sprites/sharp-arms.png").unwrap();

    vec![Arms::new(arms1, 5.0), Arms::new(arms2, 15.0), Arms::new(arms3, 20.0)]
}

pub fn get_legs(ctx: &mut Context) -> Vec<Legs> {
    let legs1 = graphics::Image::new(ctx, "/sprites/blob-legs.png").unwrap();
    let legs2 = graphics::Image::new(ctx, "/sprites/muscle-legs.png").unwrap();

    vec![Legs::new(legs1, 5.0), Legs::new(legs2, 10.0)]
}

pub fn get_human_sprites(ctx: &mut Context) -> Vec<graphics::Image> {
    let human1 = graphics::Image::new(ctx, "/sprites/gun-human.png").unwrap();
    let human2 = graphics::Image::new(ctx, "/sprites/gun-human2.png").unwrap();

    vec![human1, human2]
}

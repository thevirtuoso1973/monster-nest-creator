use ggez;

use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use monster_nest_creator::monster_build::{ Head, Body, Arms, Legs, BuilderState };
use monster_nest_creator::SCREEN_SIZE;

enum ScreenState {
    MainMenu,
    MonsterCreation, // player building their monster
    NightAttack,     // humans attack the 'nest'
}

// contains the game's state
struct MainState {
    frames: usize,
    state: ScreenState,
    font: graphics::Font,
    title: graphics::Text,
    title_img: graphics::Image,
    builder_state: BuilderState,
    day: u16,
}

fn get_heads(ctx: &mut Context) -> Vec<Head> {
    let head1 = graphics::Image::new(ctx, "/sprites/googly-eyes.png").unwrap();

    return vec![Head::new(head1, 100.0)];
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/Roboto/Roboto-Regular.ttf")?; // REVIEW: replace with scary font?
        let title = graphics::Text::new(graphics::TextFragment {
            text: "Monster Nest Creator".to_string(),
            color: Some(graphics::BLACK),
            font: Some(font),
            scale: Some(graphics::Scale { x: 40.0, y: 40.0 }),
        });
        let main_img = graphics::Image::new(ctx, "/sprites/googly-eyes.png")?;

        let s = MainState {
            frames: 0,
            state: ScreenState::MainMenu,
            font,
            title,
            title_img: main_img,
            builder_state: BuilderState::new(get_heads(ctx), Vec::new(), Vec::new(), Vec::new()), // TODO: add the possible body parts
            day: 1,
        };
        Ok(s)
    }


    fn switch_state(&mut self, new_state: ScreenState) {
        self.state = new_state;
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        // Drawables are drawn from their top-left corner.
        match self.state {
            ScreenState::MainMenu => {
                let title_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 140.0),
                    y: (10.0),
                };
                graphics::draw(ctx, &self.title, (title_dest_point,))?;

                let scale_vec = [2.0, 2.0];
                let img_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 - (self.title_img.width() as f32*scale_vec[0])),
                    y: (SCREEN_SIZE.1 - self.title_img.height() as f32*scale_vec[1]),
                };
                graphics::draw(ctx, &self.title_img, graphics::DrawParam::from((img_dest_point,)).scale(scale_vec))?;
            }
            ScreenState::MonsterCreation => {
                let day_dest_point = mint::Point2 { x: (1.0), y: (1.0) };
                graphics::draw(
                    ctx,
                    &graphics::Text::new(graphics::TextFragment {
                        text: format!("Day {}", self.day),
                        color: Some(graphics::BLACK),
                        font: Some(self.font),
                        scale: Some(graphics::Scale { x: 40.0, y: 40.0 }),
                    }),
                    (day_dest_point,),
                )?;
                self.builder_state.draw(ctx)?;
            }
            ScreenState::NightAttack => {}
        }
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match self.state {
            ScreenState::MainMenu => match keycode {
                KeyCode::Return => self.switch_state(ScreenState::MonsterCreation),
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::MonsterCreation => match keycode {
                KeyCode::Right => self.builder_state.move_option(true),
                KeyCode::Left => self.builder_state.move_option(false),
                KeyCode::Return => {
                    self.builder_state.choose_current();
                    if self.builder_state.is_fully_selected() {
                        self.switch_state(ScreenState::NightAttack);
                    }
                }
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::NightAttack => match keycode {
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            }
        }
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("monster_nest_creator", "Chris")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Monster Nest Creator"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}

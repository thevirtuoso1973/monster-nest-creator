use ggez;

use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{Context, GameResult};
use monster_nest_creator::monster::AttackState;
use monster_nest_creator::monster_build::{Arms, Body, BuilderState, Head, Legs};
use monster_nest_creator::SCREEN_SIZE;
use monster_nest_creator::sprite_loader::*;
use std::env;
use std::path;

enum ScreenState {
    MainMenu,
    MonsterCreation, // player building their monster
    NightAttack,     // humans attack the 'nest'
    EndGame,
}

// contains the game's state
struct MainState {
    frames: usize,
    state: ScreenState,
    font: graphics::Font,
    title: graphics::Text,
    title_img: graphics::Image,
    title_text: graphics::Text,
    builder_state: BuilderState,
    attack_state: AttackState,
    day: u16,
    won: bool,
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
        let title_text = graphics::Text::new(graphics::TextFragment {
            text: format!(
                "{}{}{}",
                "Create your monsters in the day, but beware,\n",
                "humans will attack your nest in the night!\n\n",
                "Press enter to continue..."
            ),
            color: Some(graphics::BLACK),
            font: Some(font),
            scale: Some(graphics::Scale { x: 20.0, y: 20.0 }),
        });

        let s = MainState {
            frames: 0,
            state: ScreenState::MainMenu,
            font,
            title,
            title_img: main_img,
            title_text,
            builder_state: BuilderState::new(
                get_heads(ctx),
                get_bodies(ctx),
                get_arms(ctx),
                get_legs(ctx),
            ),
            attack_state: AttackState::new(get_human_sprites(ctx)),
            day: 1,
            won: false,
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
        match self.state {
            ScreenState::NightAttack => {
                if let Some(check_win) = self.attack_state.update_state() {
                    if check_win && self.day == 2 {
                        self.won = true;
                        self.switch_state(ScreenState::EndGame);
                    } else if !check_win {
                        self.switch_state(ScreenState::EndGame);
                    } else { // move on to next day
                        self.day += 1;
                        self.switch_state(ScreenState::MonsterCreation);
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Drawables are drawn from their top-left corner.
        match self.state {
            ScreenState::MainMenu => {
                graphics::clear(ctx, graphics::WHITE);

                let title_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 140.0),
                    y: (10.0),
                };
                graphics::draw(ctx, &self.title, (title_dest_point,))?;

                let title_text_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 140.0),
                    y: (SCREEN_SIZE.1 / 2.0),
                };
                graphics::draw(ctx, &self.title_text, (title_text_dest_point,))?;

                let scale_vec = [2.0, 2.0];
                let img_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 - (self.title_img.width() as f32 * scale_vec[0])),
                    y: (SCREEN_SIZE.1 - self.title_img.height() as f32 * scale_vec[1]),
                };
                graphics::draw(
                    ctx,
                    &self.title_img,
                    graphics::DrawParam::from((img_dest_point,)).scale(scale_vec),
                )?;
            }
            ScreenState::MonsterCreation => {
                graphics::clear(ctx, graphics::WHITE);

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
            ScreenState::NightAttack => {
                graphics::clear(ctx, graphics::Color::from_rgb(166, 166, 166));

                self.attack_state.draw(ctx)?;
            }
            ScreenState::EndGame => {
                graphics::clear(ctx, graphics::WHITE);
                let end_text = if self.won {
                    graphics::Text::new(graphics::TextFragment {
                        text: "You won!".to_string(),
                        color: Some(graphics::Color::from_rgb(0, 255, 0)),
                        font: Some(self.font),
                        scale: Some(graphics::Scale { x: 20.0, y: 20.0 }),
                    })
                } else {
                    graphics::Text::new(graphics::TextFragment {
                        text: "All your monsters died!".to_string(),
                        color: Some(graphics::Color::from_rgb(255, 0, 0)),
                        font: Some(self.font),
                        scale: Some(graphics::Scale { x: 20.0, y: 20.0 }),
                    })
                };
                let title_text_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 140.0),
                    y: (SCREEN_SIZE.1 / 2.0),
                };
                graphics::draw(ctx, &end_text, (title_text_dest_point,))?;
            }
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
                    self.builder_state.choose_current_and_reset();
                    if self.builder_state.is_fully_selected() {
                        let (head, body, arms, legs) = self.builder_state.get_built_monster();
                        self.attack_state.add_monster(head, body, arms, legs);

                        self.attack_state.generate_humans(self.day);
                        self.switch_state(ScreenState::NightAttack);
                    }
                }
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::NightAttack => match keycode {
                KeyCode::Down => self.attack_state.move_monster_down(),
                KeyCode::Right => self.attack_state.move_monster_right(),
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::EndGame => match keycode {
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
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

use ggez;

use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{audio, Context, GameResult};
use monster_nest_creator::monster::AttackState;
use monster_nest_creator::monster_build::BuilderState;
use monster_nest_creator::sprite_loader::*;
use monster_nest_creator::SCREEN_SIZE;
use std::env;
use std::path;
use audio::SoundSource;

enum ScreenState { // TODO: make stuff look better (somehow)
    MainMenu,
    MonsterCreation, // player building their monster
    NightAttack,     // humans attack the 'nest'
    EndGame,
}

// contains the game's state
struct MainState {
    frames_modulo: usize,
    state: ScreenState,
    font: graphics::Font,
    title: graphics::Text,
    title_img: graphics::Image,
    title_text: graphics::Text,
    transition_sound: audio::Source,
    builder_state: BuilderState,
    attack_state: AttackState,
    day: u16,
    won: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/fonts/Alata-Regular.ttf")?; // REVIEW: replace with scary font?
        let title = graphics::Text::new(graphics::TextFragment {
            text: "Monster Nest".to_string(),
            color: Some(graphics::BLACK),
            font: Some(font),
            scale: Some(graphics::Scale { x: 40.0, y: 40.0 }),
        });
        let main_img = graphics::Image::new(ctx, "/sprites/googly-eyes.png")?;
        let title_text = graphics::Text::new(graphics::TextFragment {
            text: format!(
                "{}{}{}{}",
                "Create your monsters in the day, but beware,\n",
                "humans will attack your nest in the night!\n\n",
                "Press enter to continue...\n\n",
                "(Note: there is audio in this game)"
            ),
            color: Some(graphics::BLACK),
            font: Some(font),
            scale: Some(graphics::Scale { x: 20.0, y: 20.0 }),
        });

        let transition_sound = audio::Source::new(ctx, "/sounds/You_Won.mp3")?;
        let gunshot_sound = audio::Source::new(ctx, "/sounds/9_mm_gunshot.mp3")?;
        let hit_sounds = vec![
            audio::Source::new(ctx, "/sounds/hit01.mp3.flac")?,
            audio::Source::new(ctx, "/sounds/hit02.mp3.flac")?,
            audio::Source::new(ctx, "/sounds/hit03.mp3.flac")?,
        ];

        let tree_sprite = graphics::Image::new(ctx, "/sprites/tree.png")?;

        let s = MainState {
            frames_modulo: 0,
            state: ScreenState::MainMenu,
            font,
            title,
            title_img: main_img,
            title_text,
            transition_sound,
            builder_state: BuilderState::new(
                get_heads(ctx),
                get_bodies(ctx),
                get_arms(ctx),
                get_legs(ctx),
            ),
            attack_state: AttackState::new(
                get_human_sprites(ctx),
                gunshot_sound,
                hit_sounds,
                tree_sprite,
            ),
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            ScreenState::NightAttack => {
                if let Some(check_win) = self.attack_state.update_state(ctx) {
                    if check_win && self.day == 5 {
                        self.won = true;
                        if let Err(error) = self.transition_sound.play() {
                            eprintln!("{}", error);
                        }
                        self.switch_state(ScreenState::EndGame);
                    } else if !check_win {
                        self.switch_state(ScreenState::EndGame);
                    } else {
                        // move on to next day
                        self.day += 1;
                        if let Err(error) = self.transition_sound.play() {
                            eprintln!("{}", error);
                        }
                        self.attack_state.reset_monster_pos();

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
                graphics::clear(ctx, graphics::Color::from_rgb(236, 198, 198));

                let title_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 120.0),
                    y: (10.0),
                };
                graphics::draw(ctx, &self.title, (title_dest_point,))?;

                let title_text_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 170.0),
                    y: (SCREEN_SIZE.1 / 2.0 - 20.0),
                };
                graphics::draw(ctx, &self.title_text, (title_text_dest_point,))?;

                let scale_vec = [2.0, 2.0];
                let img_dest_point = mint::Point2 {
                    x: self.frames_modulo as f32,
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

                let day_dest_point = mint::Point2 { x: (10.0), y: (10.0) };
                graphics::draw(
                    ctx,
                    &graphics::Text::new(graphics::TextFragment {
                        text: format!("Day {}\n{}", self.day, if self.day > 1 { "You survived last night." } else { "" }),
                        color: Some(graphics::BLACK),
                        font: Some(self.font),
                        scale: Some(graphics::Scale { x: 40.0, y: 40.0 }),
                    }),
                    (day_dest_point,),
                )?;
                self.builder_state.draw(ctx, self.font)?;
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
                        scale: Some(graphics::Scale { x: 35.0, y: 35.0 }),
                    })
                } else {
                    graphics::Text::new(graphics::TextFragment {
                        text: format!(
                            "All your monsters died!\nYou survived until day {}.\n\nPress enter to restart.",
                            self.day
                        ),
                        color: Some(graphics::Color::from_rgb(255, 0, 0)),
                        font: Some(self.font),
                        scale: Some(graphics::Scale { x: 35.0, y: 35.0 }),
                    })
                };
                let title_text_dest_point = mint::Point2 {
                    x: (SCREEN_SIZE.0 / 2.0 - 140.0),
                    y: (SCREEN_SIZE.1 / 2.0 - 20.0),
                };
                graphics::draw(ctx, &end_text, (title_text_dest_point,))?;
            }
        }
        graphics::present(ctx)?;

        self.frames_modulo = (self.frames_modulo + 1) % SCREEN_SIZE.0 as usize;
        if (self.frames_modulo % 100) == 0 {
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
                KeyCode::Return => {
                    self.switch_state(ScreenState::MonsterCreation);
                },
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
                        self.builder_state.empty_choices();
                        self.attack_state.add_monster(head, body, arms, legs);

                        self.attack_state.generate_humans(self.day);
                        self.attack_state.generate_scenery();
                        self.switch_state(ScreenState::NightAttack);
                    }
                }
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::NightAttack => match keycode {
                // KeyCode::Down => self.attack_state.move_monster_down(),
                // KeyCode::Up => self.attack_state.move_monster_up(),
                // KeyCode::Left => self.attack_state.move_monster_left(),
                // KeyCode::Right => self.attack_state.move_monster_right(),
                KeyCode::Escape => event::quit(ctx),
                _ => (),
            },
            ScreenState::EndGame => match keycode {
                KeyCode::Return => {
                    self.switch_state(ScreenState::MainMenu);
                    self.day = 1;
                },
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
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Monster Nest")
                .vsync(true),
        )
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}

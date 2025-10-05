use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use quad_snd::{AudioContext, Playback, Sound};
use rand::Rng;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Padding, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use std::{env, fs, path::Path};

macro_rules! file_path {
    () => {
        format!("{}{}", env::current_dir().expect("").display(), "\\audio")
    };
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let _ = App::default().run(terminal);
    ratatui::restore();
    Ok(())
}

#[derive(Default)]
struct App {
    inside_player: bool,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        println!("(very) basic MP3 player...");

        let mut pos = 0;
        let mut vol = 5;
        let mut rng = rand::rng();
        let mut locker = false;

        if !Path::new("audio").exists() {
            eprintln!("\naudio folder does not exists !");
            std::process::exit(255);
        }

        let pth = file_path!();
        let paths: Vec<String> = std::fs::read_dir(pth)
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| format!("{}", e.path().display()))
            .collect();

        if paths.len() == 0 {
            eprintln!("\naudio folder is empty...");
            std::process::exit(255);
        }

        let mut _box: Vec<usize> = vec![];
        for i in 0..paths.len() {
            _box.push(i);
        }

        let ctx = AudioContext::new();

        //    audible(&ctx, &paths[pos]);

        loop {
            let line_01 = if !locker {
                if pos == 0 {
                    &paths[paths.len() - 1]
                } else {
                    &paths[pos - 1]
                }
            } else {
                ""
            };

            let line_02 = if !locker {
                &paths[pos]
            } else {
                "NOW, RANDOM until the EOF..."
            };

            let line_03 = if !locker {
                if pos == paths.len() - 1 {
                    &paths[0]
                } else {
                    &paths[pos + 1]
                }
            } else {
                ""
            };

            terminal.draw(|frame| self.draw(frame, line_01, line_02, line_03, &"x".repeat(vol)))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('r') => {
                            if self.inside_player && locker == false {
                                audible(&ctx, &paths[pos], vol);
                            }
                        }

                        KeyCode::Right => {
                            if self.inside_player {
                                pos += 1;
                                if pos == paths.len() {
                                    pos = 0;
                                }
                            }
                        }

                        KeyCode::Up => {
                            if self.inside_player {
                                vol += 1;
                                if vol > 10 {
                                    vol = 10;
                                }
                                //  println!("Volume+ : {}", vol as f32 / 10.0);
                            }
                        }

                        KeyCode::Down => {
                            if self.inside_player {
                                if vol > 0 {
                                    vol -= 1;
                                } else {
                                    vol = 0;
                                }
                                //  println!("Volume- : {}", vol as f32 / 10.0);
                            }
                        }

                        KeyCode::Left => {
                            if self.inside_player {
                                if pos > 0 {
                                    pos -= 1;
                                } else {
                                    pos = paths.len() - 1;
                                }
                            }
                        }

                        KeyCode::Char('q') => return Ok(()),

                        KeyCode::Char('t') => {
                            self.inside_player = !self.inside_player;
                        }
                        KeyCode::Char('m') => {
                            if self.inside_player {
                                if _box.len() == 0 {
                                    ratatui::restore();

                                    std::process::exit(0);
                                }
                                pos = rng.random_range(0.._box.len());
                                audible(&ctx, &paths[_box[pos]], vol);

                                //  println!("rando(M):    {}", &paths[_box[pos]]);

                                _box.remove(pos);
                                locker = true;
                            }                         }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame, line_1: &str, line_2: &str, line_3: &str, level: &str) {
        let area = frame.area();

        let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [instructions, content] = vertical.areas(area);

        let text = if self.inside_player {
            "basic MP3 player ..."
        } else {
            "@2025 by Rubino Marc"
        };
        let paragraph = Paragraph::new(text.slow_blink())
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, instructions);

        let block = Block::bordered()
            .title(" DICTATION / mp3 player ")
            .on_green();
        frame.render_widget(block, content);

        if self.inside_player {
            let block = Block::bordered().title(" REPEAT ").on_red();
            let area = sheet_surface(area, 90, 15);

            let paragraph = Paragraph::new(vec![
                Line::from(""),
                Line::from(line_1).black(),
                Line::from(line_2).on_dark_gray(),
                Line::from(line_3).black(),
                Line::from(""),
                Line::from(format!("VOLUME: {}", level)).green(),
            ])
            .centered();

            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(&block, area);

            frame.render_widget(paragraph, area);
        } else {
            let block = Block::bordered()
                .padding(Padding::horizontal(2))
                .title("~ ~ ~ USAGE ~ ~ ~")
                .on_blue();
            let area = sheet_surface(area, 80, 50);

            let paragraph = Paragraph::new(vec![
                Line::from(""),
                Line::from("01 - First, copy your MP3 files into an 'AUDIO' folder").black(),

                Line::from("(You don't need to give them a special name, the files just need to be in mp3 format)").black(),

                Line::from(r"/!\ Be warned: I won't check if there's another filetype in the folder; crash guaranteed if you ignore this warning /!\").on_red(),
                Line::from("").black(),
                Line::from("02 - & now the KEYS:").black(),
                Line::from("").black(),
                Line::from("    ←                             previous").white(),
                Line::from("    →                                 next").white(),
                Line::from("    ↑                             volume +").white(),
                Line::from("    ↓                             volume -").white(),
                Line::from("    r                               repeat").white(),
                Line::from("    m                   pick a random file").white(),
                Line::from(                                          "").white(),
                Line::from("    t       toggle about <-> player screen").white(),
                Line::from("    q                                 quit").white(),

                Line::from("").black(),
                Line::from("03 - Note: even if a new file is reading, the playback of the previous file won't stop until its end... That's a TODO to wait for in a (soon) new release, we hope!").green(),
                Line::from("").black(),
                Line::from("").black(),
                Line::from("").black(),
                Line::from("").black(),
                Line::from("ENJOY !").yellow(),

            ])
            .centered();

            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(&block, area);

            frame.render_widget(paragraph, area);
        }
    }
}

fn sheet_surface(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn audible(ctx: &AudioContext, file: &str, volume: usize) {
    let result = fs::read(file);
    let bytes = match result {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read audio file: {}", e);
            return;
        }
    };
    let data: &[u8] = &bytes;
    let sound_mp3 = Sound::load(&ctx, data); // HERE THE MAGIC HAPPENS, NO MORE include_bytes!("music_0001.mp3")

    let playback = sound_mp3.play(&ctx, Default::default());

    Playback::set_volume(&playback, ctx, volume as f32 / 10.0);
}

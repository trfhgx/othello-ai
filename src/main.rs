use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2, epaint::StrokeKind};

mod board;
mod game;
mod ai;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use board::Cell;
use game::{Game, GameState, Player};

fn play_sound(file: &str) {
    let path = format!("src/soundtrack/{}", file);
    std::thread::spawn(move || {
        if let Ok((_stream, handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&handle) {
                if let Ok(file) = File::open(&path) {
                    if let Ok(source) = Decoder::new(BufReader::new(file)) {
                        sink.append(source);
                        sink.sleep_until_end();
                    }
                }
            }
        }
    });
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Reversito",
        options,
        Box::new(|_cc| Ok(Box::new(ReversiApp::default()))),
    )
}

struct ReversiApp {
    game: Game,
    ai_mode: bool,
    ai_player: Option<Player>,
    sound_played: bool,
}

impl Default for ReversiApp {
    fn default() -> Self {
        Self { game: Game::new(),
            ai_mode: false,
            ai_player: None,
            sound_played: false,}
    }
}

impl eframe::App for ReversiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.heading("Reversito");

                let player_text = match self.game.current_player {
                    Player::Black => "Black's Turn",
                    Player::White => "White's Turn",
                };

                let state_text = match self.game.state {
                    GameState::Playing => player_text,
                    GameState::BlackWin => "Black Wins!",
                    GameState::WhiteWin => "White Wins!",
                    GameState::Draw => "Draw!",
                };

                ui.label(format!("{}", state_text));
                ui.label(format!("Black: {} | White: {}", self.game.black_score, self.game.white_score));

                ui.add_space(30.0);

                let board_size = 700.0;
                let cell_size = board_size / 8.0;
                let (response, painter) = ui.allocate_painter(Vec2::splat(board_size), egui::Sense::click());

                let offset = response.rect.min;

                for i in 0..8 {
                    for j in 0..8 {
                        let x = offset.x + j as f32 * cell_size;
                        let y = offset.y + i as f32 * cell_size;
                        let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::splat(cell_size));

                        let color = if (i + j) % 2 == 0 { Color32::from_rgb(34, 139, 34) } else { Color32::from_rgb(46, 158, 46) };
                        painter.rect_filled(rect, 0.0, color);
                        painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::BLACK), StrokeKind::Middle);

                        let cell = self.game.board.matrix[i][j];
                        let center = rect.center();
                        let radius = cell_size * 0.35;

                        match cell {
                            Cell::Black => painter.circle_filled(center, radius, Color32::BLACK),
                            Cell::White => painter.circle_filled(center, radius, Color32::WHITE),
                            Cell::Empty => {
                                if self.game.state == GameState::Playing && self.game.board.valid_move(i as isize, j as isize, self.game.current_player.to_cell()) {
                                    let color: Color32;
                                    if self.game.current_player.to_cell() == Cell::White {
                                        color = Color32::from_rgba_premultiplied(255, 255, 255, 60);
                                    } else {
                                        color = Color32::from_rgba_premultiplied(0, 0, 0, 60);
                                    }
                                    painter.circle_filled(center, radius * 0.2, color)
                                } else {
                                    painter.circle_filled(center, 0.0, Color32::TRANSPARENT)
                                }
                            }
                        };
                    }
                }

                if response.clicked() {
                    let can_human_move = if self.ai_mode {
                        match self.ai_player {
                            Some(ai) => self.game.current_player != ai,
                            None => true,
                        }
                    } else {
                        true
                    };
                    if can_human_move {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let rel_pos = pos - offset;
                            let col = (rel_pos.x / cell_size) as isize;
                            let row = (rel_pos.y / cell_size) as isize;

                            if row >= 0 && row < 8 && col >= 0 && col < 8 {
                                if self.game.make_move(row, col) {
                                    play_sound("click_cell.mp3");
                                }
                            }
                        }

                    }
                }
                
                // Play sound once when game ends
                if self.game.state != GameState::Playing && !self.sound_played {
                    match self.game.state {
                        GameState::BlackWin | GameState::WhiteWin => play_sound("win.mp3"),
                        GameState::Draw => play_sound("game_over.mp3"),
                        _ => {}
                    }
                    self.sound_played = true;
                }
                
                if self.ai_mode && self.game.state == GameState::Playing {
                    if let Some(ai_player) = self.ai_player {
                        if self.game.current_player == ai_player {

                            ctx.request_repaint();
                        }
                    }
                }

                ui.add_space(20.0);

                if ui.button("New Game").clicked() {
                    self.game.reset();
                    self.ai_mode = false;
                    self.ai_player = None;
                    self.sound_played = false;
                }
                if ui.button("LET AI PLAY").clicked() {
                    self.ai_mode = true;
                    self.ai_player = Some(self.game.current_player);
                }
            });
        });
    }
}
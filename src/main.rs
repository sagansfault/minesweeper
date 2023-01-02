#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Button, Vec2, Color32};
use crate::minesweeper::*;

mod minesweeper;

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(440.0, 520.0)),

        ..Default::default()
    };
    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Box::new(MinesweeperApp::default())),
    );
}

struct MinesweeperApp {
    grid: Grid,
    selected_size: Size
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        MinesweeperApp {
            grid: Grid::new(Size::TEN),
            selected_size: Size::TEN
        }
    }
}

impl eframe::App for MinesweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("sizes")
                        .selected_text(format!("{}", self.selected_size.num_val()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_size, Size::EIGHT, "8");
                            ui.selectable_value(&mut self.selected_size, Size::TEN, "10");
                            ui.selectable_value(&mut self.selected_size, Size::TWELVE, "12");
                    });
                    let clicked = ui.button("Generate").clicked();
                    if clicked {
                        self.grid = Grid::new(self.selected_size);
                    }
                });
            });

            egui::Grid::new("grid").spacing([-5.0, 0.0]).show(ui, |ui| {
                let len = self.grid.len;
                let mut row = 1;
                for index in 0..(self.grid.state.len()) {
                    if index >= len * row {
                        ui.end_row();
                        row += 1;
                    }
                    
                    let cell = self.grid.state.get_mut(index).unwrap();

                    let button = match cell {
                        Cell::Mine(state) => match state {
                            CellState::Hidden => Button::new(""),
                            CellState::Flagged => Button::new("🚩"),
                            CellState::Revealed => Button::new("💣").fill(Color32::WHITE),
                        },
                        Cell::Sus(val, state) => match state {
                            CellState::Hidden => Button::new(""),
                            CellState::Flagged => Button::new("🚩"),
                            CellState::Revealed => {
                                let text = if *val == 0 { String::from("") } else { format!("{}", val) };
                                Button::new(text).fill(Color32::WHITE)
                            },
                        },
                    }.min_size(Vec2 { x: 35.0, y: 35.0 });
                    let res = ui.add(button);
                    if res.secondary_clicked() {
                        match cell {
                            Cell::Mine(state) | Cell::Sus(_, state) => {
                                match state {
                                    CellState::Hidden => *cell = Cell::Mine(CellState::Flagged),
                                    CellState::Flagged => *cell = Cell::Mine(CellState::Hidden),
                                    CellState::Revealed => {},
                                }
                            }
                        }
                    } else if res.clicked() {
                        match cell {
                            Cell::Mine(state) => match state {
                                CellState::Hidden => {
                                    for ele in self.grid.state.iter_mut() {
                                        *ele = match ele {
                                            Cell::Mine(_) => Cell::Mine(CellState::Revealed),
                                            Cell::Sus(v, _) => Cell::Sus(*v, CellState::Revealed)
                                        }
                                    }
                                },
                                CellState::Flagged | CellState::Revealed => {},
                            },
                            Cell::Sus(_, state) => match state {
                                CellState::Hidden => {
                                    self.grid.reveal_recurs(index);
                                },
                                CellState::Flagged | CellState::Revealed => {},
                            },
                        }
                    }
                }
            });
        });
    }
}
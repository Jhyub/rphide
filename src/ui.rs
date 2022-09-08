
use std::sync::mpsc;
use eframe::{Frame};
use egui::{Context};
use crate::ui;

#[derive(Debug)]
pub enum UiUpdate {
    AskRestart,
    Hide,
    Unhide,
    AcutallyQuit,
}

#[derive(Debug)]
pub enum UiResult {
    RestartDiscord,
}

#[derive(Debug)]
pub struct Ui {
    title: String,
    show_restart_discord_dialog: bool,
    actually_close: bool,
    hide: bool,
    rx: mpsc::Receiver<UiUpdate>,
    tx: mpsc::Sender<UiResult>,
}

impl Ui {
    fn default(rx: mpsc::Receiver<UiUpdate>, tx: mpsc::Sender<UiResult>) -> Self {
        Self {
            title: format!("rphide v{}", env!("CARGO_PKG_VERSION")),
            show_restart_discord_dialog: false,
            actually_close: false,
            hide: false,
            rx,
            tx,
        }
    }

    pub fn launch(rx: mpsc::Receiver<UiUpdate>, tx: mpsc::Sender<UiResult>) {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "rphide",
            options,
            Box::new(|_cc| Box::new(Self::default(rx, tx))),
        );
    }
}

impl eframe::App for Ui {
    fn on_close_event(&mut self) -> bool {
        if self.actually_close {
            true
        } else {
            self.hide = true;
            false
        }
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Receive Data
        if let Ok(recv) = self.rx.try_recv() {
            match recv {
                UiUpdate::AskRestart => {
                    self.show_restart_discord_dialog = true;
                }
                UiUpdate::Hide => {
                    self.hide = true;
                }
                UiUpdate::Unhide => {
                    self.hide = false;
                }
                UiUpdate::AcutallyQuit => {
                    self.actually_close = true;
                    frame.close();
                    return
                }
            }
        }

        // Run UI
        frame.set_window_title(&self.title);
        egui::CentralPanel::default().show(ctx, |ui| {
        });

        if self.show_restart_discord_dialog {
            egui::Window::new("Restart Discord")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("You need to restart discord in order to use rphide");
                    ui.horizontal(|ui| {
                        if ui.button("Restart Discord").clicked() {
                            // Do Something Here
                            self.tx.send(ui::UiResult::RestartDiscord).unwrap();
                            self.show_restart_discord_dialog = false;
                        }
                        if ui.button("Quit rphide").clicked() {
                            self.actually_close = true;
                            frame.close();
                        }
                    });
                });
        }

        if self.hide {
            frame.set_visible(false);
        } else {
            frame.set_visible(true);
        }
    }
}

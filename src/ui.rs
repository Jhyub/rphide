
use std::sync::mpsc;
use eframe::{Frame};
use egui::{Context};
use crate::ui;

pub enum UiUpdate {
    AskRestart,
    Close,
}

pub enum UiResult {
    RestartDiscord,
}

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
                UiUpdate::Close => {
                    self.hide = true;
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
                    ui.heading("Discord needs to be restarted for rphide to work.");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            // Do Something Here
                            self.tx.send(ui::UiResult::RestartDiscord).unwrap();
                            self.show_restart_discord_dialog = false;
                        }
                        if ui.button("No").clicked() {
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

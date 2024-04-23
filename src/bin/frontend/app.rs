use crate::{
    comms::{self, QueueMsg},
    widgets,
};
use eframe::egui;
use rand::Rng;
use sandbox_research::{ipc_srv::EchoRequest, Profile, ProfileList, Status};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

/// `Frontend` applicaton state.
pub struct Frontend {
    cycle: u32,
    message: Option<String>,

    /// Last error message
    error: Option<Status>,
    /// Sender for comms
    comm_tx: Sender<QueueMsg>,
    /// Receiver for comms
    comm_rx: Receiver<QueueMsg>,
    /// List of loaded profiles
    profiles: ProfileList,

    /// Show the error dialogue
    show_error: bool,
    /// Show the settings dialogue
    show_settings: bool,
    /// Show the profile creation or modification section
    show_profile: bool,
    /// Currently selected profile for modifying
    editing_profile: Option<u32>,
    /// Buffer for new/modified profile data
    profile_buffer: Profile,

    /// Currently selected profile for deletion
    deleting_profile: Option<u32>,
    /// Delete the marked profile
    do_delete: Option<u32>,
}

impl Frontend {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let (comm_tx, comm_rx) = channel();
        let mut profiles = ProfileList::default();

        // TODO test configuration management and remove the following
        profiles.insert(
            rand::thread_rng().gen(),
            Profile::new(
                "Testing profile",
                "This profile is used for testing",
                "powershell.exe -noprofile",
            ),
        );

        Self {
            cycle: 0,
            message: None,
            error: None,
            comm_tx,
            comm_rx,
            profiles,
            show_error: false,
            show_settings: false,
            show_profile: false,
            editing_profile: None,
            profile_buffer: Profile::default(),
            deleting_profile: None,
            do_delete: None,
        }
    }

    fn list_menu(ui: &mut egui::Ui) {
        let _ = ui.button("Item 01");
        let _ = ui.button("Item 02");

        if ui.button("Close").clicked() {
            ui.close_menu();
        }
    }

    /// Section to add/modify a profile
    fn profile_data(&mut self, ui: &mut egui::Ui) {
        let errors = self.profile_buffer.validate();

        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.editing_profile.is_some() {
                ui.horizontal(|ui| {
                    ui.heading(format!("Editing: {}", &self.profile_buffer.name));
                });
            } else {
                ui.heading("Add a new profile");
            }
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.profile_buffer.name);
            });
            ui.horizontal(|ui| {
                ui.label("Description:");
                ui.text_edit_multiline(&mut self.profile_buffer.description);
            });
            ui.horizontal(|ui| {
                ui.label("Command:");
                ui.text_edit_singleline(&mut self.profile_buffer.command);
            });

            if !errors.is_empty() {
                ui.separator();

                ui.label("Failed to validate the profile:");
                for error in &errors {
                    ui.label(format!("-> {}", error));
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("<< Go back").clicked() {
                self.show_profile = false;
                self.profile_buffer = Profile::default();
                if self.editing_profile.is_some() {
                    self.editing_profile = None;
                }
            }

            let state_btn = ui.add_enabled(
                errors.is_empty(),
                egui::Button::new({
                    if self.editing_profile.is_some() {
                        "Save profile"
                    } else {
                        "Add profile"
                    }
                }),
            );

            if state_btn.clicked() {
                let id = self.editing_profile.unwrap_or_else(|| rand::thread_rng().gen());

                self.profiles.insert(id, self.profile_buffer.clone());

                self.show_profile = false;
                self.editing_profile = None;
                self.profile_buffer = Profile::default();
            }
        });
    }

    /// Section to render the list of active profiles
    fn profile_list(&mut self, ui: &mut egui::Ui) {
        let to_yeet = self.deleting_profile.unwrap_or_default();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for profile in &self.profiles {
                ui.horizontal_top(|ui| {
                    if *profile.0 == to_yeet {
                        ui.vertical(|ui| {
                            ui.label("Are you sure?");
                            ui.horizontal(|ui| {
                                if ui.button("Yes, delete the profile").clicked() {
                                    self.do_delete = Some(to_yeet);
                                };
                                if ui.button("No, go back").clicked() {
                                    self.deleting_profile = None;
                                }
                            });
                        });
                        ui.add_space(16.);
                        ui.vertical(|ui| {
                            ui.label(&profile.1.name);
                            ui.label(&profile.1.description);
                        });
                    } else {
                        ui.vertical(|ui| {
                            ui.heading(&profile.1.name);
                            ui.horizontal(|ui| {
                                let _ = ui.button("Start");
                                if ui.button("Edit").clicked() {
                                    self.editing_profile = Some(*profile.0);
                                    self.profile_buffer = profile.1.clone();
                                    self.show_profile = true;
                                };
                                if ui.button("Delete").clicked() {
                                    self.deleting_profile = Some(*profile.0);
                                }
                            });
                        });
                        ui.add_space(16.);
                        ui.vertical(|ui| {
                            ui.label(&profile.1.description);
                            ui.label(format!("PID: {}", &profile.1.pid));
                        });
                    }
                });
                ui.separator();
            }
        });
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            match self.comm_rx.try_recv() {
                Ok(QueueMsg::Echo(data)) => {
                    self.message = Some(data);
                }
                Ok(QueueMsg::Fail(error)) => {
                    self.error = Some(error);
                    self.show_error = true;
                }
                Err(error) => match error {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        tracing::error!("comm_tx has been disconnected");
                    }
                },
                _ => (),
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4., 8.);
            ui.spacing_mut().button_padding = egui::vec2(6., 4.);

            ui.horizontal(|ui| {
                ui.menu_button("File", Self::list_menu);
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                };
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("+ New Profile").clicked() {
                    self.profile_buffer = Profile::default();
                    self.editing_profile = None;
                    self.show_profile = true;
                };
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").min_height(128.).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4., 8.);
            ui.spacing_mut().button_padding = egui::vec2(6., 4.);

            ui.horizontal(|ui| {
                if ui.button("Request stuff").clicked() {
                    comms::echo(
                        self.comm_tx.clone(),
                        EchoRequest {
                            payload: self.cycle.clone().to_string(),
                        },
                    );
                    self.cycle += 2;
                };

                if ui.button("Clear stuff").clicked() {
                    self.cycle = 0;
                    self.message = None;
                }

                ui.label(format!("Count: {}", &self.cycle))
            });

            if let Some(message) = &self.message {
                ui.label(message);
            } else {
                ui.label("No messages yet");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4., 8.);
            ui.spacing_mut().button_padding = egui::vec2(6., 4.);

            if self.show_profile {
                self.profile_data(ui);
            } else {
                self.profile_list(ui);
            }
        });

        if let Some(id) = self.do_delete {
            self.profiles.remove(&id);
            self.deleting_profile = None;
            self.do_delete = None;
        }

        widgets::settings_dialogue(ctx, &mut self.show_settings);
        widgets::error_dialogue(ctx, &mut self.show_error, self.error);

        // HACK helps to update the ui even when theres no interaction
        // should experiment with other design patterns if possible to extract
        // the state updating block from this function so we dont have to repaint
        ctx.request_repaint_after(tokio::time::Duration::from_millis(128));
    }
}

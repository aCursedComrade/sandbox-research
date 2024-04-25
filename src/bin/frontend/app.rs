use crate::{
    comms::{self, QueueMsg},
    widgets,
};
use eframe::egui;
use sandbox_research::{Profile, ProfileList, ProfileListUtils, Status};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

/// `Frontend` applicaton state.
pub struct Frontend {
    #[cfg(debug_assertions)]
    cycle: u32,
    #[cfg(debug_assertions)]
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

        #[cfg(not(debug_assertions))]
        let profiles = ProfileList::default();

        #[cfg(debug_assertions)]
        let mut profiles = ProfileList::default();

        // Always insert a testing profile on debug build
        #[cfg(debug_assertions)]
        profiles.add_profile(Profile::new(
            "Powershell",
            "This profile is used for testing",
            "powershell.exe -noprofile",
        ));

        Self {
            #[cfg(debug_assertions)]
            cycle: 0,
            #[cfg(debug_assertions)]
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

    /// Polls for data and changes state
    fn poll_data(&mut self) {
        match self.comm_rx.try_recv() {
            #[cfg(debug_assertions)]
            Ok(QueueMsg::Echo(data)) => {
                self.message = Some(data);
            }
            Ok(QueueMsg::Fail(error)) => {
                self.error = Some(error);
                self.show_error = true;
            }
            Ok(QueueMsg::Spawn { error, id, pid }) => {
                if let Some(profile) = self.profiles.get_mut(&id) {
                    if error {
                        let _ = self
                            .comm_tx
                            .send(QueueMsg::Fail(Status::ProcessSpawnFailed(profile.name.clone())));
                    } else {
                        profile.pid = pid;
                        profile.is_running = true;
                    }
                } else {
                    let _ = self.comm_tx.send(QueueMsg::Fail(Status::NoSuchProfile(id)));
                }
            }
            Ok(QueueMsg::Stop { error, id }) => {
                if let Some(profile) = self.profiles.get_mut(&id) {
                    if error {
                        let _ = self
                            .comm_tx
                            .send(QueueMsg::Fail(Status::ProcessStopFailed(profile.name.clone())));
                    } else {
                        profile.pid = 0;
                        profile.is_running = false;
                    }
                } else {
                    let _ = self.comm_tx.send(QueueMsg::Fail(Status::NoSuchProfile(id)));
                }
            }
            Err(error) => match error {
                TryRecvError::Empty => (),
                TryRecvError::Disconnected => {
                    tracing::error!("comm_tx has been disconnected");
                }
            },
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
                if let Some(id) = self.editing_profile {
                    self.profile_buffer.id = id;
                }

                self.profiles.add_profile(self.profile_buffer.clone());

                self.show_profile = false;
                self.editing_profile = None;
                self.profile_buffer = Profile::default();
            }
        });
    }

    /// Profile card (inside [`profile_list`](Frontend::profile_list))
    fn profile_card(&mut self, ui: &mut egui::Ui, item: (u32, Profile)) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(&item.1.name);
                ui.horizontal(|ui| {
                    // start/stop buttons
                    if item.1.is_running {
                        if ui.button("Kill").clicked() {
                            comms::stop(self.comm_tx.clone(), item.0);
                        };
                    } else if ui.button("Start").clicked() {
                        comms::spawn(self.comm_tx.clone(), item.1.clone());
                    }
                    // edit button
                    if ui.button("Edit").clicked() {
                        self.editing_profile = Some(item.0);
                        self.profile_buffer = item.1.clone();
                        self.show_profile = true;
                    };
                    // delete button
                    if ui.button("Delete").clicked() {
                        self.deleting_profile = Some(item.0);
                    }
                });
            });
            ui.add_space(16.);
            ui.vertical(|ui| {
                ui.label(&item.1.description);
                ui.label(format!("PID: {}", &item.1.pid));
            });
        });
    }

    /// Profile deletion section (inside [`profile_list`](Frontend::profile_list))
    fn profile_delete(&mut self, ui: &mut egui::Ui, item: (u32, Profile)) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Are you sure?");
                ui.horizontal(|ui| {
                    if ui.button("Yes, delete the profile").clicked() {
                        self.do_delete = Some(item.0);
                    };
                    if ui.button("No, go back").clicked() {
                        self.deleting_profile = None;
                    }
                });
            });
            ui.add_space(16.);
            ui.vertical(|ui| {
                ui.label(&item.1.name);
                ui.label(&item.1.description);
            });
        });
    }

    /// Section to render the list of active profiles
    fn profile_list(&mut self, ui: &mut egui::Ui) {
        let to_yeet = self.deleting_profile.unwrap_or_default();
        let list = self.profiles.clone();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for item in list {
                if item.0 == to_yeet {
                    self.profile_delete(ui, item);
                } else {
                    self.profile_card(ui, item);
                }
                ui.separator();
            }
        });
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            self.poll_data();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4., 8.);
            ui.spacing_mut().button_padding = egui::vec2(6., 4.);

            ui.horizontal(|ui| {
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

        #[cfg(debug_assertions)]
        egui::TopBottomPanel::bottom("bottom_panel").min_height(256.).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Test server").clicked() {
                        comms::echo(self.comm_tx.clone(), format!("Useless number: {}", &self.cycle));
                        self.cycle += 2;
                    };

                    if ui.button("Clear fields").clicked() {
                        self.cycle = 0;
                        self.message = None;
                    }

                    ui.label(format!("Count: {}", &self.cycle));
                    ui.label(format!("Last message: {}", {
                        if let Some(message) = &self.message {
                            message
                        } else {
                            "No messages yet"
                        }
                    }));
                });

                ui.separator();

                ui.label(format!("Active profiles: {:#?}", &self.profiles));
            });
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
        widgets::error_dialogue(ctx, &mut self.show_error, self.error.clone());

        // HACK helps to update the ui even when theres no interaction
        // should experiment with other design patterns if possible to extract
        // the state updating function from this block so we dont have to repaint manually
        ctx.request_repaint_after(tokio::time::Duration::from_millis(256));
    }
}

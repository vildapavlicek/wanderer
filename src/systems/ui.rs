use bevy::prelude::ResMut;
use bevy::{
    ecs::prelude::{Query, Res, With},
    prelude::Windows,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use chrono::Local;
use std::borrow::Cow;

use crate::components::{
    player::Player, Agility, Endurance, Health, Intelligence, Level, Name, Race, Strength,
};

#[derive(Debug)]
pub struct LogMessages(Vec<LogEvent>);

impl LogMessages {
    pub fn new() -> Self {
        LogMessages(Vec::with_capacity(10))
    }

    pub fn add_message(&mut self, event: LogEvent) {
        if self.0.len() >= 10 {
            self.0.remove(0);
        }
        self.0.push(event);
    }
}

impl std::default::Default for LogMessages {
    fn default() -> Self {
        LogMessages::new()
    }
}

#[derive(Debug, Clone)]
pub enum LogEvent {
    PlayerAttack {
        time: chrono::DateTime<Local>,
        defender: EventTarget,
        damage: u32,
    },
    NPCAttackPlayer {
        time: chrono::DateTime<Local>,
        attacker: EventTarget,
        damage: u32,
    },
    NPCAttackNPC {
        time: chrono::DateTime<Local>,
        attacker: EventTarget,
        defender: EventTarget,
        damage: u32,
    },
}

impl LogEvent {
    pub fn player_attack(defender: String, damage: u32) -> Self {
        Self::PlayerAttack {
            time: chrono::Local::now(),
            defender: EventTarget::NPC(defender),
            damage,
        }
    }

    pub fn npc_attacks_player(attacker: String, damage: u32) -> Self {
        Self::NPCAttackPlayer {
            time: chrono::Local::now(),
            attacker: EventTarget::NPC(attacker),
            damage,
        }
    }

    pub fn npc_attacks_npc(attacker: String, defender: String, damage: u32) -> Self {
        Self::NPCAttackNPC {
            time: chrono::Local::now(),
            attacker: EventTarget::NPC(attacker),
            defender: EventTarget::NPC(defender),
            damage,
        }
    }
}

impl std::fmt::Display for LogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerAttack {
                time,
                defender,
                damage,
            } => {
                write!(
                    f,
                    "[{}]: You've attacked {} for {}",
                    time,
                    defender.inner(),
                    damage
                )
            }
            Self::NPCAttackPlayer {
                time,
                attacker,
                damage,
            } => {
                write!(
                    f,
                    "[{}]: {} attacks you for {}",
                    time,
                    attacker.inner(),
                    damage
                )
            }
            Self::NPCAttackNPC {
                time,
                attacker,
                defender,
                damage,
            } => write!(
                f,
                "[{}]: {} attacks {} for {}",
                time,
                attacker.inner(),
                defender.inner(),
                damage
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventTarget {
    Player,
    NPC(String),
}

impl EventTarget {
    pub fn inner(&self) -> Cow<str> {
        match self {
            Self::NPC(v) => Cow::Borrowed(v.as_str()),
            Self::Player => Cow::Borrowed("You"),
        }
    }
}

// impl EventTarget {
//     pub fn inner(&self) -> String {
//         match self {
//             Self::NPC(v) => v.to_owned(),
//             Self::Player => "You".to_owned(),
//         }
//     }
// }

pub fn ui(
    egui_ctx: Res<EguiContext>,
    logs: Res<LogMessages>,
    windows: Res<Windows>,
    player_query: Query<
        (
            &Health,
            &Agility,
            &Endurance,
            &Intelligence,
            &Strength,
            &Level,
            &Name,
            &Race,
        ),
        With<Player>,
    >,
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();

    let (health, agility, endurance, intelligence, strength, level, name, race) = player_query
        .single()
        .expect("failed to get player related data for UI");

    egui::TopBottomPanel::bottom("text panel")
        // .default_height(window.height() * 0.33)
        .resizable(false)
        // .max_height(window.height() * 0.33)
        .show(egui_ctx.ctx(), |ui| {
            // ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            //     ui.heading("LogPanel")
            // });
            // egui::Label::new("Log panel").heading();

            ui.allocate_exact_size(egui::Vec2::new(250., 100.), egui::Sense::hover());

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add(
                    egui::Label::new(
                        "This is just some testing log message to see what it looks like,",
                    ), // egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );

                for message in logs.0.iter() {
                    ui.add(egui::Label::new(message.to_string()));
                }
            });
        });

    egui::SidePanel::left("side_panel")
        .default_width(window.width() * 0.25) //window.width() * 0.33
        .resizable(false)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Player info");

            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.image(egui::TextureId::User(1), egui::Vec2::new(64., 64.));

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.label(name.0.as_str());
                });

                ui.horizontal(|ui| {
                    ui.label("Race: ");
                    ui.label(race.to_string());
                });

                ui.horizontal(|ui| {
                    ui.label("Level: ");
                    ui.label(level.0.to_string());
                });

                ui.separator();

                ui.add_space(25.);

                ui.horizontal(|ui| {
                    ui.label("Health: ");
                    ui.label(health.to_ui_format());
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Strength: ");
                        ui.label(strength.inner().to_string());
                        ui.label("Agility: ");
                        ui.label(agility.inner().to_string());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Endurance: ");
                        ui.label(endurance.inner().to_string());
                        ui.label("Intelligence: ");
                        ui.label(intelligence.inner().to_string());
                    });
                });
            })
        });
}

pub fn update_logs(mut events: bevy::prelude::EventReader<LogEvent>, mut log: ResMut<LogMessages>) {
    for event in events.iter() {
        log.add_message(event.clone());
    }
}

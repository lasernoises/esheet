use egui::{DragValue, ScrollArea, TextEdit, Ui, Window};
use serde::{Deserialize, Serialize};

use crate::state::{self, Item, Level};

// TODO: Figure out correct value
const ABILITY_BOOST_COUNT: u8 = 3;
const FIRST_LEVEL_SKILL_TRAINING_COUNT: u8 = 4;
const SKILL_TRAINING_COUNT: u8 = 1;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    state: state::CharSheet,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { ref mut state } = self;

        state.compute();

        // ctx.set_visuals(egui::Visuals::light());

        egui::SidePanel::left("side_panel")
            .exact_width(256.)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Plan");

                plan(ui, state);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            basic_stats(ui, state);

            #[derive(Clone, PartialEq)]
            enum Tab {
                Skills,
                Inventory,
            }

            use Tab::*;

            let id = ui.id().with("tab");
            let mut active_tab = ui.data_mut(|w| w.get_temp(id).unwrap_or(Skills));

            ui.horizontal(|ui| {
                if ui
                    .selectable_label(active_tab == Skills, "Skills")
                    .clicked()
                {
                    active_tab = Skills;
                }

                if ui
                    .selectable_label(active_tab == Inventory, "Inventory")
                    .clicked()
                {
                    active_tab = Inventory;
                }
            });

            match active_tab {
                Skills => skill_list(ui, &mut state.computed.skills),
                Inventory => inventory(ui, state),
            }

            ui.data_mut(|w| w.insert_temp(id, active_tab));
        });
    }
}

fn basic_stats(ui: &mut Ui, sheet: &mut state::CharSheet) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Level:");
            egui::ComboBox::from_id_source("level")
                .selected_text(format!("{}", sheet.level))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut sheet.level, 1, "1");
                    ui.selectable_value(&mut sheet.level, 2, "2");
                    ui.selectable_value(&mut sheet.level, 3, "3");
                    ui.selectable_value(&mut sheet.level, 4, "4");
                    ui.selectable_value(&mut sheet.level, 5, "5");
                    ui.selectable_value(&mut sheet.level, 6, "6");
                    ui.selectable_value(&mut sheet.level, 7, "7");
                    ui.selectable_value(&mut sheet.level, 8, "8");
                    ui.selectable_value(&mut sheet.level, 9, "9");
                    ui.selectable_value(&mut sheet.level, 10, "10");
                    ui.selectable_value(&mut sheet.level, 11, "11");
                    ui.selectable_value(&mut sheet.level, 12, "12");
                    ui.selectable_value(&mut sheet.level, 13, "13");
                    ui.selectable_value(&mut sheet.level, 14, "14");
                    ui.selectable_value(&mut sheet.level, 15, "15");
                    ui.selectable_value(&mut sheet.level, 16, "16");
                    ui.selectable_value(&mut sheet.level, 17, "17");
                    ui.selectable_value(&mut sheet.level, 18, "18");
                    ui.selectable_value(&mut sheet.level, 19, "19");
                    ui.selectable_value(&mut sheet.level, 20, "20");
                });

            ui.label("Character Name:");
            ui.text_edit_singleline(&mut sheet.name);
        });
    });
}

macro_rules! skills {
    ($((label: $label:expr, field: $field:ident, ability: $ability:ident,)),*,) => {
        fn skill_list(ui: &mut Ui, skills: &mut Skills) {
            egui::Grid::new("skill_list")
                .num_columns(3)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    $(
                        ui.label($label);
                        ui.label(skills.$field.0.label());
                        ui.label(format!("{:+}", skills.$field.1));
                        ui.end_row();
                    )*
                });
        }

        #[derive(Clone, Debug, Default, Deserialize, Serialize)]
        pub struct Skills {
            $(
                pub $field: (state::Training, i8),
            )*
        }

        #[derive(Clone, Debug, Default, Deserialize, Serialize)]
        pub struct SkillTraining {
            $(
                /// True means an increase of one.
                pub $field: bool,
            )*
        }

        impl Skills {
            pub fn apply_training(&mut self, training: &SkillTraining) {
                $(
                    if (training.$field) {
                        self.$field.0.increase();
                    }
                )*
            }

            pub fn compute_modifiers(&mut self, level: Level, abilities: &state::Abilities) {
                $(
                    self.$field.1 = abilities.$ability + self.$field.0.proficiency_bonus(level) as i8;
                )*
            }
        }

        fn skill_training(ui: &mut Ui, sheet: &mut state::CharSheet, level: Level) {
            let level_plan = sheet.level_plan(level);
            let training = &mut level_plan.skill_training;

            let selected_count = 0 $(+ training.$field as u8)*;

            let max_count = if level == 1 {
                FIRST_LEVEL_SKILL_TRAINING_COUNT
            } else {
                SKILL_TRAINING_COUNT
            };

            ui.label(format!("{selected_count} / {max_count}"));

            let full = selected_count >= max_count;

            $(
                ui.add_enabled(
                    !full || training.$field,
                    egui::Checkbox::new(&mut training.$field, $label),
                );
            )*
        }
    };
}

skills![
    (
        label: "Acrobatics",
        field: acrobatics,
        ability: dexterity,
    ),
    (
        label: "Athletics",
        field: athletics,
        ability: strength,
    ),
    (
        label: "Computers",
        field: computers,
        ability: mind,
    ),
    (
        label: "Crafting",
        field: crafting,
        ability: mind,
    ),
    (
        label: "Deception",
        field: deception,
        ability: charisma,
    ),
    (
        label: "Diplomacy",
        field: diplomacy,
        ability: charisma,
    ),
    (
        label: "Medicine",
        field: medicine,
        ability: mind,
    ),
    (
        label: "Nature",
        field: nature,
        ability: mind,
    ),
    (
        label: "Performance",
        field: performance,
        ability: charisma,
    ),
    (
        label: "Piloting",
        field: piloting,
        ability: mind,
    ),
    (
        label: "Society",
        field: society,
        ability: mind,
    ),
    (
        label: "Stealth",
        field: stealth,
        ability: dexterity,
    ),
    (
        label: "Survival",
        field: survival,
        ability: mind,
    ),
    (
        label: "Thievery",
        field: thievery,
        ability: dexterity,
    ),
    (
        label: "Intimidation",
        field: intimidation,
        ability: charisma,
    ),
];

fn inventory(ui: &mut Ui, sheet: &mut state::CharSheet) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Platinum");
            ui.add(DragValue::new(&mut sheet.inventory.platinum));
        });

        ui.vertical(|ui| {
            ui.label("Gold");
            ui.add(DragValue::new(&mut sheet.inventory.gold));
        });

        ui.vertical(|ui| {
            ui.label("Silver");
            ui.add(DragValue::new(&mut sheet.inventory.silver));
        });

        ui.vertical(|ui| {
            ui.label("Copper");
            ui.add(DragValue::new(&mut sheet.inventory.copper));
        });
    });

    window_button("Add Item", ui, |ui| {
        // let id = ui.id().with("search");
        // let mut search = ui.data_mut(|d| d.get_temp(id).unwrap_or(String::new()));

        ui.add(TextEdit::singleline(&mut sheet.add_item.item_search).hint_text("Search"));

        ui.separator();

        ui.label("Custom Item");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Name");
                ui.add(
                    TextEdit::singleline(&mut sheet.add_item.custom_item_name).desired_width(128.),
                );
            });

            ui.vertical(|ui| {
                ui.label("Bulk");
                ui.add(DragValue::new(&mut sheet.add_item.custom_item_bulk));
            });
        });

        if ui.button("Add").clicked() {
            sheet.inventory.items.push((
                Item {
                    id: 0,
                    name: sheet.add_item.custom_item_name.clone(),
                    bulk: sheet.add_item.custom_item_bulk,
                },
                1,
            ))
        }

        // ui.data_mut(|d| d.insert_temp(id, search));
    });

    ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("inventory")
            .num_columns(3)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Item");
                ui.label("Bulk");
                ui.label("Count");
                ui.end_row();

                for (ref item, ref mut count) in &mut sheet.inventory.items {
                    ui.label(&item.name);
                    ui.label(item.bulk.to_string());
                    // ui.label(count.to_string());
                    ui.add(DragValue::new(count));
                    ui.end_row();
                }
            });
    });
}

fn plan(ui: &mut Ui, sheet: &mut state::CharSheet) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(
            egui::Layout {
                main_dir: egui::Direction::TopDown,
                main_wrap: true,
                main_align: egui::Align::Min,
                main_justify: false,
                cross_align: egui::Align::Max,
                cross_justify: true,
            },
            |ui| {
                for l in 1..=20 {
                    level(ui, sheet, l);
                }
            },
        );
    });
}

fn level(ui: &mut Ui, sheet: &mut state::CharSheet, level: Level) {
    ui.push_id(level, |ui| {
        ui.vertical(|ui| {
            ui.label(format!("Level {level}"));

            if [1, 5, 10, 15, 20].contains(&level) {
                window_button("Abilities", ui, |ui| {
                    ability_boosts(ui, sheet, level);
                });
            }

            if [1, 3, 5, 7, 9, 11, 13, 15, 17, 19].contains(&level) {
                window_button("Skill Training", ui, |ui| {
                    skill_training(ui, sheet, level);
                });
            }

            if level == 1 {
                window_button("Heritage", ui, |ui| {});
            }

            window_button("Ancestry Feat", ui, |ui| {});
            window_button("Class Feat", ui, |ui| {});

            if level == 1 {
                window_button("Cantrips", ui, |ui| {});
            }

            window_button("Spells", ui, |ui| {});
        });
    });
}

fn ability_boosts(ui: &mut Ui, sheet: &mut state::CharSheet, level: Level) {
    let level_plan = sheet.level_plan(level);
    let boosts = &mut level_plan.ability_boosts;

    let selected_count =
        boosts.strength as u8 + boosts.dexterity as u8 + boosts.mind as u8 + boosts.charisma as u8;

    ui.label(format!("{selected_count} / {ABILITY_BOOST_COUNT}"));

    let full = selected_count >= ABILITY_BOOST_COUNT;

    ui.add_enabled(
        !full || boosts.strength,
        egui::Checkbox::new(&mut boosts.strength, "Strength"),
    );
    ui.add_enabled(
        !full || boosts.dexterity,
        egui::Checkbox::new(&mut boosts.dexterity, "Dexterity"),
    );
    ui.add_enabled(
        !full || boosts.mind,
        egui::Checkbox::new(&mut boosts.mind, "Mind"),
    );
    ui.add_enabled(
        !full || boosts.charisma,
        egui::Checkbox::new(&mut boosts.charisma, "Charisma"),
    );
}

fn window_button(title: &str, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.push_id(title, |ui| {
        let id = ui.id().with("open");
        let mut open = ui.data_mut(|d| d.get_temp(id).unwrap_or(false));

        if ui.button(title).clicked() {
            open ^= true;
        }

        Window::new(title)
            .id(ui.id().with("window"))
            .open(&mut open)
            .collapsible(false)
            .show(ui.ctx(), add_contents);

        ui.data_mut(|d| d.insert_temp(id, open));
    });
}

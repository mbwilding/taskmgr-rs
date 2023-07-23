use egui::Key::Tab;
use egui::Rounding;
use egui::TextStyle::Body;
use sysinfo::{ProcessExt, System, SystemExt};
use itertools::Itertools;
use window_titles::{Connection, ConnectionTrait};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TaskManagerApp {
    #[serde(skip)]
    sys: System,
}

impl Default for TaskManagerApp {
    fn default() -> Self {
        Self {
            sys: System::new_all(),
        }
    }
}

impl TaskManagerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Set the font color for the dark theme to white.
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TaskManagerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { sys } = self;

        sys.refresh_all();

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {


                // ui.horizontal_centered(|ui| {
                //     ui.text_edit_singleline(&mut "");
                // });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.button("🔢 Processes");
                ui.button("📈 Performance");
                ui.button("📊 App history");
                ui.button("🏁 Startup apps");
                ui.button("👥 Users");
                ui.button("📄 Details");
                ui.button("🛠 Services");
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                ui.button("⚙ Settings");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.heading("Performance");
                ui.separator();
            });

            let cpus = sys.physical_core_count().unwrap(); // May need to be virtual (32 rather than 16)
            let cpus_f32 = cpus as f32;

            // Table
            if true
            {
                use egui_extras::{Column, TableBuilder};
                let text_height = egui::TextStyle::Body.resolve(ui.style()).size;

                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .auto_shrink([false, false])
                    .cell_layout(egui::Layout::right_to_left(egui::Align::Center))
                    .column(Column::initial(100.0).range(40.0..=300.0)) // Name
                    .column(Column::initial(100.0).range(50.0..=50.0))  // CPU
                    .column(Column::initial(100.0).range(40.0..=300.0)) // Memory
                    .column(Column::initial(100.0).range(40.0..=300.0)) // Disk
                    .column(Column::initial(100.0).range(40.0..=300.0)) // Network
                    .column(Column::remainder()) // Blank
                    .min_scrolled_height(0.0);

                table
                    .header(40.0, |mut header| {
                        header.col(|ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.heading("Name");
                            });
                        });
                        header.col(|ui| {
                            ui.heading("CPU");
                        });
                        header.col(|ui| {
                            ui.heading("Memory");
                        });
                        header.col(|ui| {
                            ui.heading("Disk");
                        });
                        header.col(|ui| {
                            ui.heading("Network");
                        });
                    })
                    .body(|mut body| {
                        let row_height = 18.0;
                        for (pid, process) in sys.processes() {
                            body.row(row_height, |mut row| {
                                // Name
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                        let name = process.name();
                                        ui.horizontal(|ui| {
                                            ui.label("▶ 💻");
                                            ui.label(format!("{}", name));
                                        });
                                    });
                                });
                                // CPU
                                row.col(|ui| {
                                    let cpu = process.cpu_usage() / cpus_f32;
                                    if cpu == 0.0 {
                                        ui.label("0%");
                                    } else {
                                        ui.label(format!("{:.1}%", cpu));
                                    }
                                });
                                // Memory
                                row.col(|ui| {
                                    let memory = process.memory() as f64 / (1024 * 1024) as f64;
                                    ui.label(format!("{:.1} MB", memory));
                                });
                                // Disk
                                row.col(|ui| {
                                    let disk_read = process.disk_usage().read_bytes;
                                    let disk_write = process.disk_usage().written_bytes;
                                    let disk_combined = disk_read + disk_write;
                                    let disk = disk_combined as f64 / (1024 * 1024) as f64;
                                    ui.label(format!("{:.1} MB/s", disk));
                                });
                                // Network
                                row.col(|ui| {
                                    if ui.button("kill").clicked() {
                                        process.kill();
                                    }
                                });
                            });
                        }
                    });
            }
        });
    }
}

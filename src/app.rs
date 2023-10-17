use crate::enums::*;
use crate::windows;
use egui::*;
use std::time::{Duration, Instant};
use sysinfo::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TaskManagerApp {
    pub current_window: EWindow,

    pub processes_sort: EProcessesSort,

    #[serde(skip)]
    pub sys: System,

    #[serde(skip)]
    pub refresh_interval: Duration,

    #[serde(skip)]
    pub last_refresh_time: Instant,

    #[serde(skip)]
    pub search: String,
}

impl Default for TaskManagerApp {
    fn default() -> Self {
        Self {
            current_window: EWindow::Processes,
            processes_sort: EProcessesSort::Cpu,
            sys: System::new_all(),
            refresh_interval: Duration::from_secs(1),
            last_refresh_time: Instant::now(),
            search: String::new(),
        }
    }
}

impl TaskManagerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Set the font color for the dark theme to white.
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn bottom_panel(ctx: &Context, current_window: &mut EWindow, sys: &mut System) {
        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // ui.style_mut().override_text_style = Some(TextStyle::Heading);
                        ui.selectable_value(current_window, EWindow::Processes, "🔢 Processes");
                        ui.selectable_value(current_window, EWindow::Performance, "📈 Performance");
                        ui.selectable_value(current_window, EWindow::AppHistory, "📊 App history");
                        ui.selectable_value(
                            current_window,
                            EWindow::StartupApps,
                            "🏁 Startup apps",
                        );
                        ui.selectable_value(current_window, EWindow::Users, "👥 Users");
                        ui.selectable_value(current_window, EWindow::Details, "📄 Details");
                        ui.selectable_value(current_window, EWindow::Services, "🛠 Services");
                        ui.selectable_value(current_window, EWindow::Settings, "⚙ Settings");
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        #[cfg(target_os = "windows")]
                        {
                            ui.label(format!("Host: {}", sys.host_name().unwrap()));
                            ui.label(format!("User: {}", sys.users().first().unwrap().name()));
                            ui.label(format!(
                                "OS: {} {}",
                                sys.name().unwrap(),
                                sys.os_version().unwrap()
                            ));
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            ui.label(format!("Host: {}", sys.host_name().unwrap()));
                            ui.label(format!("OS: {}", sys.name().unwrap()));
                            ui.label(format!("Version: {}", sys.os_version().unwrap()));
                            ui.label(format!("Kernel: {}", sys.kernel_version().unwrap()));
                        }
                    });
                });
            });
    }
}

impl eframe::App for TaskManagerApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if false {
            TopBottomPanel::top("top_panel").show(ctx, |ui| {
                menu::bar(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.text_edit_singleline(&mut self.search);
                    });
                });
            });
        }

        Self::bottom_panel(ctx, &mut self.current_window, &mut self.sys);

        CentralPanel::default().show(ctx, |ui| match self.current_window {
            EWindow::Processes => {
                windows::processes::show(self, ui);
            }
            EWindow::Performance => {}
            EWindow::AppHistory => {}
            EWindow::StartupApps => {}
            EWindow::Users => {}
            EWindow::Details => {}
            EWindow::Services => {}
            EWindow::Settings => {}
        });
    }
}

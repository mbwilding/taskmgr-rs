use crate::enums::*;
use crate::windows;
use egui::*;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use sysinfo::*;

#[derive(Deserialize, Serialize)]
#[serde(default)]
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

    #[serde(skip)]
    pub top_bar_toggle: bool,
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
            top_bar_toggle: false,
        }
    }
}

impl TaskManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| {
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Type a name, publisher, or PID to search")
                        .ui(ui);
                });
            });
        });
    }

    fn bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let current_window = &mut self.current_window;

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
                            ui.label(format!(
                                "Host: {}",
                                System::host_name().unwrap_or_default()
                            ));
                            ui.label(format!(
                                "User: {}",
                                System::users().first().unwrap().name()
                            ));
                            ui.label(format!(
                                "OS: {} {}",
                                System::name().unwrap_or_default(),
                                System::os_version().unwrap_or_default()
                            ));
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            ui.label(format!("Host: {}", System::host_name().unwrap_or_default()));
                            ui.label(format!("OS: {}", System::name().unwrap_or_default()));
                            ui.label(format!(
                                "Version: {}",
                                System::host_name().unwrap_or_default()
                            ));
                            ui.label(format!(
                                "Kernel: {}",
                                System::kernel_version().unwrap_or_default()
                            ));
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
        if self.top_bar_toggle {
            self.top_panel(ctx);
        }
        self.bottom_panel(ctx);

        CentralPanel::default().show(ctx, |ui| match self.current_window {
            EWindow::Processes => windows::processes::show(self, ui),
            EWindow::Performance => windows::performance::show(self, ui),
            EWindow::AppHistory => windows::app_history::show(self, ui),
            EWindow::StartupApps => windows::startup_apps::show(self, ui),
            EWindow::Users => windows::users::show(self, ui),
            EWindow::Details => windows::details::show(self, ui),
            EWindow::Services => windows::services::show(self, ui),
            EWindow::Settings => windows::settings::show(self, ui),
        });
    }
}

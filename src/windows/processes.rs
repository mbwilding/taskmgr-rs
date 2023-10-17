use crate::enums::*;
use crate::TaskManagerApp;
use egui::*;
use egui_extras::*;
use std::time::Instant;
use sysinfo::*;

pub fn show(app: &mut TaskManagerApp, ui: &mut Ui) {
    // ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
    //     ui.heading("Processes");
    //     ui.separator();
    // });

    // Update data
    let now = Instant::now();
    if now - app.last_refresh_time >= app.refresh_interval {
        //sys.refresh_all();
        app.sys.refresh_specifics(
            RefreshKind::new()
                //.with_networks()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage().without_frequency())
                .with_processes(
                    ProcessRefreshKind::new()
                        .with_cpu()
                        .with_user()
                        .with_disk_usage(), //.without_user(),
                ),
        );
        app.last_refresh_time = now;
    }

    let mut processes: Vec<&Process> = app.sys.processes().values().collect();

    let cpus = app.sys.cpus().len(); //sys.physical_core_count().unwrap();
    let cpus_f32 = cpus as f32;

    match app.processes_sort {
        EProcessesSort::Name => {
            processes.sort_by(|a, b| a.name().cmp(b.name()));
        }
        EProcessesSort::User => {
            processes.sort_by(|a, b| a.user_id().cmp(&b.user_id()));
        }
        EProcessesSort::Cpu => {
            processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        }
        EProcessesSort::Memory => {
            processes.sort_by(|a, b| b.memory().partial_cmp(&a.memory()).unwrap());
        }
        EProcessesSort::Disk => {
            processes.sort_by(|a, b| {
                (b.disk_usage().read_bytes + b.disk_usage().written_bytes)
                    .partial_cmp(&(a.disk_usage().read_bytes + a.disk_usage().written_bytes))
                    .unwrap()
            });
        }
        EProcessesSort::Network => {
            // TODO
        }
    }

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .auto_shrink([false, false])
        .cell_layout(Layout::right_to_left(Align::Center))
        .column(Column::initial(100.0).range(40.0..=300.0)) // Name
        .column(Column::initial(100.0).range(40.0..=300.0)) // User
        .column(Column::initial(100.0).range(50.0..=50.0)) // CPU
        .column(Column::initial(100.0).range(40.0..=300.0)) // Memory
        .column(Column::initial(100.0).range(40.0..=300.0)) // Disk
        .column(Column::initial(100.0).range(40.0..=300.0)) // Network
        .column(Column::remainder()) // Blank
        .min_scrolled_height(0.0)
        .header(40.0, |mut header| {
            header.col(|ui| {
                ui.with_layout(Layout::left_to_right(Align::BOTTOM), |ui| {
                    ui.selectable_value(&mut app.processes_sort, EProcessesSort::Name, "Name");
                });
            });
            header.col(|ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.heading(" ");
                    ui.selectable_value(&mut app.processes_sort, EProcessesSort::User, "User");
                });
            });
            header.col(|ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    let cpu = app.sys.global_cpu_info().cpu_usage();
                    ui.heading(format!("{:.0}%", cpu));
                    ui.selectable_value(&mut app.processes_sort, EProcessesSort::Cpu, "CPU");
                });
            });
            header.col(|ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    let memory =
                        (app.sys.used_memory() as f64 / app.sys.total_memory() as f64) * 100.0;
                    ui.heading(format!("{:.0}%", memory));
                    ui.selectable_value(&mut app.processes_sort, EProcessesSort::Memory, "Memory");
                });
            });
            header.col(|ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.heading(" ");
                    ui.selectable_value(&mut app.processes_sort, EProcessesSort::Disk, "Disk");
                });
            });
            header.col(|ui| {
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    ui.heading(" ");
                    ui.selectable_value(
                        &mut app.processes_sort,
                        EProcessesSort::Network,
                        "Network",
                    );
                });
            });
        })
        .body(|body| {
            body.rows(28.0, processes.len(), |i, mut row| {
                let process = processes[i];
                // Name
                row.col(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let name = process.name();
                        // let title = window_titles.window_title().unwrap();
                        ui.horizontal(|ui| {
                            ui.label("▶ 💻");
                            ui.label(name);
                        });
                    });
                });
                // User
                row.col(|ui| {
                    if let Some(id) = process.user_id() {
                        if let Some(user) = app.sys.get_user_by_id(id) {
                            ui.label(user.name());
                        } else {
                            ui.label(" ");
                        }
                    } else {
                        ui.label(" ");
                    }
                });
                // CPU
                row.col(|ui| {
                    let cpu = process.cpu_usage() / cpus_f32;
                    if cpu < 0.01 {
                        ui.label("0%");
                    } else {
                        ui.label(format!("{:.2}%", cpu));
                    }
                });
                // Memory
                row.col(|ui| {
                    let memory = process.memory() as f64 / (1024 * 1024) as f64;
                    if memory < 0.01 {
                        ui.label("0 MB");
                    } else {
                        ui.label(format!("{:.2} MB", memory));
                    }
                });
                // Disk
                row.col(|ui| {
                    let disk_read = process.disk_usage().read_bytes;
                    let disk_write = process.disk_usage().written_bytes;
                    let disk_combined = disk_read + disk_write;
                    let disk = disk_combined as f64 / (1024 * 1024) as f64;
                    if disk < 0.01 {
                        ui.label("0 MB/s");
                    } else {
                        ui.label(format!("{:.2} MB/s", disk));
                    }
                });
                // Network
                row.col(|ui| {
                    // TODO
                    ui.label("0 Mbps");
                    // if ui.button("kill").clicked() {
                    //     process.kill();
                    // }
                });
            });
        });
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum EWindow {
    Processes,
    Performance,
    AppHistory,
    StartupApps,
    Users,
    Details,
    Services,
    Settings,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum EProcessesSort {
    Name,
    User,
    Cpu,
    Memory,
    Disk,
    Network,
}

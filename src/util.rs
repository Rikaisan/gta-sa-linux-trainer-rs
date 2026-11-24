use std::collections::HashMap;
use sysinfo::System;

pub fn get_target_pid() -> Option<u32> {
    let mut system = System::new().expect("Couldn't initialize sysinfo");
    system.refresh_all();

    let matching: HashMap<&str, u32> = system
        .processes()
        .iter()
        .filter(|&(_, proc)| proc.name().eq("gta-sa.exe") || proc.name().eq("gta_sa.exe"))
        .map(|(pid, proc)| {
            let pid = pid.as_u32();
            (
                proc.name()
                    .to_str()
                    .unwrap_or_else(|| panic!("Invalid process name on process with id {pid}")),
                pid,
            )
        })
        .collect();
    match matching.len() {
        0 => None,
        1 => Some(*matching.iter().next()?.1),
        2.. => Some(*matching.get("gta_sa.exe")?), // Preffer original process instead of the Steam launcher
    }
}

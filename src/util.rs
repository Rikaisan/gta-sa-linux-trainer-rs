use std::collections::HashMap;
use sysinfo::System;

pub fn get_target_pid() -> Option<u32> {
    let mut system = System::new().expect("Couldn't initialize sysinfo");
    system.refresh_all();

    let matching: HashMap<&str, u32> = system.processes()
        .iter()
        .filter_map(|(pid, proc)|
            (proc.name().eq("gta-sa.exe") || proc.name().eq("gta_sa.exe"))
            .then(|| (
                proc.name()
                    .to_str()
                    .expect(&format!("Invalid process name on process with id {}", pid.as_u32())),
                pid.as_u32()
            ))
        ).collect();
    match matching.len() {
        0 => None,
        1 => Some(*matching.iter().next()?.1),
        2.. => Some(*matching.get("gta_sa.exe")?) // Preffer original process instead of the Steam launcher
    }
}
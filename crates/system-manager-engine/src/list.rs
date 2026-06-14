use anyhow::Result;
use std::path::Path;
use std::fs;
use crate::activate::StateV1;

const STATE_FILE: &str = "/var/lib/system-manager/state/system-manager-state.json";

pub fn list_profile() -> Result<()> {
    log::info!("Reading active profile state from {}", STATE_FILE);

    if !Path::new(STATE_FILE).exists() {
        println!("No active system-manager profile found.");
        println!("(State file {} does not exist)", STATE_FILE);
        return Ok(());
    }

    let state_content = fs::read_to_string(STATE_FILE)
        .map_err(|e| anyhow::anyhow!("Failed to read state file: {}", e))?;

    let state: StateV1 = serde_json::from_str(&state_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse state file: {}", e))?;

    println!("--- Managed Services ---");
    for service in state.services.keys() {
        println!("- {}", service);
    }

    println!("
--- Managed /etc Files ---");
    for file in state.file_tree.files.iter().chain(state.file_tree.backed_up_files.iter()) {
        println!("- {}", file.display());
    }

    Ok(())
}

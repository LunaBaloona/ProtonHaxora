slint::include_modules!();
use std::process::Command;
use std::env;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::thread;
use slint::{VecModel, SharedString, ModelRc};
use steamlocate::SteamDir;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    last_game_name: String,
    last_app_id: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            last_game_name: "".into(),
            last_app_id: "".into()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    // Load config from ~/.config/protonhaxora/default-config.toml
    let cfg: AppConfig = confy::load("protonhaxora", None).unwrap_or_default();
    ui.set_search_text(cfg.last_game_name.clone().into());
    ui.set_app_id(cfg.last_app_id.clone().into());

    // 1. Fetch installed games
    let mut games: BTreeMap<String, String> = BTreeMap::new();
    if let Ok(steam_dir) = SteamDir::locate() {
        if let Ok(library_iter) = steam_dir.libraries() {
            for library in library_iter {
                if let Ok(lib) = library {
                    for app in lib.apps() {
                        if let Ok(a) = app {
                            if let Some(name) = &a.name {
                                games.insert(name.clone(), a.app_id.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let all_game_names: Vec<String> = games.keys().cloned().collect();

    // Initial population of the list (filtered by saved search text if any)
    let initial_search = cfg.last_game_name.to_lowercase();
    let initial_filtered: Vec<SharedString> = all_game_names
    .iter()
    .filter(|n| n.to_lowercase().contains(&initial_search))
    .map(|n| SharedString::from(n))
    .collect();
    ui.set_game_names(ModelRc::from(Rc::new(VecModel::from(initial_filtered))));

    // 2. Search Callback (Filters the list as you type)
    let ui_handle_search = ui.as_weak();
    let names_for_search = all_game_names.clone();
    ui.on_search_edited(move |text| {
        if let Some(ui) = ui_handle_search.upgrade() {
            let search_term = text.to_lowercase();
            let filtered: Vec<SharedString> = names_for_search
            .iter()
            .filter(|n| n.to_lowercase().contains(&search_term))
            .map(|n| SharedString::from(n))
            .collect();
            ui.set_game_names(ModelRc::from(Rc::new(VecModel::from(filtered))));
        }
    });

    // 3. Selection Callback
    let ui_handle_select = ui.as_weak();
    let games_clone = games.clone();
    ui.on_game_selected(move |name| {
        if let Some(ui) = ui_handle_select.upgrade() {
            if let Some(id) = games_clone.get(name.as_str()) {
                ui.set_app_id(SharedString::from(id));
                // Save this selection to config
                let new_cfg = AppConfig {
                    last_game_name: name.to_string(),
                        last_app_id: id.clone(),
                };
                let _ = confy::store("protonhaxora", None, new_cfg);
            }
        }
    });

    // 4. Launch Logic
    ui.on_run_protonhax(|app_id| {
        let app_id_str = app_id.to_string();

        println!("Launching Steam Game {}...", app_id_str);
        let _ = Command::new("steam")
        .arg(format!("steam://run/{}", app_id_str))
        .spawn();

        thread::spawn(move || {
            let device_state = DeviceState::new();
            println!("Waiting for F1...");
            loop {
                let keys = device_state.get_keys();
                if keys.contains(&Keycode::F1) {
                    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                    let path = format!("{}/AuroraLauncher/Aurora.exe", home);

                    let _ = Command::new("protonhax")
                    .arg("run")
                    .arg(&app_id_str)
                    .arg(path)
                    .spawn();
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    });

    ui.run()?;
    Ok(())
}

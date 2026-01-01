slint::include_modules!();
use std::process::Command;
use std::env;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::thread;
use slint::{VecModel, SharedString, ModelRc};
use steamlocate::SteamDir;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    // 1. Fetch installed games
    let mut games: BTreeMap<String, String> = BTreeMap::new();

    // Note: removed 'mut' here to prevent the warning
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

    // 2. Load names into UI
    let names: Vec<SharedString> = games.keys().map(|n| SharedString::from(n)).collect();
    let name_model = Rc::new(VecModel::from(names));
    ui.set_game_names(ModelRc::from(name_model));

    // 3. Handle selection
    let ui_handle = ui.as_weak();
    let games_clone = games.clone();
    ui.on_game_selected(move |name| {
        if let Some(ui) = ui_handle.upgrade() {
            if let Some(id) = games_clone.get(name.as_str()) {
                ui.set_app_id(SharedString::from(id));
            }
        }
    });

    // 4. Two-step Launch Logic
    ui.on_run_protonhax(|app_id| {
        let app_id_str = app_id.to_string();

        // 1. Launch the Steam Game immediately using the steam binary
        println!("Launching Steam Game {}...", app_id_str);
        let _ = Command::new("steam")
        .arg(format!("steam://run/{}", app_id_str))
        .spawn();

        // Start background thread to listen for F1
        thread::spawn(move || {
            let device_state = DeviceState::new();
            println!("Ready! Press F1 when the game is at the menu...");

            loop {
                let keys = device_state.get_keys();
                if keys.contains(&Keycode::F1) {
                    println!("F1 detected! Triggering Aurora via protonhax...");

                    // Optional: Send a KDE notification so you see it in the corner of your screen
                    let _ = Command::new("notify-send")
                    .arg("phxaur")
                    .arg("F1 detected! Launching Aurora...")
                    .spawn();

                    let home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
                    let path = format!("{}/AuroraLauncher/Aurora.exe", home);

                    let _ = Command::new("protonhax")
                    .arg("run")
                    .arg(&app_id_str)
                    .arg(path)
                    .spawn();

                    break;
                }
                // Sleep briefly to keep CPU usage low
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    });

    ui.run()?;
    Ok(())
}

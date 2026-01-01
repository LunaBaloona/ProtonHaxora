slint::include_modules!();
use slint::{ModelRc, SharedString, VecModel};
use std::collections::BTreeMap;
use std::env;
use std::process::Command;
use std::rc::Rc;
use steamlocate::SteamDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    // 1. Fetch installed games from steam libraary
    let mut games: BTreeMap<String, String> = BTreeMap::new();

    println!("Scanning Steam libraries...");
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
    println!("Found {} games.", games.len());

    // 2. Prepare the data for slint
    let sorted_names: Vec<SharedString> =
        games.keys().map(|name| SharedString::from(name)).collect();

    let name_model = Rc::new(VecModel::from(sorted_names));
    ui.set_game_names(ModelRc::from(name_model));

    // 3. Handle game selection from dropdown
    let ui_handle = ui.as_weak();
    let games_clone = games.clone();
    ui.on_game_selected(move |name| {
        if let Some(ui) = ui_handle.upgrade() {
            if let Some(id) = games_clone.get(name.as_str()) {
                ui.set_app_id(SharedString::from(id));
            }
        }
    });

    // 4. Handle the Laaunch button
    ui.on_run_protonhax(|app_id| {
        let home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let path = format!("{}/AuroraLauncher/Aurora.exe", home);

        println!("Launching App ID {} with Aurora...", app_id);

        let status = Command::new("protonhax")
            .arg("run")
            .arg(app_id.as_str())
            .arg(path)
            .spawn();

        match status {
            Ok(_) => println!("Protonhax process started."),
            Err(e) => eprintln!("Error launching protonhax: {}", e),
        }
    });

    println!("UI starting...");
    ui.run()?;
    Ok(())
}

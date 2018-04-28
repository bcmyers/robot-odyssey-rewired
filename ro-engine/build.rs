use std::env;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::process::Command;


fn python<S: AsRef<OsStr> + Debug>(args: &[S]) {
    let status = Command::new("python3")
        .args(args)
        .status()
        .expect("Failed to execute python subprocess");
    if !status.success() {
        panic!("Error from Python process while running {:?}", args);
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let original_dir = env::var("ORIGINAL_DIR").unwrap_or_else(|_| "../original".to_string());

    python(&["scripts/check-originals.py", &original_dir, &out_dir]);

    let game_scripts = &[
        "bt_game.py",
        "bt_lab.py",
        "bt_tutorial.py",
        "bt_menu.py",
        "bt_renderer.py",
    ];

    for script in game_scripts.iter() {
        let script = format!("scripts/{}", script);
        python(&[&script, &out_dir]);
    }
}

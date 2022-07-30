// TODO test on linux

use std::{thread, time, env, process::Command};

fn notify() {
    match env::consts::OS {
        "macos" => notify_macos(),
        "linux" => notify_linux(),
        _ => {
            println!("Unsuported");
            return;
        }
    }
}

fn notify_macos() {
    Command::new("osascript")
        .arg("-e")
        .arg("display alert \"Time to drink!\"")
        .spawn()
        .expect("Failed to show notification")
    ;
}

fn notify_linux() {
    Command::new("xmessage")
        .arg("Time to drink!")
        .spawn()
        .expect("Failed to show notification")
    ;
}

fn main() {
    if !["linux", "macos"].contains(&env::consts::OS) {
        println!("Unsuported os.");
        return;
    }

    loop {
        thread::sleep(time::Duration::from_secs(1800));
        notify();
    }
}

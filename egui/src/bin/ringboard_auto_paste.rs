/// It is used to automatically paste the clipboard content after running the ringboard-egui application which was customized the exit code.
/// To execute this file, use the following command:
/// ```bash
/// cargo run --release --bin ringboard_auto_paste
/// ```
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;

// Specify the command for ringboard-egui execution
const RINGBOARD_COMMAND: &str = "../target/release/ringboard-egui";
// Directory of ringboard source (for `cargo run`)
const RINGBOARD_DIR: &str = "/home/senken/senkenn/clipboard-history/egui";

fn simulate_ctrl_v() -> bool {
    // Check if `ydotool` is installed
    if !Command::new("which")
        .arg("ydotool")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error: `ydotool` is not installed.");
        eprintln!("On Linux, install it using:");
        eprintln!("  sudo apt update && sudo apt install ydotool  (Debian/Ubuntu)");
        eprintln!("  sudo dnf install ydotool  (Fedora)");
        eprintln!("  sudo pacman -S ydotool  (Arch Linux)");
        return false;
    }

    // Simulate Ctrl+V key presses using ydotool
    // Key codes: 29 = Ctrl, 47 = V
    // Sequence: 29:1 (Ctrl down), 47:1 (V down), 47:0 (V up), 29:0 (Ctrl up)
    match Command::new("ydotool")
        .args(&["key", "29:1", "47:1", "47:0", "29:0"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("Simulated Ctrl+V successfully.");
            true
        }
        Ok(status) => {
            eprintln!("Error executing `ydotool`. Status: {:?}", status);
            false
        }
        Err(e) => {
            eprintln!("Error during Ctrl+V simulation: {}", e);
            false
        }
    }
}

fn main() {
    println!("Launching {}...", RINGBOARD_COMMAND);
    let status = Command::new(RINGBOARD_COMMAND)
        .current_dir(RINGBOARD_DIR)
        .status();

    match status {
        Ok(s) => {
            println!(
                "{} has exited. exit code={}",
                RINGBOARD_COMMAND,
                s.code().unwrap_or(-1)
            );
            sleep(Duration::from_millis(100));
            if s.success() {
                simulate_ctrl_v();
                exit(0);
            } else {
                eprintln!("Non-zero exit code; skipping paste.");
                exit(s.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("Error running {}: {}", RINGBOARD_COMMAND, e);
            exit(1);
        }
    }
}

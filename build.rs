use std::process::Command;

fn main() {
  Command::new("npm")
    .args(&["run", "build"])
    .current_dir("client")
    .status()
    .unwrap();
}
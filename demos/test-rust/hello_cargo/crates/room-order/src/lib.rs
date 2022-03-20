mod identity;

// ref: speech-manageer
pub fn pause() {
    let _ = std::process::Command::new("cmd.exe")
        .arg("/c")
        .arg("pause")
        .status();
}
pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

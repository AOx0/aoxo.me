use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

fn main() {
    loop {
        let c = Command::new("curl")
            .arg("-L")
            .args(["--connect-timeout", "3"])
            .arg("aoxo.me/status.html")
            .output()
            .expect("Failed to execute guard")
            .stdout.len();

        if c == 0 {
            Command::new("killall")
                .arg("aoxo")
                .output()
                .expect("Failed to kill process");

            if let Ok(_) = Command::new("/Users/alejandro/actix/target/release/aoxo")
                .spawn() {
                    sleep(Duration::from_secs(2));
                    continue;
                }
        }
        
        sleep(Duration::from_secs(60));
    }
}

use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

fn main() {
    const OK: [u8; 2] = [111, 110];

    loop {
        let c = Command::new("curl")
            .arg("-L")
            .args(["--connect-timeout", "3"])
            .arg("aoxo.me/status.html")
            .output()
            .expect("Failed to execute guard");
        

        if c.stdout.len() == 0 {
//            println!("Server is down");
            Command::new("killall")
                .arg("aoxo")
                .output()
                .expect("Failed to kill process");

            if let Ok(_) = Command::new("/Users/alejandro/actix/target/release/aoxo")
                .spawn() {
                    sleep(Duration::from_secs(2));
                    continue;
                } else {
//                    eprintln!("Failed to spawn") 
                };
        } else if &c.stdout[..] == &OK[..] {
//            println!("Server up!");
        } else {
//            println!("Unknown error");
        }
        
        sleep(Duration::from_secs(60));
    
    }
}

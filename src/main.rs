use std::env::Args;
use std::process::Command;

#[derive(Debug,PartialEq)]
enum Platform {
    All,
    Native,
    Web
}

impl From<Args> for Platform {
    fn from(args: Args) -> Self {
        let args: Vec<String> = args.collect();
        if let Some(arg) = args.get(1) {
            let arg = arg.to_lowercase();
            if arg == "web" {
                Platform::Web
            } else if arg == "native" {
                Platform::Native
            } else {
                Platform::All
            }
        } else {
            Platform::All
        }
    }
}

fn execute(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args.iter())
        .status()
        .expect("Couldn't get ExitStatus.");
    if !status.success() {
        std::process::exit(-1);
    }
}

fn main() {
    let platform: Platform = std::env::args().into();
    execute("cargo", &["install", "cargo-cross-test"]);
    execute("cargo", &["cross-test", &format!("{:?}", platform)]);
}

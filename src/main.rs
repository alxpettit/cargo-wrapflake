use std::env;
use std::process::{Command, Stdio};

fn main() {
    let mut args: Vec<String> = env::args().skip(2).collect();
    println!("{:?}", args);
    let nix_flake = vec!["develop".to_owned(), "--command".to_owned()];

    let mut child_args = nix_flake;
    child_args.push("cargo".to_owned());
    child_args.append(&mut args);

    Command::new("nix")
        .args(child_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .expect("Failed to run cargo.")
        .wait()
        .expect("Isn't running :(");
}

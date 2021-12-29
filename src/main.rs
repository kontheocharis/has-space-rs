use std::{
    env,
    process::{exit, Command},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SpaceQueryEntry {
    index: i32,
    #[serde(rename = "has-focus")]
    focused: bool,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let usage = || -> ! {
        eprintln!("Usage: has_space_rs {{next|prev}}");
        exit(1);
    };

    let change: isize = if args.len() != 2 {
        usage();
    } else if args[1] == "next" {
        1
    } else if args[1] == "prev" {
        -1
    } else {
        usage();
    };

    let stdout = Command::new("yabai")
        .args(&["-m", "query", "--spaces", "--display"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let spaces: Vec<SpaceQueryEntry> = serde_json::from_slice(&stdout).expect("Got invalid JSON");

    let current = spaces
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            if x.focused {
                Some(i as isize)
            } else {
                None
            }
        })
        .expect("Was expecting to find one space which is focused");

    let next = current + change;
    if next >= 0 && next < spaces.len() as isize {
        exit(0);
    } else {
        exit(1);
    }
}

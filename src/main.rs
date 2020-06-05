mod app_data;
mod github;
mod messages;

use github::*;
use messages::*;

use {
    app_data::AppData,
    dialoguer::Input,
    std::{
        env,
        process::{Command, Output},
    },
};

fn main() {
    let mut c = AppData::new().unwrap();
    let user_name = c.get("user_name").map(String::from).unwrap_or_else(|| {
        let name: String = Input::new()
            .with_prompt("Your github nick")
            .interact()
            .unwrap();

        c.set("user_name", name.clone().as_str());
        c.save().unwrap();

        name
    });

    let args: Vec<_> = env::args().collect();

    let switch_is_set = |switches: &[&str]| {
        args.iter()
            .any(|el| switches.iter().any(|switch| switch == el))
    };

    if switch_is_set(&["-h", "--help"]) {
        println!("{}", HELP_MSG);
        return;
    } else if switch_is_set(&["-c", "--clean"]) {
        std::fs::remove_file(dirs::home_dir().unwrap().join("./clone-cfg.bin")).unwrap();
        println!("Clean");
        return;
    }

    let current_user = GHProfile(user_name.clone());

    let mut repo_name = if args.len() == 1 {
        current_user.choice_repo()
    } else {
        let repo_or_user_name = args[1].clone();

        if current_user.repo_exists(&repo_or_user_name) {
            Some(repo_or_user_name) // it's repo name
        } else {
            GHProfile(repo_or_user_name).choice_repo()
        }
    }
    .unwrap();

    if repo_name.find('/').is_none() {
        repo_name = format!("{}/{}", user_name, repo_name);
    }

    println!("Cloning from https://github.com/{}.git", repo_name);
    let result = Command::new("git")
        .arg("clone")
        .arg(format!("https://github.com/{}.git", repo_name))
        .output()
        .expect("Error during download repo");

    println!("{}", get_message(result));
}

fn get_message(obj: Output) -> String {
    String::from_utf8(if !obj.stdout.is_empty() {
        obj.stdout
    } else {
        obj.stderr
    })
    .unwrap()
}

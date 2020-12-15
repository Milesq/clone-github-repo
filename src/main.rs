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

fn execute_switch(args: Vec<String>, actions: Vec<(&[&str], impl Fn())>) -> bool {
    let is_switch_set = |switches: &[&str]| {
        args.iter()
            .any(|el| switches.iter().any(|switch| switch == el))
    };

    for (args, handler) in actions {
        if is_switch_set(args) {
            handler();
            return true;
        }
    }

    false
}

fn preparse_args(args: Vec<String>) -> bool {
    let actions: Vec<(&[&str], fn())> = vec![
        (&["-h", "--help"], || println!("{}", HELP_MSG)),
        (&["-c", "--clean"], || {
            std::fs::remove_file(dirs::home_dir().unwrap().join("./clone-cfg.bin")).unwrap();
            println!("Clean");
        }),
    ];

    execute_switch(args, actions)
}

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

    if preparse_args(args.clone()) {
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
            GHProfile(repo_or_user_name.clone())
                .choice_repo()
                .map(|repo_owned_by_another_user| {
                    format!("{}/{}", repo_or_user_name, repo_owned_by_another_user)
                })
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

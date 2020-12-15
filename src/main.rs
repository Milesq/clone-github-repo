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

    println!("{:?}", args);
    // println!("{:?}", match_repo_adress(Some()));
}

#[derive(Debug, PartialEq)]
enum RepoAdressType {
    OwnedByCurrentUser,        // clone
    SpecifiedCurrentUsersRepo, // clone my-repo
    OwnedByStrangeUser,        // clone github-nickname
    SpecifiedUserAndRepo,      // clone github-nickname/his-repo
}

fn match_repo_adress(argument: Option<String>) -> Option<RepoAdressType> {
    use RepoAdressType::*;
    let argument = argument?;
    Some(SpecifiedUserAndRepo)
}

fn get_message(obj: Output) -> String {
    String::from_utf8(if !obj.stdout.is_empty() {
        obj.stdout
    } else {
        obj.stderr
    })
    .unwrap()
}

#[cfg(test)]
mod test_match_repo_adress {
    use super::{
        *,
        RepoAdressType::*
    };

    #[test]
    fn returns_none_when_argument_is_none() {
        assert_eq!(match_repo_adress(None), None);
    }

    #[test]
    fn returns_specified_user_and_repo() {
        assert_eq!(
            match_repo_adress(Some(String::from("Milesq/awesome-project"))),
            Some(SpecifiedUserAndRepo)
        );
    }
}

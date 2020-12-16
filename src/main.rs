mod app_data;
mod github;
mod messages;
mod utils;

use github::*;
use messages::*;
use utils::*;

use {
    app_data::AppData,
    dialoguer::Input,
    std::{env, process::Command},
};

// #[link(name = "double_input")]
// extern {
//     fn DoubleInput(input: libc::c_int) -> libc::c_int;
// }

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

    println!("{:?}", match_repo_adress(&user_name, args.get(1)));
}

#[derive(Debug, PartialEq)]
enum RepoAdressType {
    OwnedByCurrentUser,        // clone
    SpecifiedCurrentUsersRepo, // clone my-repo
    OwnedByStrangeUser,        // clone github-nickname
    SpecifiedUserAndRepo,      // clone github-nickname/his-repo
}

fn match_repo_adress(current_user: &String, argument: Option<&String>) -> RepoAdressType {
    use RepoAdressType::*;
    let argument = match argument {
        Some(argument) => argument,
        None => return OwnedByCurrentUser,
    };

    if argument.find('/').is_some() {
        return SpecifiedUserAndRepo;
    }

    let current_user = GHProfile("Milesq".to_string());

    if current_user.repo_exists(argument) {
        return SpecifiedCurrentUsersRepo;
    }

    OwnedByStrangeUser
}

#[cfg(test)]
mod test_match_repo_adress {
    use super::{RepoAdressType::*, *};

    #[test]
    fn returns_none_when_argument_is_none() {
        assert_eq!(match_repo_adress(None), OwnedByCurrentUser);
    }

    #[test]
    fn returns_specified_user_and_repo() {
        assert_eq!(
            match_repo_adress(Some(&String::from("Milesq/awesome-project"))),
            SpecifiedUserAndRepo
        );
    }
}

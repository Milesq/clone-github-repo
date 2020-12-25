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
    std::{convert::Into, env},
};

fn main() {
    let mut c = AppData::new().unwrap();
    let user_name = c.get("user_name").map(String::from).unwrap_or_else(|| {
        let name: String = Input::new()
            .with_prompt("Your github nick")
            .interact()
            .unwrap();

        c.set("user_name", &name);
        c.save().unwrap();

        name
    });

    c.get("token").map(String::from).unwrap_or_else(|| {
        let token: String = Input::new()
            .with_prompt("Your github access token")
            .interact()
            .unwrap();

        c.set("token", &token);
        c.save().unwrap();

        token
    });

    let args: Vec<_> = env::args().collect();

    if preparse_args(args.clone()) {
        return;
    }

    // println!(
    //     "{:?}",
    //     match_repo_adress(&user_name, args.get(1).map(|el| el.as_str()))
    // );
}

#[derive(Debug, PartialEq)]
enum RepoAdressType {
    OwnedByCurrentUser,        // clone
    SpecifiedCurrentUsersRepo, // clone my-repo
    OwnedByStrangeUser,        // clone github-nickname
    SpecifiedUserAndRepo,      // clone github-nickname/his-repo
}

fn match_repo_adress(current_user: impl Into<String>, argument: Option<&str>) -> RepoAdressType {
    let current_user = current_user.into();

    use RepoAdressType::*;
    let argument: String = match argument {
        Some(argument) => argument.into(),
        None => return OwnedByCurrentUser,
    };

    if argument.find('/').is_some() {
        return SpecifiedUserAndRepo;
    }

    let current_user = GHProfile(
        current_user.clone(),
        AppData::new().unwrap().get("token").unwrap().to_string(),
    );

    if current_user.repo_exists(&argument) {
        return SpecifiedCurrentUsersRepo;
    }

    OwnedByStrangeUser
}

#[cfg(test)]
mod test_match_repo_adress {
    use super::{RepoAdressType::*, *};

    #[test]
    fn returns_none_when_argument_is_none() {
        assert_eq!(match_repo_adress("Milesq", None), OwnedByCurrentUser);
    }

    #[test]
    fn returns_specified_user_and_repo() {
        assert_eq!(
            match_repo_adress("Milesq", Some("Milesq/awesome-project")),
            SpecifiedUserAndRepo
        );
    }
}

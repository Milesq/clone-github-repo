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

    if c.get("token").is_none() {
        let token: String = Input::new()
            .with_prompt("Your github access token")
            .interact()
            .unwrap();

        c.set("token", &token);
        c.save().unwrap();
    }

    let args: Vec<_> = env::args().collect();

    if preparse_args(args.clone()) {
        return;
    }

    let arg = args.get(1).map(|el| el.as_str());
    let arg_type = match_repo_adress(&user_name, arg);
    let arg = arg.or(Some("")).unwrap();

    use RepoAdressType::*;
    let path = match arg_type {
        OwnedByCurrentUser => {
            let choosen = GHProfile::new("Milesq").choice_repo().unwrap();
            format!("{}/{}", user_name, choosen)
        }
        SpecifiedCurrentUsersRepo => {
            format!("{}/{}", user_name, arg)
        }
        OwnedByStrangeUser => {
            let choosen = GHProfile::new(arg).choice_repo().unwrap();
            format!("{}/{}", arg, choosen)
        }
        SpecifiedUserAndRepo => arg.into(),
    };

    let path = format!("https://github.com/{}.git/", path);

    println!("{}", path);

    let result = Command::new("git")
        .arg("clone")
        .arg(path)
        .output()
        .expect("Error during download repo");

    println!("{}", get_message(result));
}

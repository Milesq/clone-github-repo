use {
    dialoguer::Select,
    isahc::prelude::*,
    serde_json::Value,
    std::{
        env, io,
        process::{Command, Output},
    },
};

fn main() {
    let user_name = Some("Milesq");
    let args: Vec<String> = env::args().collect();
    let mut repo = if args.len() == 1 {
        if user_name.is_none() {
            panic!("Passed to few arguments");
        }

        let options = get_repos(user_name.unwrap());
        if let Some(options) = options {
            let choosen = Select::new()
                .with_prompt("Please choose one of yours repo")
                .items(options.as_slice())
                .paged(true)
                .interact_opt()
                .unwrap()
                .map(|choosen| options.get(choosen).unwrap());

            if let Some(choosen) = choosen {
                choosen.clone()
            } else {
                input()
            }
        } else {
            input()
        }
    } else {
        args[1].clone()
    };

    if repo.find('/').is_none() {
        repo = format!("{}/{}", user_name.unwrap(), repo);
    }

    let result = Command::new("git")
        .arg("clone")
        .arg(format!("https://github.com/{}.git", repo))
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

fn get_repos(user_name: &str) -> Option<Vec<String>> {
    let url = format!("https://api.github.com/users/{}/repos", user_name);

    let mut resp = to_opt(isahc::get(url.as_str()))?;
    let repos = to_opt(resp.text())?;
    let repos = to_opt(serde_json::from_str::<Value>(&repos))?;

    if let Value::Array(repos) = repos {
        let repos = repos
            .iter()
            .filter_map(|repo| {
                if let Value::String(name) = repo["name"].clone() {
                    Some(name)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(repos)
    } else {
        None
    }
}

fn to_opt<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

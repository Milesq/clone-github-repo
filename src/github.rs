use {dialoguer::Select, isahc::prelude::*, serde_json::Value};

#[derive(Debug, Clone)]
pub struct GHProfile(pub String);

impl GHProfile {
    pub fn repos(&self) -> Option<Vec<String>> {
        let mut all_repos = Vec::new();
        let mut done = false;
        let mut current_page = 1;

        while !done {
            let url = format!(
                "https://api.github.com/users/{}/repos?page={}",
                self.0, current_page
            );
            current_page += 1;

            let mut resp = to_opt(isahc::get(url.as_str()))?;
            let repos = to_opt(resp.text())?;
            let repos = to_opt(serde_json::from_str::<Value>(&repos))?;

            if let Value::Array(repos) = repos {
                if repos.is_empty() {
                    done = true;
                }

                let mut new_repos = repos
                    .iter()
                    .filter_map(|repo| {
                        if let Value::String(name) = repo["name"].clone() {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                all_repos.append(&mut new_repos);
            }
        }

        Some(all_repos)
    }

    pub fn repo_exists(&self, name: &str) -> bool {
        self.repos()
            .unwrap()
            .iter()
            .any(|repo| repo.as_str() == name)
    }

    pub fn choice_repo(&self) -> Option<String> {
        let repos = self.repos()?;
        Select::new()
            .with_prompt("Please choose repo to download")
            .items(repos.as_slice())
            .paged(true)
            .interact_opt()
            .unwrap()
            .map(|choosen| repos.get(choosen))
            .flatten()
            .map(|choosen| choosen.as_str().to_string())
    }
}

fn to_opt<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

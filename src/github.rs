use {dialoguer::Select, isahc::prelude::*, serde_json::Value};

#[derive(Debug, Clone)]
pub struct GHProfile(pub String);

impl GHProfile {
    pub fn repos(&self) -> Option<Vec<String>> {
        println!("repos!");

        Some(vec!["".to_string(), "das".to_string()])
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

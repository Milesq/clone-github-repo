use {dialoguer::Select, serde_json::{Value, json}, gql::GraphqlClient};

#[derive(Debug, Clone)]
pub struct GHProfile(pub String);

impl GHProfile {
    pub fn repos(&self) -> Option<Vec<String>> {
        let mut gql = gql::GraphqlClient::new("https://api.github.com/graphql");
        let get_repos_query = gql
            .auth("80da5a3b6eeb85f66fc6111529fd01d73d012b27")
            .query(include_str!("./getRepos.gql"));

        let data = get_repos_query.send(Some(json!({
            "login": "Milesq"
        })));

        println!("{:#?}", data);

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

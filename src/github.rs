use {
    dialoguer::Select,
    gql::GraphqlClient,
    serde_json::{json, Value},
};

#[derive(Debug, Clone)]
pub struct GHProfile(pub String, pub String);

impl GHProfile {
    pub fn repos(&self) -> Option<Vec<String>> {
        let mut gql = GraphqlClient::new("https://api.github.com/graphql");
        let get_repos_query = gql.auth(&self.1).query(include_str!("./getRepos.gql"));

        let data = get_repos_query
            .send(Some(json!({
                "login": self.0
            })))
            .unwrap();

        Some(
            data["user"]["repositories"]["nodes"]
                .as_array()?
                .iter()
                .map(|el| match &el["name"] {
                    Value::String(s) => s.clone(),
                    _ => panic!("value is not a string"),
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn repo_exists(&self, name: &str) -> bool {
        let mut gql = GraphqlClient::new("https://api.github.com/graphql");
        let get_repos_query = gql.auth(&self.1).query(include_str!("./repoExists.gql"));

        let data = get_repos_query
            .send(Some(json!({
                "login": self.0,
                "name": name
            })))
            .unwrap();

        matches!(data["repository"], Value::Object(_))
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

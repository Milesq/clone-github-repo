use {dialoguer::Select, gql::GraphqlClient, serde::Deserialize, serde_json::json};

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
            serde_json::from_value::<ReposResponse>(data)
                .ok()?
                .user
                .repositories
                .nodes
                .iter()
                .map(|el| el.name.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn repo_exists(&self, name: &str) -> bool {
        let mut gql = GraphqlClient::new("https://api.github.com/graphql");
        let get_repos_query = gql.auth(&self.1).query(include_str!("./getRepos.gql"));

        let data = get_repos_query
            .send(Some(json!({
                "login": self.0,
                "repo": name
            })))
            .unwrap();

        println!("{:?}", data);

        true
        // self.repos()
        //     .unwrap()
        //     .iter()
        //     .any(|repo| repo.as_str() == name)
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

#[derive(Deserialize, Debug)]
struct ReposResponse {
    user: User,
}

#[derive(Deserialize, Debug)]
struct User {
    repositories: Repo,
}
#[derive(Deserialize, Debug)]
struct Repo {
    nodes: Vec<Node>,
}

#[derive(Deserialize, Debug)]
struct Node {
    name: String,
}

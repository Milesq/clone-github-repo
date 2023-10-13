use {
    dialoguer::FuzzySelect,
    gql::GraphqlClient,
    serde::Deserialize,
    serde_json::{json, Value},
};

#[derive(Debug, Clone)]
pub struct GHProfile(pub String, pub String);

impl GHProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self(
            name.into(),
            crate::AppData::new()
                .unwrap()
                .get("token")
                .unwrap()
                .to_string(),
        )
    }

    fn repos_internal(&self, cursor: Option<String>) -> Option<(Vec<String>, Option<String>)> {
        let mut gql = GraphqlClient::new("https://api.github.com/graphql");
        let get_repos_query = gql.auth(&self.1).query(include_str!("./getRepos.gql"));

        let data = get_repos_query
            .send(Some(json!({
                "login": self.0,
                "after": cursor,
            })))
            .unwrap();
        let repos = data["user"]["repositories"].clone();

        let page_info: PageInfo = serde_json::from_value(repos["pageInfo"].clone()).ok()?;

        Some((
            repos["nodes"]
                .as_array()?
                .iter()
                .map(|el| match &el["name"] {
                    Value::String(s) => s.clone(),
                    _ => panic!("value is not a string"),
                })
                .collect::<Vec<_>>(),
            if page_info.hasNextPage {
                Some(page_info.endCursor)
            } else {
                None
            },
        ))
    }

    pub fn repos(&self) -> Option<Vec<String>> {
        let mut total = self.repos_internal(None)?;

        while let Some(cursor) = &total.1 {
            let (mut data, next_page_id) = self.repos_internal(Some(cursor.clone()))?;
            total.0.append(&mut data);
            total.1 = next_page_id;
        }

        Some(total.0)
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
        FuzzySelect::new()
            .with_prompt("Please choose repo to download")
            .items(repos.as_slice())
            .interact_opt()
            .unwrap()
            .map(|choosen| repos.get(choosen))
            .flatten()
            .map(|choosen| choosen.as_str().to_string())
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct PageInfo {
    hasNextPage: bool,
    endCursor: String,
}

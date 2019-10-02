use crate::{
    query::contributions_query::ContributionsQueryUserContributionsCollectionContributionCalendar,
    query::contributions_query::ContributionsQueryUserRepositories,
};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/ContributionsQuery.graphql",
    response_derives = "Debug"
)]
struct ContributionsQuery;

#[derive(Debug)]
pub struct GithubResponse {
    pub contributions: ContributionsQueryUserContributionsCollectionContributionCalendar,
    pub repositories: ContributionsQueryUserRepositories,
}

impl GithubResponse {
    pub fn query(username: String, token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let query =
            ContributionsQuery::build_query(contributions_query::Variables { login: username });

        let client = reqwest::Client::new();
        let mut resp = client
            .post("https://api.github.com/graphql")
            .bearer_auth(token)
            .json(&query)
            .send()?;

        let response_body: Response<contributions_query::ResponseData> = resp.json()?;
        let data = response_body.data.expect("missing data").user.unwrap();
        let contributions = data.contributions_collection.contribution_calendar;
        let repositories = data.repositories;

        Ok(GithubResponse {
            contributions,
            repositories,
        })
    }
}

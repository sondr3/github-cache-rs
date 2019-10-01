use dotenv::dotenv;
use graphql_client::{GraphQLQuery, Response};
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.json",
    query_path = "queries/ContributionsQuery.graphql",
    response_derives = "Debug"
)]
struct ContributionsQuery;

#[derive(Deserialize, Debug)]
struct Config {
    token: String,
    username: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config: Config = match envy::from_env() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", config);

    let query = ContributionsQuery::build_query(contributions_query::Variables {
        login: config.username,
    });

    let client = reqwest::Client::new();
    let mut resp = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.token)
        .json(&query)
        .send()?;

    let response_body: Response<contributions_query::ResponseData> = resp.json()?;
    let data = response_body.data.expect("missing data");

    println!("{:#?}", data);

    Ok(())
}

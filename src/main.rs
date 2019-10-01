use dotenv::dotenv;
use graphql_client::{GraphQLQuery, Response};
use std::env;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/github-schema.graphql",
    query_path = "queries/ContributionsQuery.graphql",
    response_derives = "Debug"
)]
pub struct ContributionsQuery;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = match env::var("TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("No token available!"),
    };

    let username = match env::var("USERNAME") {
        Ok(val) => val,
        Err(_) => panic!("No token available!"),
    };

    println!("{:?}", token);

    let query = ContributionsQuery::build_query(contributions_query::Variables { login: username });

    let client = reqwest::Client::new();
    let mut resp = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&query)
        .send()?;

    let response_body: Response<contributions_query::ResponseData> = resp.json()?;
    println!("{:#?}", response_body);

    Ok(())
}

mod github;

use dotenv::dotenv;
use github::{Contribution, User, Week};
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
    let data = response_body.data.expect("missing data").user.unwrap();
    let contributions = data.contributions_collection.contribution_calendar;
    let repositories = &data.repositories;

    let mut user = User::new();

    user.contributions.total_contributions = contributions.total_contributions;
    for color in &mut contributions.colors.into_iter() {
        user.contributions.colors.push(color);
    }

    for (i, week) in &mut contributions.weeks.into_iter().enumerate() {
        let mut w = Week::new();

        for (d, contrib) in week.contribution_days.iter().enumerate() {
            w.days.insert(
                d,
                Contribution::new(contrib.contribution_count, contrib.color.to_owned()),
            );
        }
        user.contributions.weeks.insert(i, w);
    }

    for node in &mut repositories
        .nodes
        .as_ref()
        .expect("Missing repository nodes")
        .iter()
    {
        if let Some(node) = node {
            user.repositories
                .insert(node.name.to_owned(), node.stargazers.total_count);
        }
    }

    println!("{:#?}", user);

    Ok(())
}

#![feature(proc_macro_hygiene, decl_macro)]

mod github;
mod query;
mod server;

#[macro_use]
extern crate rocket;

use crate::query::GithubResponse;
use dotenv::dotenv;
use github::User;
use serde::Deserialize;
use std::sync::Mutex;

type AppState = Mutex<User>;

impl AppState {
    fn new() -> Self {}
}

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

    let response = GithubResponse::query(config.username, config.token)?;
    let user = User::from_response(response);

    println!("{:#?}", user);

    server::run();

    Ok(())
}

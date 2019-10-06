use crate::{
    github::{ContributionCollection, Repositories},
    Config, GithubResponse, User,
};
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type AppState = Arc<Mutex<User>>;

#[get("/")]
fn index(user: State<AppState>) -> Json<User> {
    let data = user.lock().unwrap().clone();
    Json(data)
}

#[get("/contributions")]
fn contributions(user: State<AppState>) -> Json<ContributionCollection> {
    let data = user.lock().unwrap().clone();
    Json(data.contributions)
}

#[get("/repositories")]
fn repositories(user: State<AppState>) -> Json<Repositories> {
    let data = user.lock().unwrap().clone();
    Json(data.repositories)
}

fn get_user(config: &Config) -> User {
    let response =
        GithubResponse::query(&config.username, &config.token).expect("GitHub API error.");
    User::from_response(response)
}

pub fn run(config: Config) {
    let (sender, receiver) = channel();
    let config_clone = config.clone();

    thread::spawn(move || {
        rocket::ignite()
            .manage(AppState::new(Mutex::new(get_user(&config_clone))))
            .mount("/", routes![index, contributions, repositories])
            .launch();
    });

    thread::spawn(move || loop {
        sender
            .send(get_user(&config))
            .expect("Could not send to thread");
        thread::sleep(Duration::from_secs(3600));
    });

    loop {
        let _ = receiver.try_recv();
    }
}

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let token = match env::var("TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("No token available!")
    };

    println!("{:?}", token);
}

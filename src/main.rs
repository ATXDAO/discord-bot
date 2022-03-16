use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{gateway::Ready},
    framework::{standard::macros::group, StandardFramework},
    prelude::*,
};
use std::env;
mod commands; 
use commands::{math::*, help::*};



// takes commands and creates new variables
#[group]
#[commands(multiply, help, joke)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let framework =
    StandardFramework::new().configure(|c| c.prefix("~")).group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

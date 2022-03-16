use serenity::framework::standard::{macros::command, CommandResult, CommandError, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Serialize)]
struct Response {
    jokes: Vec<Joke>,
}
#[derive(Deserialize, Debug, Serialize)]
struct Joke {
    id: u32,
    delivery: String,
    setup: String,
}

impl fmt::Display for Joke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}.....{:?}", self.setup, self.delivery)
    }
}

// used to impl the fmt::Display to be able to send in the serenity::say function
impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.jokes)
    }
}

impl Response {
    pub fn name(&self, msg: &Message) -> String {
       format!("Hey there {}! Here is the info you requested...

       ", msg.author.name)
    }
}

fn help_message(msg: &Message) -> String {
    format!("Hello there, {}!
Let's see about getting you what you need.
❓ Need technical help?
➡️ Post in the <#CHANNEL_ID> channel and other members can help you.
— ATXDAO Bot 🤖
", msg.author.name)
}


#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, help_message(msg)).await?;

    Ok(())
}

// 
async fn call(num: u32) -> Result<Response, CommandError> {
    let client = reqwest::Client::new();
    if num > 1 {
        let res = client.get(format!("https://v2.jokeapi.dev/joke/Any?amount={}", num))
            .send()
            .await?
            .json::<Response>()
            .await?;
            Ok(res)
    } else {
        let res = client.get("https://v2.jokeapi.dev/joke/Any")
            .send()
            .await?
            .json::<Joke>()
            .await?;
        
        let r = Response{jokes: vec![res]};
        Ok(r)
    }
    
}

#[command]
async fn joke(ctx: &Context, msg: &Message, mut arg: Args) -> CommandResult {
    let num = arg.single::<u32>()?;
    if num > 10 {
        msg.reply(ctx, "Chill, that's too many jokes").await?;
        return Ok(())
    }
    let res = match call(num).await {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            msg.reply(ctx, "Error running command, please try again").await?;
            return Ok(())
        },
    };
    msg.channel_id.say(&ctx.http, res.name(msg)).await?;
    for x in res.jokes.iter() {
        msg.channel_id.say(&ctx.http, x).await?;
    }
    
    Ok(())
}
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!
You have summoned me. Let's see about getting you what you need.
❓ Need technical help?
➡️ Post in the <#977300234046672956> channel and other humans will assist you.
❓ Need to do general texting ?
➡️ Post in the <#977355986400579584> channel and other humans will assist you.
❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>
❓ Something wrong?
➡️ You can flag an admin with @admin
I hope that resolves your issue!
— HelpBot 🤖
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("scanning message: {:?}", msg);
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    
    // BUILD & RUN COMMANDS:
    // 1. cargo build
    // 2. cargo run -- <TOKEN_HERE>

    // // token for discord api (pass it as cmd arg)
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    println!("Searching for {}", query);
    env::set_var("DISCORD_TOKEN", query);

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// fn main()
// {
//     let args: Vec<String> = env::args().collect();

//     for (key, value) in std::env::vars() {        
//         println!("In file, run time:  {} : {}", key, value);
//     }

//     let query = &args[1];
    
//     println!("Searching for {}", query);
    
//     env::set_var("CARGO_TOKEN", query);

//     let tkn = env::var("CARGO_TOKEN").expect("CARGO_TOKEN  expected!");
//     println!("compile-time: CARGO_BIN_EXE_cargo-env: {}", tkn);
//     for (key, value) in std::env::vars() {
//         if key.starts_with("CARGO_") {
//             println!("runtime: {}: {}", key, value);
//         }
//     }
// }

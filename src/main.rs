use dotenv_codegen::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;

use tracing::info;

mod commands;

use commands::ping::*;
use commands::touhou::*;

#[group]
#[commands(ping, touhou)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = dotenv!("DISCORD_TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("An error ocurred while running the client: {:?}", why);
    }
}
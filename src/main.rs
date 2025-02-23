use std::env;

use rand::{
    distr::Uniform,
    rngs::{StdRng, ThreadRng},
    seq::{IndexedRandom, IteratorRandom},
    Rng, SeedableRng,
};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use strum::IntoEnumIterator;
use tantan::{TanTanPrompts, Tantan};

struct Handler {
    rng: Mutex<StdRng>,
}
mod tantan;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name == "OpenTanTanBeta" {
            return;
        }

        // Goes through every prompt per enum and matches it with msg content
        for prompt_enum in TanTanPrompts::iter() {
            if prompt_enum.get_prompts().r#match(msg.content.clone()) {
                // Take rng from handler
                let mut rng = self.rng.lock().await;

                // Reply with random valid responce
                if let Err(why) = msg
                    .channel_id
                    // Replace any '{}' with username of sender
                    .say(
                        &ctx.http,
                        prompt_enum
                            .get_replies()
                            .choose(&mut rng)
                            .map(|reply| reply.replace("{}", &msg.author.name))
                            .unwrap_or_else(|| "Err qwq".to_string()),
                    )
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Sets up tracing
    tracing_subscriber::fmt::init();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler {
            rng: Mutex::new(StdRng::from_os_rng()),
        })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

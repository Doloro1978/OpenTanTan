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
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }

        for prompt_enum in TanTanPrompts::iter() {
            for prompt in prompt_enum.get_prompts().into_iter() {
                if msg.content.contains(prompt) {
                    let mut rng = self.rng.lock().await;

                    if let Err(why) = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            prompt_enum
                                .get_replies()
                                .choose(&mut rng)
                                .map(|reply| reply.replace("{}", &msg.author.name))
                                .unwrap_or_else(|| "Err".to_string()),
                        )
                        .await
                    {
                        println!("Error sending message: {why:?}");
                    }
                    return;
                }
            }
        }
        ()
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = "";
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
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

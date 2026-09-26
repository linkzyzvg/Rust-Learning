use poise::serenity_prelude as serenity;
use std::env;

mod events;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework: poise::Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![],
            event_handler: |_ctx, event| {
                Box::pin(async move {
                    // Pattern match specifically for the Ready gateway event
                    if let serenity::FullEvent::Ready { data_about_bot } = event {
                        events::ready::handle_ready(data_about_bot);
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let lazarus = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    lazarus.unwrap().start().await.unwrap();
}

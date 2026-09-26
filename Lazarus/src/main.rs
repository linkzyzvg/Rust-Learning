use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework: poise::Framework<Data, Error> = poise::Framework::builder() // <-- Add your types here
        .options(poise::FrameworkOptions {
            commands: vec![],
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

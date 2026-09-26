use poise::serenity_prelude as serenity;

pub fn handle_ready(ready: &serenity::Ready) {
    let lazarus = &ready.user.name;

    // Serenity provides discriminator as a u16 or a fallback string depending on your crate version
    let discrim = ready.user.discriminator.map(|d| d.get()).unwrap_or(0);

    // Serenity presents shard info as an Option<[u32; 2]>: [shard_index, shard_total]
    let (shard_index, shard_total) = ready.shard.map(|s| (s.id.0, s.total)).unwrap_or((0, 1));

    let guild_count = ready.guilds.len();

    let cyan = "\x1b[36m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let gray = "\x1b[90m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!(
        r#"{cyan}
 ╭──────────────────────────────────────────────────────────────────────────╮
 │ {bold}{green}✔{reset}{cyan}  Lazarus LIVE & CONNECTED TO DISCORD GATEWAY{reset}{cyan}                  │
 ╰──────────────────────────────────────────────────────────────────────────╯
  {gray}➔{reset} {bold}Identity:{reset}      {green}{lazarus}#{discrim:04}{reset}
  {gray}➔{reset} {bold}Guilds:{reset}        {yellow}{guild_count}{reset} active servers cached
  {gray}➔{reset} {bold}Sharding:{reset}      Allocation monitoring {cyan}[Shard #{shard_index} / {shard_total:02}]{reset}
  {gray}➔{reset} {bold}Environment:{reset}   Rust Architecture TLS Runtime Active
  {gray}──────────────────────────────────────────────────────────────────────────{reset}
"#
    );
}

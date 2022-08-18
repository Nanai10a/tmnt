#![feature(decl_macro)]

use chrono::{FixedOffset, TimeZone};
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let offset: FixedOffset = FixedOffset::east(9 * 60 * 60);

    let start = offset.ymd(2022, 8, 12);
    let end = offset.ymd(2022, 9, 25);
    let today = today!(offset);

    let all = (end - start).num_days();
    let progress = (today - start).num_days();
    let remaining = (end - today).num_days();

    if remaining < 0 {
        eprintln!("gone 2022 summer vacation.");
        std::process::exit(1);
    }

    let token = env!("SECRET_TOKEN");
    let url = env!("SECRET_URL");
    let http = Http::new(&token);
    let webhook = try_in_place!(
        Webhook::from_url(&http, &url).await,
        "cannot create webhook instance."
    );

    let embed = Embed::fake(|ce| {
        ce.title("此方, 日付変更線")
            .description(format!(
                "本日は夏休み{progress}日目, 残り**{remaining}日**だ！"
            ))
            .timestamp(today.and_hms(0, 0, 0).to_rfc3339())
            .colour(color!(progress as f32 / all as f32))
            .footer(|cef| cef.text("お前の二度寝の数を数えろ"))
    });

    try_in_place!(
        webhook
            .execute(http, false, |ew| ew.embeds(vec![embed]))
            .await,
        "failed post webhook message."
    );
}

macro today($offset:expr) {
    $offset
        .from_local_date(&::chrono::Local::now().date_naive())
        .single()
        .expect("ambiguos date")
}

macro env($key:expr) {{
    let key = $key;
    try_in_place!(
        ::std::env::var(key),
        "cannot interpret environment value (${key})."
    )
}}

macro try_in_place($expr:expr, $msg:expr) {
    if let Ok(o) = $expr {
        o
    } else {
        eprintln!($msg);
        ::std::process::exit(1);
    }
}

macro color($percentage:expr) {{
    use serenity::utils::Colour;
    let from = Colour::BLUE;
    let to = Colour::DARK_ORANGE;

    let diff = (
        to.r() as i16 - from.r() as i16,
        to.g() as i16 - from.g() as i16,
        to.b() as i16 - from.b() as i16,
    );

    let changes = (
        (diff.0 as f32 * $percentage).floor() as i16,
        (diff.1 as f32 * $percentage).floor() as i16,
        (diff.2 as f32 * $percentage).floor() as i16,
    );

    let current = Colour::from_rgb(
        (from.r() as i16 + changes.0) as _,
        (from.g() as i16 + changes.1) as _,
        (from.b() as i16 + changes.2) as _,
    );

    current
}}

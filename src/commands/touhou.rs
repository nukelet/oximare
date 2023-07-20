use std::str;

use serenity::prelude::*;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use rand::thread_rng;
use rand::Rng;
use rand::seq::IteratorRandom;
use reqwest::Client;
use roxmltree::Document;

#[command]
pub async fn touhou(ctx: &Context, msg: &Message) -> CommandResult {
    let api_url = "https://safebooru.org/index.php?page=dapi&s=post&q=index";
    let search_limit: u8 = 40;
    let page_number: u8 = {
        let mut rng = thread_rng();
        rng.gen::<u8>() % search_limit
    };

    let client = Client::new();
    let request = client.get(api_url)
        .query(&[
            ("limit", &search_limit.to_string()),
            ("tags", &"touhou".to_string()),
            ("pid", &page_number.to_string()),
        ])
        .build()
        .unwrap();

    
    // the API replies with a bunch of entries matching our query
    let response = client.execute(request).await?.bytes().await?;
    let data = str::from_utf8(&response)?;
    let root = Document::parse(&data)?;
    // randomly select an url from our list of entries
    let resource_url = {
        let mut rng = thread_rng();
        root.descendants()
            .filter_map(|c| c.attribute("file_url"))
            .choose(&mut rng).unwrap()
    };

    msg.reply(&ctx.http, resource_url).await?;

    Ok(())
}
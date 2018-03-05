use diesel::prelude::*;
use chrono::prelude::*;
use rss::{ChannelBuilder, ItemBuilder, EnclosureBuilder, Item};
use rss::extension::itunes::{ITunesItemExtensionBuilder};

use rocket::response::Failure;
use rocket::http::Status;

//use lib::database::DB;
use database::DbConn;

use config::Config;
use lib::models::*;

#[get("/audiobook_rss/<slug>")]
pub fn get_audiobook_rss(slug: String, conn: DbConn, config: Config) -> Result<String, Failure> {
    let audiobook = audiobook::table
        .filter(audiobook::slug.eq(slug))
        .load::<AudioBook>(&*conn);

    // check if we could find the book
    match audiobook {
        Ok(mut audiobook) => {
            if let Some(audiobook) = audiobook.pop() {
                // found it, build rss feed
                build_channel(audiobook, conn, config)
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}

fn build_channel(audiobook: AudioBook, conn: DbConn, config: Config) -> Result<String, Failure> {
    let mut channel = ChannelBuilder::default();

    let series = series::table
        .find(audiobook.series_id)
        .load::<Series>(&*conn).unwrap()
        .pop().unwrap();

    channel
        .title(audiobook.title.clone())
        .link(format!("{}/audiobook_rss/{}", config.path.external_url, audiobook.slug))
        .language(series.translation.clone())
        .description(audiobook.description.clone().unwrap_or(audiobook.title.clone()));
    
    let author = author::table
        .find(series.author_id)
        .load::<Author>(&*conn)
        .unwrap().pop().unwrap();

    let parts = part::table
        .filter(part::audiobook_id.eq(audiobook.id))
        .order(part::import_date.asc())
        .load::<Part>(&*conn).unwrap();

    let items: Vec<Item> = parts.iter().map(|ref pt| {
        let mut item = ItemBuilder::default();
        let mut ext = ITunesItemExtensionBuilder::default();
        let mut enc = EnclosureBuilder::default();

        ext
            .author(author.name.clone())
            .duration(NaiveTime::from_num_seconds_from_midnight(pt.duration as u32, 0).format("%H:%M:%S").to_string());

        let url = format!("{}/part/{}", config.path.external_url, pt.id);
        let mut mime = "application/octet-stream";
        if pt.file_name.ends_with(".m4a") || pt.file_name.ends_with(".m4b") || pt.file_name.ends_with(".mp4") {
            mime = "audio/mp4";
        }
        if pt.file_name.ends_with(".mp3") {
            mime = "audio/mpeg";
        }

        enc
            .url(url)
            .length(pt.file_size.to_string())
            .mime_type(String::from(mime));

        item
            .title(audiobook.title.clone())
            .author(author.name.clone())
            .pub_date(pt.import_date.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .itunes_ext(ext.build().unwrap())
            .enclosure(enc.build().unwrap());

        if let Some(ref desc) = audiobook.description {
            item.description(desc.clone());
        }

        item.build().unwrap()
    }).collect();

    channel.items(items);

    Ok(channel.build().unwrap().to_string())
}

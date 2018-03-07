use std::collections::HashSet;

use diesel::prelude::*;
use chrono::prelude::*;
use rss::{ChannelBuilder, ItemBuilder, EnclosureBuilder, ImageBuilder, Item};
use rss::extension::itunes::{ITunesItemExtensionBuilder, ITunesChannelExtensionBuilder};
use rss::extension::dublincore::DublinCoreExtensionBuilder;

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

// TODO: clean up channel building and merge with /series_rss/<slug>
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

    let speaker = speaker::table
        .find(audiobook.speaker_id)
        .load::<Speaker>(&*conn)
        .unwrap().pop().unwrap();

    let parts = part::table
        .filter(part::audiobook_id.eq(audiobook.id))
        .order(part::import_date.asc())
        .load::<Part>(&*conn).unwrap();

    let mut contributors = HashSet::<String>::new();
    contributors.insert(author.name.clone());
    contributors.insert(speaker.name.clone());

    let mut image = ImageBuilder::default();

    image
        .url(format!("{}/cover/{}.jpg", config.path.external_url, parts[0].id));
    
    let items: Vec<Item> = parts.iter().map(|ref pt| {
        let mut item = ItemBuilder::default();
        let mut ext = ITunesItemExtensionBuilder::default();
        let mut enc = EnclosureBuilder::default();

        ext
            .author(author.name.clone())
            .image(format!("{}/cover/{}.jpg", config.path.external_url, pt.id))
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
            .pub_date(pt.import_date.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
            .itunes_ext(ext.build().unwrap())
            .enclosure(enc.build().unwrap());

        if let Some(ref desc) = audiobook.description {
            item.description(desc.clone());
        }

        item.build().unwrap()
    }).collect();

    let contributors: Vec<String> = contributors.iter().map(|s| s.clone()).collect();
    let mut dc = DublinCoreExtensionBuilder::default();
    dc.contributors(contributors);

    channel.dublin_core_ext(dc.build().unwrap());
    channel.image(image.build().unwrap());
    channel.items(items);

    let mut channel_ext = ITunesChannelExtensionBuilder::default();
    channel_ext
        .author(author.name.clone());
    channel.itunes_ext(channel_ext.build().unwrap());

    Ok(channel.build().unwrap().to_string())
}

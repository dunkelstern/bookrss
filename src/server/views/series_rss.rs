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

#[get("/series_rss/<slug>")]
pub fn get_series_rss(slug: String, conn: DbConn, config: Config) -> Result<String, Failure> {
    let series = series::table
        .filter(series::slug.eq(slug))
        .load::<Series>(&*conn);

    // check if we could find the book
    match series {
        Ok(mut series) => {
            if let Some(series) = series.pop() {
                // found it, build rss feed
                build_channel(series, conn, config)
            } else {
                // not found
                Err(Failure(Status::NotFound))
            }
        },
        // DB error
        Err(_) => Err(Failure(Status::ServiceUnavailable))
    }
}

// TODO: clean up channel building and merge with /audiobook_rss/<slug>
fn build_channel(series: Series, conn: DbConn, config: Config) -> Result<String, Failure> {
    let mut channel = ChannelBuilder::default();

    channel
        .title(series.title.clone())
        .link(format!("{}/series_rss/{}", config.path.external_url, series.slug))
        .language(series.translation.clone())
        .description(series.description.unwrap_or(series.title));

    let books = audiobook::table
        .filter(audiobook::series_id.eq(series.id))
        .order(audiobook::part_no.asc())
        .load::<AudioBook>(&*conn);

    if let Err(_err) = books {
        return Err(Failure(Status::NotFound));
    }
    let books = books.unwrap();
    
    let author = author::table
        .find(series.author_id)
        .load::<Author>(&*conn)
        .unwrap().pop().unwrap();

    let mut contributors = HashSet::<String>::new();
    contributors.insert(author.name.clone());

    let mut image = ImageBuilder::default();

    let items: Vec<Item> = books.iter().map(|ref book| {
        let speaker = speaker::table
            .find(book.speaker_id)
            .load::<Speaker>(&*conn)
            .unwrap().pop().unwrap();
        
        contributors.insert(speaker.name.clone());

        let parts = part::table
            .filter(part::audiobook_id.eq(book.id))
            .order(part::import_date.asc())
            .load::<Part>(&*conn).unwrap();

        image
            .url(format!("{}/cover/{}.jpg", config.path.external_url, parts[0].id));
        
        parts.iter().map(|ref pt| {
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
                .title(book.title.clone())
                .author(author.name.clone())
                .pub_date(pt.import_date.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
                .itunes_ext(ext.build().unwrap())
                .enclosure(enc.build().unwrap());

            if let Some(ref desc) = book.description {
                item.description(desc.clone());
            }

            item.build().unwrap()
        }).collect()
    }).flat_map(|x: Vec<Item>| x).collect();

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

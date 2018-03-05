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

#[get("/series_rss/<id>")]
pub fn get_series_rss(id: i32, conn: DbConn, config: Config) -> Result<String, Failure> {
    let series = series::table
        .find(id)
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

fn build_channel(series: Series, conn: DbConn, config: Config) -> Result<String, Failure> {
    let mut channel = ChannelBuilder::default();

    channel
        .title(series.title.clone())
        .link(config.path.external_url.clone())
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

    let items: Vec<Item> = books.iter().map(|ref book| {
        let parts = part::table
            .filter(part::audiobook_id.eq(book.id))
            .order(part::import_date.asc())
            .load::<Part>(&*conn).unwrap();
        
        parts.iter().map(|ref pt| {
            let mut item = ItemBuilder::default();
            let mut ext = ITunesItemExtensionBuilder::default();
            let mut enc = EnclosureBuilder::default();

            ext
                .author(author.name.clone())
                .duration(NaiveTime::from_num_seconds_from_midnight(pt.duration as u32, 0).format("%H:%M:%S").to_string());

            // FIXME: generate correct url
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
                .pub_date(pt.import_date.format("%Y-%m-%dT%H:%M:%SZ").to_string())
                .itunes_ext(ext.build().unwrap())
                .enclosure(enc.build().unwrap());

            if let Some(ref desc) = book.description {
                item.description(desc.clone());
            }

            item.build().unwrap()
        }).collect()
    }).flat_map(|x: Vec<Item>| x).collect();

    channel.items(items);

    Ok(channel.build().unwrap().to_string())
}

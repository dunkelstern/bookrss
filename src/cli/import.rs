use std::path::{Path, PathBuf};
use std::fs;
use std::str::FromStr;


use chrono::prelude::*;
use clap::ArgMatches;
use lib::settings::Settings;
use diesel::prelude::*;
use diesel::insert_into;

use ffmpeg::{identify, de_aax, convert, MediaInfo};
use database::get_db_conn;
use lib::models::*;

/// Import command, reads metadata from file, copies file into managed 
/// data directory and inserts information into DB
pub fn import(settings: Settings, args: &ArgMatches) {
    let filename = Path::new(args.value_of("INPUT").unwrap());
    println!("Importing {:?}", filename);

    // Identify file
    let info = identify(filename);

    match info {
        // Identification succcess, copy to data dir
        Ok(info) => {
            match convert_or_copy(filename, &settings, &info) {
                // copy or conversion success, add to database
                Ok(filename) => save_to_db(&filename, &info, &args, &settings),

                // print error message
                Err(err) => println!("Error: {}", err),
            }
        }

        // print error message
        Err(err) => {
            println!("Error parsing metadata: {:?}", err);
        }
    }
}

/// Either converts/decodes/de-DRMs a media file or just copies it
/// into the managed data directory
fn convert_or_copy(filename: &Path, settings: &Settings, info: &MediaInfo) -> Result<Box<PathBuf>, &'static str> {
    let mut imported_filename = Path::new(&settings.path.data_path).join(filename.file_name().unwrap());

    println!(" -> Converting/copying file");

    // if this is an audible aax file, remove the DRM
    if info.format == "aax" {
        // change the extension to `m4a`
        imported_filename = imported_filename.with_extension("m4a");

        // try to load activation bytes from config
        if let Some(ref audible_settings) = settings.audible {
            
            // remove the DRM
            let result = de_aax(filename, &imported_filename, &audible_settings.activation_bytes);
            if result.success() {
                Ok(Box::new(imported_filename))
            } else {
                Err("could not convert file")
            }
        } else {
            // No activation bytes in config, inform user
            Err("please configure audible = {{ activation_bytes = \"<hex>\" }}")
        }
    } else if info.format == "aa" {
        // change the extension to `mp3`
        imported_filename = imported_filename.with_extension("mp3");        

        // convert file
        let result = convert(filename, &imported_filename, "copy");
        if result.success() {
            Ok(Box::new(imported_filename))
        } else {
            Err("could not convert file")
        }
    } else {
        // try copying the file into the data directory
        if let Ok(_) = fs::copy(filename, &imported_filename) {
            Ok(Box::new(imported_filename))
        } else {
            // something went wrong, inform user
            Err("could not copy file")
        }
    }
}

/// Save metadata to database
fn save_to_db(filename: &Path, info: &MediaInfo, args: &ArgMatches, settings: &Settings) {
    println!(" -> Creating DB entries");

    let conn = get_db_conn(settings);
    let part_no = i32::from_str(args.value_of("PART").unwrap_or("0")).unwrap_or(0);
    let split = args.is_present("SPLIT");
    let series_name = String::from(args.value_of("SERIES").unwrap());

    // get or create author
    let mut author_result = author::table
        .filter(author::name.like(&info.author))
        .load::<Author>(&*conn)
        .unwrap();

    if author_result.len() == 0 {
        let new_author = NewAuthor {
            language: String::from("unknown"),
            name: info.author.clone(),
        };

        let _ = insert_into(author::table)
            .values(&new_author)
            .execute(&*conn)
            .unwrap();

        author_result = author::table
            .filter(author::name.like(&info.author))
            .load::<Author>(&*conn)
            .unwrap();
    }

    // get or create speaker
    let mut speaker_result = speaker::table
        .filter(speaker::name.like(&info.narrator))
        .load::<Speaker>(&*conn)
        .unwrap();

    if speaker_result.len() == 0 {
        let new_speaker = NewSpeaker {
            language: String::from("unknown"),
            name: info.narrator.clone(),
        };

        let _ = insert_into(speaker::table)
            .values(&new_speaker)
            .execute(&*conn)
            .unwrap();

        speaker_result = speaker::table
            .filter(speaker::name.like(&info.narrator))
            .load::<Speaker>(&*conn)
            .unwrap();
    }

    // get or create series
    let mut series_result = series::table
        .filter(series::title.like(&series_name))
        .load::<Series>(&*conn)
        .unwrap();

    if series_result.len() == 0 {
        let new_series = NewSeries {
            title: series_name.clone(),
            translation: String::from("unknown"),
            description: None,
            author_id: author_result[0].id,
        };

        let _ = insert_into(series::table)
            .values(&new_series)
            .execute(&*conn)
            .unwrap();

        series_result = series::table
            .filter(series::title.like(&series_name))
            .load::<Series>(&*conn)
            .unwrap();
    }

    // get or create audiobook
    let mut audiobook_result = audiobook::table
        .filter(audiobook::title.like(&info.title))
        .load::<AudioBook>(&*conn)
        .unwrap();

    if audiobook_result.len() == 0 {
        let new_audiobook = NewAudioBook {
            title: info.title.clone(),
            description: info.description.clone(),
            part_no: part_no,
            publish_date: info.date.clone(),
            speaker_id: speaker_result[0].id,
            series_id: series_result[0].id,
        };

        let _ = insert_into(audiobook::table)
            .values(&new_audiobook)
            .execute(&*conn)
            .unwrap();

        audiobook_result = audiobook::table
            .filter(audiobook::title.like(&info.title))
            .load::<AudioBook>(&*conn)
            .unwrap();
    }

    // create part
    let mut start_time: i32 = 0;
    if split {
        // find out which part this is
        let parts = part::table
            .filter(part::audiobook_id.eq(audiobook_result[0].id))
            .order(part::start_time.asc())
            .load::<Part>(&*conn)
            .unwrap();
        
        start_time = parts.iter().fold(0, |acc, ref item| acc + item.duration);
    }

    let new_part = NewPart {
        import_date: Local::now().naive_local(),
        file_name: filename.file_name().unwrap().to_string_lossy().to_string(),
        file_size: info.size,
        start_time,
        duration: info.duration,
        bit_rate: info.bit_rate,
        audiobook_id: audiobook_result[0].id,
    };

    let _ = insert_into(part::table)
        .values(&new_part)
        .execute(&*conn)
        .unwrap();

    println!(" -> Import finished");
}

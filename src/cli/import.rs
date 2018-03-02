use std::path::{Path, PathBuf};
use std::fs;

use clap::ArgMatches;
use lib::settings::Settings;

use ffmpeg::{identify, de_aax, MediaFormat};

/// Import command, reads metadata from file, copies file into managed 
/// data directory and inserts information into DB
pub fn import(settings: Settings, args: &ArgMatches) {
    let filename = Path::new(args.value_of("INPUT").unwrap());
    println!("Importing {:?}", filename);

    // Identify file
    let format = identify(filename);

    match format {
        // Identification succcess, copy to data dir
        Ok(format) => {
            match convert_or_copy(filename, settings, &format) {
                // copy or conversion success, add to database
                Ok(filename) => save_to_db(&filename, &format),

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
fn convert_or_copy(filename: &Path, settings: Settings, format: &MediaFormat) -> Result<Box<PathBuf>, &'static str> {
    let mut imported_filename = Path::new(&settings.path.data_path).join(filename.file_name().unwrap());

    println!(" -> {:?}", imported_filename);

    // if this is an audible aax file, remove the DRM
    match format.tags.major_brand {
        Some(ref t) if t == "aax " => {
            // change the extension to `m4a`
            imported_filename = imported_filename.with_extension("m4a");

            // try to load activation bytes from config
            if let Some(audible_settings) = settings.audible {
                
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
        }
        _ => {
            // try copying the file into the data directory
            if let Ok(_) = fs::copy(filename, &imported_filename) {
                Ok(Box::new(imported_filename))
            } else {
                // something went wrong, inform user
                Err("could not copy file")
            }
        }
    }
}

/// Save metadata to database
fn save_to_db(filename: &Path, format: &MediaFormat) {
    println!(" -> Creating DB entries");
}

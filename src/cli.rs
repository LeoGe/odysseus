use std::env;
use std::io::{Write, Read};
use std::fs;
use std::collections::HashSet;
use std::path::PathBuf;
use odysseus_lib::Store;
use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
    let matches =
    App::new("Odysseus command line interface for Zyklop.")
        .version("0.1")
        .author("Lorenz Schmidt, Leonie Geyer")
        .about("Controls the local music library")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("init")
            .about("Initialize a new music workspace")
            .arg(Arg::with_name("PATH")
                .help("Sets the input path")
                .required(true)
                .index(1))
        )
        .subcommand(SubCommand::with_name("list")
            .about("List all playlists in music library")
        )
        .subcommand(SubCommand::with_name("add")
            .about("Add music to the library")
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_match)) => {
            let full_path = sub_match.value_of("PATH").unwrap();
            // parse path
            let mut full_path = PathBuf::from(full_path.to_string());

            if !full_path.is_absolute() {
                full_path = PathBuf::from(env::current_dir().unwrap()).join(full_path);
            }

            if full_path.join("Music.toml").exists() {
                eprintln!(" => Workspace in {} already exists!", full_path.to_str().unwrap());
                return;
            }

            println!(" => Initialize workspace in {}", full_path.to_str().unwrap());

            // create root and files/ folder
            fs::create_dir_all(&full_path.join("files")).unwrap();

            // create Hex.toml file
            fs::File::create(&full_path.join("Hex.toml")).unwrap();

        },
        ("list", Some(_)) => {
            let store = Store::from_pwd().unwrap();
            for playlist in store.playlists() {
                println!(" => {}", playlist.name);
            }
        }
        ("add", Some(sub_match)) => {
            // look for folders which are not in toml
            // get all playlist names
            let store = Store::from_pwd().unwrap();
            let playlist_names: HashSet<String> = store.playlists()
                .iter().map(|pl| pl.name.clone())
                .collect();
            
            // get all folder names in pwd/files and filter out already known ones
            let music_files_path = store.root_path().join("files");
            let folder_names: HashSet<String> = std::fs::read_dir(music_files_path).unwrap()
                .into_iter()
                .filter_map(|x| x.ok())
                .map(|p| p.file_name())
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            dbg!(&playlist_names, &folder_names);
            let new_playlist_names = folder_names
                .difference(&playlist_names);

            dbg!(new_playlist_names);

            // add playlist entries in toml with folder names as name

            // open editor with part of toml including new playlist entries

            // on closing add and commit to git repo with predefined commit message
        }
        _ => {
        }
    }
}

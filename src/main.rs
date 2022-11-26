use rusqlite::{Connection, Result};
use lazy_static::lazy_static;
use regex::Regex;
use urlencoding::{encode, decode};
// use std::env;
// use std::path::Path;

#[derive(Debug)]
struct Entry {
    entry_url: String,
    entry_name: String,
    entry_shorturl: String,
    menu_name: String,
}

fn show_help() {
println!("
lzeditor 0.1.0
A Very simple Bookmarks database tool

USAGE:
    lzeditor [OPTIONS] <INPUT_DATA>

OPTIONS:
    -m     Create a new Menu Item
    -e     Create a new URL Entry
    -x     Export the current Database to a HTML File that can be read by any web browser

EXAPLES:
    lzeditor -m Videos          # Menu Item
    lzeditor -m Commedy Videos  # if two items, the first is hierarchically subordinate to the second (the second must exist)
    lzeditor -e https://youtu.be/xxxxxxxx 'URL Name' 'Videos'
    lzeditor -x '$HOME/Documents'\n");
}

fn create_database(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS menu(
            menu_name      TEXT PRIMARY KEY NOT NULL,
            parent_name    TEXT
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries(
            entry_url      TEXT PRIMARY KEY NOT NULL,
            entry_name     TEXT NOT NULL, 
            entry_shorturl TEXT NOT NULL,
            menu_name      TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

fn verify_url(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[[:alnum:]]{3,9}:[/]{2}.+[.].+").unwrap();
    }
    RE.is_match(text)
}




fn main() -> Result<()> {
    // Create Database if not exists
    let mut conn = Connection::open("lzdatabase.db")?;
    create_database(&mut conn)?;

    // if Path::new("lzdatabase.db").exists() {
    //     println!("Database seems to exist, Yea!");
    // }

    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        show_help();
        eprintln!("more arguments required");
        std::process::exit(1);
    }

    if args[1] == "-m" {
        if args.len() == 3 {
            conn.execute("INSERT INTO menu (menu_name) VALUES (?1)", &[&args[2]],)?;
        } else if args.len() == 4 {
            conn.execute("INSERT INTO menu (menu_name, parent_name) VALUES (?1, ?2)", (&args[2], &args[3]))?;
        }
    } else if args[1] == "-e" {
        let full = &args[2].as_str(); // Full URL
        if verify_url(full) {
            let items: Vec<&str> = full.split("/").collect();  // extract base from url, e.g. "youtube.com"
            let encoded_url = encode(&args[2]);
            let encoded_name = encode(&args[3]);

            let new_entry = Entry {
                entry_url: String::from(encoded_url),
                entry_name: String::from(encoded_name),
                entry_shorturl: items[2].to_string(),
                menu_name: String::from(&args[4]),
            };
            conn.execute(
                "INSERT INTO entries (entry_url, entry_name, entry_shorturl, menu_name) VALUES (?1, ?2, ?3, ?4)",
                (&new_entry.entry_url, &new_entry.entry_name, &new_entry.entry_shorturl, &new_entry.menu_name),
            )?;
        } else {
            println!("not a good url!");
        }
    } else if args[1] == "-x" {
        println!("Export function has not been implemented yet!");
        // let decoded = decode("%F0%9F%91%BE%20Exterminate%21")?;
        // println!("{}", decoded);
    } else {
        show_help();
    }

    


    // let full = "https://youtu.be/M-gBnHJA";
    // if verify_url(full) {
    //     let items: Vec<&str> = full.split("/").collect();  // extract base from url, e.g. "youtube.com"
    //     // println!("{}", items[2]);
    //     // for item in full.split("/") {
    //     //     println!("{}", item);
    //     // }

    //     let new_entry = Entry {
    //         entry_url: full.to_string(),
    //         entry_name: "Lunar Landing Mission | For All Kerbalkind Stream".to_string(),
    //         entry_shorturl: items[2].to_string(),
    //         menu_name: "Music".to_string(),
    //     };
    //     conn.execute(
    //         "INSERT INTO entries (entry_url, entry_name, entry_shorturl, menu_name) VALUES (?1, ?2, ?3, ?4)",
    //         (&new_entry.entry_url, &new_entry.entry_name, &new_entry.entry_shorturl, &new_entry.menu_name),
    //     )?;
    // } else {
    //     println!("not a good url!");
    // }


    // let s = "foo".to_string();   // to String
    // let s = String::from("foo"); // to String
    // let ss = s.as_str();         // from String to str


    println!("Program has reached it's end.");

    Ok(())
}
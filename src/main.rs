extern crate dotenv;
extern crate chrono;
use std::io;
use std::env;
use chrono::{DateTime, Local};
use postgres::{Client, NoTls, Error};

struct WhatIDid {
    _id: i32,
    what: String,
    when: String,
    start: String,
    end: String
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().expect(".env file not found");
    let user = env::var("DBUSER").unwrap();
    let pass = env::var("DBPASS").unwrap();
    let host = env::var("DBHOST").unwrap();
    let port = env::var("DBPORT").unwrap();
    let db = env::var("DBNAME").unwrap();
    let conn_str = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);
    let mut client = Client::connect(&conn_str, NoTls)?;

    let date: DateTime<Local> = Local::now();
    let when = date.format("%Y-%m-%d").to_string();

    let mut input = String::new();
    println!("Please, enter what you did:");
    io::stdin().read_line(&mut input).unwrap();
    let what: String = input.trim().parse().unwrap();
    input = "".to_string();

    println!("Please, enter start time:");
    io::stdin().read_line(&mut input).unwrap();
    let mut start: String = input.trim().parse().unwrap();
    input = "".to_string();
    start = format!("{}:{}", start, "00");
    if start.chars().count() < 3 {
        start = format!("{}:{}", start, "00:00");
    } else {
        start = format!("{}:{}", start, "00");
    }

    println!("Please, enter end time:");
    io::stdin().read_line(&mut input).unwrap();
    let mut end: String = input.trim().parse().unwrap();
    
    if end.chars().count() < 1 {
        end = date.format("%H:%M:%S").to_string();
    } else if end.chars().count() < 3 {
        end = format!("{}:{}", end, "00:00");
    } else {
        end = format!("{}:{}", end, "00");
    }
    
    
    println!("{} {} {}", when, start, end);

    let what_i_did = WhatIDid {
        _id: 0,
        what: what,
        when: when,
        start: start,
        end: end
    };
    
    client.execute(
        "INSERT INTO public.whatidid (what, \"when\", \"start\", \"end\") VALUES ($1, to_timestamp($2, 'YYYY-MM-DD'), to_timestamp($3, 'HH24:MI:SS'), to_timestamp($4, 'HH24:MI:SS'))", 
        &[&what_i_did.what, &what_i_did.when, &what_i_did.start, &what_i_did.end],
    )?;

    Ok(())
}
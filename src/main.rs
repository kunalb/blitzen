use std::error;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use chrono::{self, Datelike};
use chrono_tz::America::New_York;
use clap::{App, Arg, SubCommand};
use html2md::parse_html;
use regex::Regex;
use shellexpand::tilde;
use ureq;

const CACHE_DIR: &'static str = "~/.cache/bzn/";
const CONFIG_DIR: &'static str = "~/.config/bzn/";

fn fetch(agent: ureq::Agent, year: u32, day: u32) -> Result<(), Box<dyn error::Error>> {
    let cached = cache_path(year, day);
    if cached.exists() {
        let file = fs::File::open(cached)?;
        for line in BufReader::new(file).lines() {
            println!("{}", line?);
        }
        return Ok(());
    }

    let path = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = agent.get(&path).call();
    let resp_ok = resp.ok();

    eprintln!("Requesting {}: {:?}", path, resp);
    let mut result = resp.into_reader();
    let mut bytes = vec![];
    result.read_to_end(&mut bytes)?;

    if resp_ok {
        let mut file = fs::File::create(cached)?;
        file.write_all(&bytes)?;
    }

    print!("{}", String::from_utf8(bytes)?);
    Ok(())
}

fn get_session_key_path() -> Box<Path> {
    let mut path = PathBuf::new();
    path.push(tilde(CONFIG_DIR).to_string());
    path.push("session.conf");
    path.into_boxed_path()
}

fn get_session_key() -> Result<String, Box<dyn error::Error>> {
    match fs::read_to_string(get_session_key_path()) {
        Ok(val) if !val.trim().is_empty() => Ok(val),
        Ok(_) => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Empty session key",
        ))),
        Err(e) => Err(Box::new(e)),
    }
}

fn save_session_key(session_key: &str) -> Result<(), Box<dyn error::Error>> {
    Ok(fs::File::create(&get_session_key_path())?.write_all(session_key.as_bytes())?)
}

fn get_agent(session_key: &str) -> ureq::Agent {
    let agent = ureq::agent();
    agent.set_cookie(
        ureq::Cookie::build("session", String::from(session_key))
            .domain("adventofcode.com")
            .secure(true)
            .finish(),
    );
    agent.build();
    agent
}

fn cache_path(year: u32, day: u32) -> Box<Path> {
    let expanded = tilde(CACHE_DIR).to_string();
    let mut path = PathBuf::new();
    path.push(expanded);
    path.push(format!("{}_{}", year, day));
    path.into_boxed_path()
}

fn ensure_dir(dir: &str) -> Result<(), Box<dyn error::Error>> {
    let expanded = tilde(dir).to_string();
    let cache_dir = Path::new(&expanded);
    Ok(fs::create_dir_all(cache_dir)?)
}

fn submit(
    agent: ureq::Agent,
    year: u32,
    day: u32,
    level: u32,
) -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let path = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let resp = agent
        .post(&path)
        .send_form(&[("level", &level.to_string()), ("answer", &buffer)]);

    eprintln!("(Posting `{}` to {}: {:?})\n", buffer.trim(), path, resp);
    let re = Regex::new(r#"<main>((?s:.)*?)</main>"#)?;

    let response = resp.into_string()?;
    println!("{}", parse_html(re.find(&response).unwrap().as_str()));

    Ok(())
}

fn get_default_times() -> (u32, u32, u32) {
    let current_time = chrono::Utc::now();
    let ny_time = current_time.with_timezone(&New_York);
    (
        ny_time.year() as u32,
        ny_time.month() as u32,
        ny_time.day() as u32,
    )
}

fn main() -> Result<(), Box<dyn error::Error>> {
    ensure_dir(CACHE_DIR)?;
    ensure_dir(CONFIG_DIR)?;

    // Date-time arguments
    let (y, m, d) = get_default_times();
    let active_advent = m == 12 && d < 26;
    let y = y.to_string();
    let d = d.to_string();

    let arg_y = Arg::with_name("year").short("y").takes_value(true);
    let arg_d = Arg::with_name("day").short("d").takes_value(true);
    let (arg_y, arg_d) = if active_advent {
        (arg_y.default_value(&y), arg_d.default_value(&d))
    } else {
        (arg_y.required(true), arg_d.required(true))
    };

    // Session key
    let arg_s = Arg::with_name("session").short("s").takes_value(true);

    let session_key;
    let arg_s = match get_session_key() {
        Ok(x) => {
            session_key = x;
            arg_s.default_value(&session_key)
        }
        Err(_) => arg_s.required(true),
    };

    let matches = App::new("Bliten (bzn)")
        .version("0.1")
        .author("Kunal Bhalla <bhalla.kunal@gmail.com>")
        .about("Fetches inputs and submits results for Advent of Code")
        .arg(arg_s)
        .arg(arg_y)
        .arg(arg_d)
        .subcommand(SubCommand::with_name("fetch").about("Fetch inputs"))
        .subcommand(
            SubCommand::with_name("submit").about("Submit result").arg(
                Arg::with_name("level")
                    .short("l")
                    .takes_value(true)
                    .default_value("1"),
            ),
        )
        .get_matches();

    let year = matches.value_of("year").unwrap().parse::<u32>()?;
    let day = matches.value_of("day").unwrap().parse::<u32>()?;
    let session_key = matches.value_of("session").unwrap();
    if matches.is_present("session") {
        save_session_key(&session_key)?;
    }

    let agent = get_agent(session_key);

    if let Some(submit_matches) = matches.subcommand_matches("submit") {
        let level = submit_matches.value_of("level").unwrap().parse::<u32>()?;
        submit(agent, year, day, level)?;
    } else {
        fetch(agent, year, day)?
    }

    Ok(())
}

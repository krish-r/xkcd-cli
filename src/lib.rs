use clap::Parser;
use colored::*;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::File,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const URL: &str = "https://xkcd.com/info.0.json";

#[derive(Parser)]
#[command(name = "xkcd-cli")]
pub struct Cli {
    /// latest
    #[arg(short, long, group = "category")]
    pub latest: bool,

    /// random
    #[arg(short, long, group = "category")]
    pub random: bool,

    /// output
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn execute() {
    let cli = Cli::parse();

    let resp = match (cli.latest, cli.random) {
        (true, _) => fetch_latest_comic_info(URL),
        (_, true) => {
            let current_time_as_sec = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let max = fetch_latest_comic_info(URL).unwrap().num;
            fetch_random_comic_info(current_time_as_sec, max)
        }
        (false, false) => {
            eprintln!(
                "{}",
                "Expecting at least one argument. Check `--help`".red()
            );
            std::process::exit(1);
        }
    };

    let resp = match resp {
        Ok(r) => r,
        Err(err) => {
            // TODO: Handle err
            eprintln!("Encountered {}", err);
            std::process::exit(1);
        }
    };

    print_comic_info(&resp);

    let mut file = setup_output_dir(&resp.num.to_string(), cli.output);
    if let Err(err) = download_comic(&mut file, resp) {
        // TODO: Handle err
        eprintln!("Encountered {}", err);
        std::process::exit(1);
    }
}

fn fetch_latest_comic_info(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.json()?;
    Ok(resp)
}

fn fetch_random_comic_info(seed: u64, max: u32) -> Result<Response, Box<dyn std::error::Error>> {
    let mut rand_num = 0;
    while rand_num == 0 || rand_num == 404 {
        // rand_num = rand::thread_rng().gen_range(1..=max);
        rand_num = rand_chacha::ChaCha8Rng::seed_from_u64(seed).gen_range(1..=max);
    }
    fetch_latest_comic_info(&format!("https://xkcd.com/{}/info.0.json", rand_num))
}

fn setup_output_dir(fname: &str, custom_output_dir: Option<PathBuf>) -> File {
    let mut dir = std::env::current_dir().expect("Encountered error when fetching current_dir");
    dir.push("comics");
    let mut filename = format!("{}.png", fname);

    if let Some(d) = custom_output_dir {
        let file = &d.file_name().unwrap().to_str().unwrap();
        filename = file.to_string();
        dir = d.parent().unwrap().to_path_buf();
    }

    fs::create_dir_all(&dir).expect("Encountered error while creating comics dir");
    dir.push(filename);

    fs::File::create(dir.into_os_string()).unwrap()
}

fn print_comic_info(resp: &Response) {
    println!("{}", "Comic Info:".bold().green());
    println!("Comic #      : {}", resp.num.to_string().green());
    println!("Title        : {}", resp.safe_title.green());
    println!("Date (Y-M-D) : {}-{}-{}", resp.year, resp.month, resp.day);
    println!("Alt          : {}", resp.alt.cyan());
    println!("URL          : {}", resp.img);
}

fn download_comic(file: &mut File, resp: Response) -> Result<(), Box<dyn std::error::Error>> {
    reqwest::blocking::get(resp.img)?.copy_to(file)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Response {
    month: String,
    num: u32,
    year: String,
    safe_title: String,
    alt: String,
    img: String,
    day: String,
}

#[cfg(test)]
mod test {

    use super::*;

    impl Response {
        fn new(
            month: String,
            num: u32,
            year: String,
            safe_title: String,
            alt: String,
            img: String,
            day: String,
        ) -> Self {
            Response {
                month,
                num,
                year,
                safe_title,
                alt,
                img,
                day,
            }
        }
    }

    #[test]
    fn test_fetching_comic_info() {
        let url = "https://xkcd.com/1/info.0.json";
        let resp = fetch_latest_comic_info(url).unwrap();
        let expected_resp = Response::new(
            String::from("1"),
            1,
            String::from("2006"),
            String::from("Barrel - Part 1"),
            String::from("Don't we all."),
            String::from("https://imgs.xkcd.com/comics/barrel_cropped_(1).jpg"),
            String::from("1"),
        );
        assert_eq!(expected_resp, resp);
    }

    #[test]
    fn test_invalid_url() {
        let url = "";
        let resp = match fetch_latest_comic_info(url) {
            Ok(_) => panic!("Should not be Ok"),
            Err(e) => e.to_string(),
        };
        assert_eq!("builder error: relative URL without a base", resp);
    }

    #[test]
    fn test_fetching_random_comic_info() {
        let resp = fetch_random_comic_info(16, 1000).unwrap();
        let expected_resp = Response::new(
            String::from("7"),
            925,
            String::from("2011"),
            String::from("Cell Phones"),
            String::from("He holds the laptop like that on purpose, to make you cringe."),
            String::from("https://imgs.xkcd.com/comics/cell_phones.png"),
            String::from("15"),
        );
        assert_eq!(expected_resp, resp);
    }
}

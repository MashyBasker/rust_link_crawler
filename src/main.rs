use reqwest;
use tokio;
use reqwest::Error;
use scraper::{Html, Selector};
use std::{fs::{self, File}, io::{self, Write}};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let start_link = "https://en.wikipedia.org/wiki/Sherlock_(TV_series)";
    let res = reqwest::get(start_link)
        .await?
        .text()
        .await?;
    let links = get_links(res);
    let mut i = 1;
    let _ = write_to_file(i, links).unwrap();
    let mut depth = 1;
    let mut stop: bool = false;
    while !stop {
        let links_to_visit = read_from_file(depth);
        let mut resp;
        for l in links_to_visit {
            if l.starts_with("http") && !l.is_empty() {
                resp = reqwest::get(l.clone())
                    .await?
                    .text()
                    .await?;
                if l.contains("God") {
                    stop = true;
                    break;
                }
                i += 1;
                let links = get_links(resp);
                let _ = write_to_file(depth+1, links).unwrap();
            }
        }
        depth += 1;
        print!("NOW IN DEPTH {}", depth);
        println!("");
    }
    println!("{} links visited", i);
    Ok(())
}

fn get_links(frag: String) -> Vec<String> {
    let doc = Html::parse_document(&frag);
    let mut links: Vec<String> = Vec::new();
    let selector = Selector::parse("a").unwrap();
    for ele in doc.select(&selector) {
        if let Some(link) = ele.value().attr("href") {
            if link.starts_with("https") && !link.is_empty() {
                println!("Visiting {}", link);
                links.push(link.to_string());
            }
        }
    }
    links
}

fn write_to_file(depth: i16, links: Vec<String>) -> Result<(), io::Error> {
    let mut f = File::create(format!("./links/depth-{}.txt", depth))
        .expect("Err: Unable to create file");
    for l in links {
        write!(f, "{}\n", l)?;
    }
    Ok(())
}

fn read_from_file(depth: i16) -> Vec<String> {
    let f = format!("./links/depth-{}.txt", depth);
    fs::read_to_string(f)
        .expect("Err: Could not read file")
        .split("\n")
        .map(|x| x.to_string())
        .collect()
}

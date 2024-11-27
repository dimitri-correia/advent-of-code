use reqwest::Error;
use scraper::{Html, Selector};

use crate::utils::get_headers;

pub async fn fetch_scores() -> Result<(), Error> {
    let headers = get_headers();
    let url = "https://adventofcode.com/events".to_string();
    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers.clone())
        .send()
        .await?;
    let content = response.text().await?;

    let document = Html::parse_document(&content);
    let main_selector = Selector::parse("article").unwrap();
    let mut document = document.select(&main_selector);
    let article = document.next().unwrap();

    let mut markdown_content = String::new();
    markdown_content.push_str("# Advent of Code Stars\n\n");

    let mut nb_years = 0;

    let div_selector = Selector::parse("div").unwrap();
    let mut divs = article.select(&div_selector);
    while let Some(div) = divs.next() {
        let div = div.text().collect::<String>();
        let mut parts = div.split_whitespace();
        let parsed_year = parts
            .next()
            .unwrap()
            .trim_matches(|c| c == '[' || c == ']')
            .parse::<u32>()
            .unwrap();
        let parsed_star_count = parts
            .next()
            .unwrap_or("0*")
            .replace("*", "")
            .trim()
            .parse::<u32>()
            .unwrap();
        markdown_content.push_str(&format!("[![advent-of-code](https://img.shields.io/badge/Advent_of_Code-{}-F80046.svg)](https://adventofcode.com/{})\n", parsed_year, parsed_year));
        markdown_content.push_str(&format!(
            "{} ⭐️ ({}%)\n\n",
            parsed_star_count,
            100 * parsed_star_count / 50
        ));
        nb_years += 1;
    }

    let p_selector = Selector::parse("p").unwrap();
    let mut paragraphs = article.select(&p_selector);
    while let Some(paragraph) = paragraphs.next() {
        let paragraph = paragraph.text().collect::<String>();
        if paragraph.starts_with("Total stars") {
            let parsed_star_count = paragraph
                .trim()
                .split_whitespace()
                .last()
                .unwrap_or("0*")
                .replace("*", "")
                .trim()
                .parse::<u32>()
                .unwrap();
            markdown_content.push_str(&format!("## Total Stars\n\n"));
            markdown_content.push_str(&format!(
                "{} ⭐️ ({}%)\n\n",
                parsed_star_count,
                100 * parsed_star_count / (50 * nb_years)
            ));
        }
    }

    std::fs::write("Readme.md", markdown_content).unwrap();

    Ok(())
}

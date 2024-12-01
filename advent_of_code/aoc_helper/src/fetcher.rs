use reqwest::Error;
use scraper::{Html, Selector};
use std::process::Command;
use std::{fs::File, io::Write};

use crate::utils::get_headers;

pub async fn fetch_challenges() -> Result<(), Error> {
    for year in (2015..=2024).rev() {
        let dir_name = format!("advent_of_code/advent_of_code_y{}", year);
        if !std::fs::metadata(&dir_name).is_ok() {
            Command::new("cargo")
                .args(&["init", &dir_name, "--lib"])
                .output()
                .expect("Failed to execute cargo command");
        }

        if !std::fs::metadata(format!("{}/src/lib.rs", dir_name)).is_err() {
            let lib_file_path = format!("{}/src/lib.rs", dir_name);
            let mut lib_file = File::create(&lib_file_path).unwrap();
            for day in 1..=25 {
                let s = format!("pub mod day{:02};\n", day);
                lib_file.write_all(s.as_bytes()).unwrap();
            }
        }

        let headers = get_headers();

        for day in 1..=25 {
            let url = format!("{}/{}/day/{}", "https://adventofcode.com", year, day);
            let dir_path = format!("advent_of_code/advent_of_code_y{}/src/day{:02}", year, day);
            std::fs::create_dir_all(&dir_path).unwrap();

            let md_file_path = format!("{}/day{:02}.md", dir_path, day);
            let md_file = std::fs::read(md_file_path.clone());
            let part_2_finder = b"--- Part Two ---";
            if !md_file.is_err()
                && md_file
                    .unwrap()
                    .windows(part_2_finder.len())
                    .any(|w| w == part_2_finder)
            {
                continue;
            }

            println!("Fetching day {} of year {}", day, year);
            let response = reqwest::Client::new()
                .get(&url)
                .headers(headers.clone())
                .send()
                .await?;
            let content = response.text().await?;

            let document = Html::parse_document(&content);
            let main_selector = Selector::parse("article").unwrap();
            let mut document = document.select(&main_selector);
            let part_1 = document
                .next()
                .map(|element| element.inner_html())
                .unwrap_or_default();
            let mut main_content = part_1;
            let part_2 = document.next();
            if part_2.is_some() {
                let part_2 = part_2
                    .map(|element| element.inner_html())
                    .unwrap_or_default();
                main_content.push_str(&part_2);
            }
            let link_as_md = format!(
                "[Day {}](https://adventofcode.com/{}/day/{})",
                day, year, day
            );
            let markdown_content =
                format!("{} \n {}", link_as_md, html2md::parse_html(&main_content));

            std::fs::write(md_file_path, markdown_content).unwrap();

            let path_input_file = format!("{}/input1.txt", dir_path);
            if std::fs::metadata(&path_input_file).is_err()
                || std::fs::read(path_input_file)
                    .unwrap()
                    .starts_with("Please don't repeatedly request".as_bytes())
            {
                std::fs::write(format!("{}/input1_ex.txt", dir_path), "").unwrap();
                let input_url = format!("{}/input", url);
                let input_response = reqwest::Client::new()
                    .get(&input_url)
                    .headers(headers.clone())
                    .send()
                    .await?;
                let input_content = input_response.text().await?;
                std::fs::write(format!("{}/input1.txt", dir_path), input_content).unwrap();
            }
            if std::fs::metadata(format!("{}/mod.rs", dir_path)).is_err() {
                let template_dir = "advent_of_code/aoc_helper/template";
                let template_files = std::fs::read_dir(template_dir).unwrap();
                for file in template_files {
                    let file = file.unwrap();
                    let file_name = file.file_name();
                    let file_name = file_name.to_str().unwrap();
                    let src = format!("{}/{}", template_dir, file_name);
                    let dest = format!("{}/{}", dir_path, file_name);
                    std::fs::copy(src, dest).unwrap();
                }
            }
        }
    }
    Ok(())
}

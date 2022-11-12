use std::io;

extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

#[tokio::scrape]

fn main() {
    

//generate_message();
scrape();
    
}

fn generate_message() {
    println!("title + name + comma:");
    let mut researchername = String::new();

    io::stdin()
        .read_line(&mut researchername)
        .expect("Failed to read line");

    println!("paper name in quotes:");
    let mut papername = String::new();

    io::stdin()
        .read_line(&mut papername)
        .expect("Failed to read line");
    
    


    println!("Dear {researchername}");
    // println!("");
    println!("Greetings from Santa Barbara, California.");
    println!("My name is Nir Chemaya, and I’m one of the lead organizers for the DeFi Seminar at the Department of Economics at the University of California, Santa Barbara. ");
    println!("This seminar aims to build a DeFi-Econ community so researchers worldwide can share their work on DeFi-Crypto, and the economic questions that this new environment raises.");
    println!("");
    print!("We have recently encountered your work {papername}posted on SSRN, and we want to invite you to attend our seminar.");
    //print!("posted on SSRN, and we want to invite you to attend our seminar.");
    println!("");
    println!("Here’s our seminar website:");
    println!("https://ucsbdefi.wixsite.com/seminar.");
    println!("The seminar takes place virtually (over zoom) on Fridays at 9:30 am Pacific Time Zone (PT), and you can sign up to attend the seminar through our website.");
    println!("");
    println!("Please also feel free to share our website with friends and any relevant researchers. We look forward to hearing from you. Have a nice day.");
    println!("");
    println!("Best,");
    println!("Nir");
}

async fn scrape() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.wireshark.org/download/");
    let text = resp.text();

    let document = Html::parse_document(&text);
    let selector = Selector::parse(r#"table > tbody > tr > td > a"#).unwrap();
    for title in document.select(&selector) {
        println!("{}", resp.url().to_string());
        println!(
            "{}",
            title
                .value()
                .attr("href")
                .expect("href not found")
                .to_string()
        );
    }

    Ok(())
}
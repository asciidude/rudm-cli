extern crate reqwest;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    /* Disclaimer */
    println!("\r\n\r\n===> DISCLAIMER, READ THIS <===");
    println!("This project will NOT be updated and was simply made for fun and as an alternative to pydm");
    println!("Do NOT ask for help or support directly to the developer, I will not give any support.");
    println!("Have fun using the program, thank you! ~ use the positional argument 'disable-disclaimer' (boolean) to disable this message.\r\n\r\n");

    let url: &str = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf";

    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let mut file = File::create("./downloads/out.pdf").unwrap();

    println!("[DEBUG] Status: {0}", res.status());
    println!("[DEBUG] Headers: {:#?}", res.headers());
    
    writeln!(&mut file, "{0}", res.text().await?).unwrap();

    Ok(())
}
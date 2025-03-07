use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};


// a structs to work with the API response
#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

// a struct for the choices
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

// a struct for the request you will make to the API
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_token: u16, // more than words you want it to generate
}

// tokio to make async request
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync >> {
    // load my env variables
    dotenv().ok();
    // create a HttpConnector, hyper
    let https = HttpsConnector::new();
    // create a client
    let client = Client::builder().build(https);
    // URL to which we will make the request
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    // preamble, prompt to chat-gpt
    let preamble = "Generate a Sql code for the given statement";
    // token, in the header,
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);
    // loop, a way to read user input
    loop {
        println!(">");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");
            println!("");
    // spinner, wait for the response
        let sp = Spinner::new(&Spinners::Dots12, "\t\t OpenAi is Thinking...".into());
    // request to chatgpt for every user input
    let oai_request = OAIRequest {
        prompt: format!("{} {}", preamble, user_text),
        max_token: 1000,
    };

    let body = Body::from(serde_json::to_vec(&oai_request)?);
    let req = Request::post(uri)
                .header(header::CONTENT_TYPE, "application/json")
                .header("AUTHORIZATION", &auth_header_val)
                .body(body)
                .unwrap();
    // response and we print that response
    let res = client.request(req).await?;
    let body = hyper::body::aggregate(res).await?;
    let json: OAIResponse = serde_json::from_reader(body.reader())?;
    sp.stop();
    println!("");
    println!("{}", json.choices[0].text);
    }
    Ok(())
}

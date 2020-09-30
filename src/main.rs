use std::str::FromStr;
use std::sync::RwLock;
use std::fs::File;
use std::collections::HashMap;
use std::io::{Write, Read};
use std::io;

use actix_web::{get, web, App, HttpServer, HttpResponse};
use actix_web::http::{StatusCode, HeaderName, HeaderValue};

use lazy_static::lazy_static;
use std::ops::Deref;

const LOCATION: &str = "Location";
const PERSIST_FILE_PATH: &str = "./persist.json";

type URLMapType = HashMap<String, String>;
lazy_static! {
    static ref URL_MAP: RwLock<URLMapType> = RwLock::new(URLMapType::new());
}

fn read_map(key: String) -> Option<String> {
    let read_guard = URL_MAP.read().unwrap();
    let read_result = read_guard.get(key.as_str());

    return match read_result {
        Some(str) => {
            Some(str.to_string())
        }
        None => {
            None
        }
    };
}

fn write_map(key: String, value: String) {
    let mut write_guard = URL_MAP.write().unwrap();
    write_guard.insert(key, value);
    if let Err(e) = persist() {
        println!("err when persist {}", e);
    }
}

fn persist() -> io::Result<()> {
    let read_guard = URL_MAP.read().unwrap();
    let json_str = serde_json::to_string(read_guard.deref()).unwrap();
    let mut buffer = File::create(PERSIST_FILE_PATH)?;
    buffer.write_all(json_str.as_bytes())?;
    buffer.flush()?;

    Ok(())
}

fn read_persist() {
    match File::open(PERSIST_FILE_PATH) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let res = serde_json::from_slice::<URLMapType>(buffer.as_slice()).unwrap();

            let mut write_guard = URL_MAP.write().unwrap();
            *write_guard = res;
        }
        Err(e) => {
            println!("err when open persist file path {}", e);
        }
    }
}

#[get("/{id}")]
async fn index(web::Path(id): web::Path<String>) -> HttpResponse {
    let read_result = read_map(id.clone());
    if let Some(url) = read_result {
        println!("Map {}=>{}", id, url);

        let mut resp = HttpResponse::new(StatusCode::TEMPORARY_REDIRECT);
        let resp_header = resp.headers_mut();
        resp_header.insert(
            HeaderName::from_str(LOCATION).unwrap(),
            HeaderValue::from_str(url.as_str()).unwrap(),
        );

        resp
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    read_persist();

    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind("0.0.0.0:80")?
        .run()
        .await
}

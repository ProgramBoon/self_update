extern crate daemonize;

use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, ExitStatus};
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::io::Write;
use std::{fs, string};
use std::{thread, time};
use std::str;
use std::collections::HashMap;
use futures::executor::block_on;

use std::convert::Infallible;
use async_std;
//use futures::executor::ThreadPool;
use tokio::runtime::Handle;
use tokio::task;
use curl::easy::Easy;
use rocket::serde::de::Error;
// use error_chain::error_chain;
use self_update::cargo_crate_version;
use async_runtime::*;

// use pear::result::AsResult;

// error_chain! {
//      foreign_links {
//          HttpRequest(reqwest::Error);
//          IoError(::std::io::Error);
//      }
//  }
// #[tokio::main]




#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    update();
    // for stream in listener.incoming() {
    //     handle_client(stream?);


        // let uri = "http://localhost:8080/post/";
        // let data = serde_json::json!({ "name": "chashu", "data":"adad" });
        //
        // let res = surf::post(uri)
        //     .body(http_types::Body::from_json(&data)?)
        //     .await?;
        // assert_eq!(res.status(), http_types::StatusCode::Ok);

        // println!("{}",global_data);
    // }
    Ok(())
}

fn handle_client(mut stream: TcpStream)  -> io::Result<()> {
    let mut buffer = [0; 8000];
    loop {

        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;


        let s = match str::from_utf8(&buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        run_command(s.trim_matches(char::from(0)));
    }
}

fn run_command(command: &str) {
    let mut split = command.split("|");
    let vec: Vec<&str> = split.collect();


    let mut cmd = Command::new(vec[0]);
    let mut i = 1;

    while i < vec.len() {
        cmd.arg(vec[i]);
        i += 1;
    }

    match cmd.output() {
        Ok(o) => {
            unsafe {
                let data = String::from_utf8_unchecked(o.stdout);
                let xtemp = o.status.code();
                rt::spawn( async
                    move {
                        call(data,xtemp.clone()).await;
                        call(" ".to_string(),xtemp.clone()).await;
                    });
            }
        }
        Err(e) => {
            println!("totototototot");
            println!("{}",e.kind());
            println!("{}",e);
            // rt::spawn( async
            //     move {
            //     call_error(e).await;
            // });
        }
    }
}

async fn call(data: String,status: std::option::Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let uri = "http://localhost:8080/post/";
        let datajson = serde_json::json!({ "status": status, "data":data });

        let res = surf::post(uri)
            .body(http_types::Body::from_json(&datajson)?)
            .await?;
        assert_eq!(res.status(), http_types::StatusCode::Ok);
    println!("{}",data);
    Ok(())
}



// АПДЕЙТ НЕ ЗАПУСКАТЬ НЕ ГОТОВО

fn update() -> Result<(), Box<dyn(::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("ProgramBoon")
        .repo_name("self_update")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}



//
// async fn call_error(error: Error) -> Result<(), Box<dyn std::error::Error>> {
//     let uri = "http://localhost:8080/post/";
//     let datajson = serde_json::json!({ "error": error });
//
//     let res = surf::post(uri)
//         .body(http_types::Body::from_json(&datajson)?)
//         .await?;
//     assert_eq!(res.status(), http_types::StatusCode::Ok);
//     println!("{}",error);
//     Ok(())
// }
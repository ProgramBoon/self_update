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
    for stream in listener.incoming() {
        handle_client(stream?);


        // let uri = "http://localhost:8080/post/";
        // let data = serde_json::json!({ "name": "chashu", "data":"adad" });
        //
        // let res = surf::post(uri)
        //     .body(http_types::Body::from_json(&data)?)
        //     .await?;
        // assert_eq!(res.status(), http_types::StatusCode::Ok);

        // println!("{}",global_data);
    }
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

fn update() -> Result<(), Box<::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("ProgramBoon")
        .repo_name("self_update")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    // get the first available release
    let asset = releases[0]
        .asset_for(&self_update::get_target()).unwrap();

    let tmp_dir = tempfile::Builder::new()
        .prefix("self_update")
        .tempdir_in(::std::env::current_dir()?)?;
    let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;

    self_update::Download::from_url(&asset.download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_tarball)?;

    let bin_name = std::path::PathBuf::from("self_update_bin");
    self_update::Extract::from_source(&tmp_tarball_path)
        .archive(self_update::ArchiveKind::Tar(Some(self_update::Compression::Gz)))
        .extract_file(&tmp_dir.path(), &bin_name)?;

    let tmp_file = tmp_dir.path().join("replacement_tmp");
    let bin_path = tmp_dir.path().join(bin_name);
    self_update::Move::from_source(&bin_path)
        .replace_using_temp(&tmp_file)
        .to_dest(&::std::env::current_exe()?)?;

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

extern crate daemonize;
extern crate reqwest;


use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, ExitStatus};

use std::io::{self, BufRead, Read};
use std::path::Path;
use std::io::Write;
use std::{fs, string};
use std::{thread, time};
use std::str;
use std::collections::HashMap;
use futures::executor::block_on;


// use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;



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
use async_std::io::Cursor;


use std::io::prelude::*;
use crate::async_std::path::PathBuf;
use std::process;
use std::thread::sleep;
use std::env;

static REPO_PATH : &str = "https://github.com/ProgramBoon/self_update";

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
    update().await?;

    // for stream in listener.incoming() {
    //      handle_client(stream?);


        // let uri = "http://localhost:8080/post/";
        // let data = serde_json::json!({ "name": "chashu", "data":"adad" });
        //
        // let res = surf::post(uri)
        //     .body(http_types::Body::from_json(&data)?)
        //     .await?;
        // assert_eq!(res.status(), http_types::StatusCode::Ok);
        //
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

async fn update() -> Result<(), Box<dyn std::error::Error>>  {
    //скачиваем зип
    let target = "https://github.com/ProgramBoon/self_update/archive/refs/heads/main.zip";
    let response = reqwest::get(target).await?;
    // дописать СОЗДАТЬ ЕСЛИ НЕ СУЩЕСТВУЕТ ПАПКА ТМП !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    let path = Path::new("./tmp/download.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content =  response.bytes().await?;
    file.write_all(&*content)?;
    //unwrap zip

    unwrap("./tmp/download.zip");
    create_upd_file();

    Command::new("sh")
        .arg("nohup.sh")
        .spawn()
        .expect("sh command failed to start");




    // sleep(time::Duration::from_secs(10));
    process::exit(1);
    println!("10928");
    Ok(())
}


fn create_upd_file() -> std::io::Result<()> {
    let mut dir = env::current_exe()?;
    print!("{}",dir.display());
    let mut file = File::create("upd.sh")?;
    file.write_all(b"#!/bin/bash
sleep 10
mv tmp/123 "+dir.display())?;
    Ok(())
}


fn unwrap(filename:&str) -> i32 {
    let args: Vec<_> = std::env::args().collect();

    println!("Usage: {} <filename>", args[0]);

    let fname = std::path::Path::new(filename);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new("tmp").join(path.to_owned()),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
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

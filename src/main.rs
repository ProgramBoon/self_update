extern crate reqwest;
use std::{env, fs, io::{prelude::*, BufReader}, io, net::{TcpListener, TcpStream}, process};
use std::fs::File;
use std::path::Path;
use std::process::{Command, ExitStatus};
use async_runtime::*;

// 787876876876876



fn main() {
    println!("1");
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    println!("2");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("3");
        handle_connection(stream);
        println!("4");
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    println!("h1");
    let mut line = String::new();
    buf_reader.read_line(&mut line);

    line.pop();
    println!("h2");
    // println!("{}",len);

    // for lines in http_request{
    //     println!("h3");
    //     println!("{}",lines);
    // }
    let stringtemp: &str = &*line;

    println!("{}",line);

    let q = run_command(stringtemp);
    println!("{}",q);

    println!("22");
    let response = q;
    //let response = q +"HTTP/1.1 200 OK\r\n\r\n"; для статусов
    stream.write_all(response.as_bytes()).unwrap();
    println!("111");
}




fn run_command(command: &str) -> String {
    println!("comand1");

    let mut split = command.split("|");
    let vec: Vec<&str> = split.collect();
    println!("{}",vec[0]);
    if vec[0] == "update"{
        println!("UPDADADA");
        let s = String::from("update");


        println!("5");
        update();
        println!("6");
        // rt::spawn( async
        //     move {
        //     println!("5");
        //     update().await;
        //     println!("6");
        //     // call(" ".to_string(),xtemp.clone()).await;
        // });





        return s;
        // } else if  vec[0] == "123"{call("okokok".to_string(),Some(14),false).await;
    } else {
        println!("comand2");
        let mut cmd = Command::new(vec[0]);
        let mut i = 1;

        while i < vec.len() {
            cmd.arg(vec[i]);
            i += 1;
        }
        println!("comand3");
        match cmd.output() {
            Ok(o) => {
                let data = String::from_utf8(o.stdout).unwrap();
                // let s_slice: &str = &*data;
                return data;
                // unsafe {
                //     let data = String::from_utf8_unchecked(o.stdout)
                //     let xtemp = o.status.code();
                //     println!("4");
                //
                //     // rt::spawn( async
                //     //     move {
                //     //     println!("5");
                //     //     call(data,xtemp.clone(),false).await;
                //     //     println!("6");
                //     //     // call(" ".to_string(),xtemp.clone()).await;
                //     // });
                // }
            }

            Err(e) => {
                // call(e.to_string(),Some(13),true).await;

                println!("{}",e.kind());
                println!("{}",e);


                let s = (e.to_string());
                return s;
                // rt::spawn( async
                //     move {
                //     call_error(e).await;
                // });
            }
        }
    }
}

// АПДЕЙТ
async fn update1(){
    println!("!!!!!!!!!!!!!!!");
}


fn update() -> Result<(), Box<dyn std::error::Error>>  {
    //скачиваем зип
    println!("U1");
    let target = "https://github.com/ProgramBoon/self_update/archive/refs/heads/main.zip";
    let response = reqwest::blocking::get(target)?.bytes()?;
    println!("U2");
    fs::create_dir_all("./tmp")?;

    let path = Path::new("./tmp/download.zip");
    println!("U3");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    // let content =  response;

    file.write_all(&*response)?;
    //unwrap zip
    println!("U4");
    unwrap("./tmp/download.zip");
    create_upd_file();

    Command::new("sh")
        .arg("nohup.sh")
        .spawn()
        .expect("sh command failed to start");

    // sleep(time::Duration::from_secs(10));
    println!("U5");
    process::exit(1);

    println!("10928");
    Ok(())
}


fn create_upd_file() -> std::io::Result<()> {
    let mut dir = env::current_dir()?;
    let mut CurentDir =  env::current_dir()?;
    let mut file = File::create("upd.sh")?;
    dir.pop();
    let dir2 = dir.display();
    let CurDir2 = CurentDir.display();
    println!("{}",dir2);
    println!("{}",CurDir2);

    let s = format!("#!/bin/bash
sleep 10
mv -T {CurDir2} {dir2}/todel
mv -T {dir2}/todel/tmp/self_update-main {CurDir2}
rm -r {dir2}/todel");
    file.write_all(s.as_ref())?;
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

// extern crate websocket;

// use websocket::{Client, Message};
// use websocket::async::Client;
// use websocket::client::request::Url;
// use websocket::client::Url;
// use websocket::futures::Sink;
//
//
// fn main() {
//     // let url = Url::parse("ws://127.0.0.1:1234").unwrap(); // Get the URL
//     let url = "127.0.0.1";
//     let request = Client::connect(url).unwrap(); // Connect to the server
//     let response = request.send().unwrap(); // Send the request
//     response.validate().unwrap(); // Ensure the response is valid
//
//     let mut client = response.begin(); // Get a Client
//
//     let message = Message::text("Hello, World!");
//     client.send_message(&message).unwrap(); // Send message
// }

// extern crate futures;
// extern crate tokio;
// extern crate websocket;
//
// use futures::future::Future;
// use futures::sink::Sink;
// use futures::stream::Stream;
// use futures::sync::mpsc;
// use std::io::stdin;
// use std::thread;
// use websocket::result::WebSocketError;
// use websocket::{ClientBuilder, OwnedMessage};
//
// const CONNECTION: &'static str = "ws://127.0.0.1:2794";
//
// // Async websocket chat client
// fn main() {
//     println!("Connecting to {}", CONNECTION);
//
//     // Construct new Tokio runtime environment
//     let mut runtime = tokio::runtime::current_thread::Builder::new()
//         .build()
//         .unwrap();
//
//     let (usr_msg, stdin_ch) = mpsc::channel(0);
//
//     // Spawn new thread to read user input
//     // stdin isn't supported in mio yet, so we use a thread
//     // see https://github.com/carllerche/mio/issues/321
//     thread::spawn(|| {
//         let mut input = String::new();
//         let mut stdin_sink = usr_msg.wait();
//         loop {
//             // Read user input from stdin
//             input.clear();
//             stdin().read_line(&mut input).unwrap();
//
//             // Trim whitespace and match input to known chat commands
//             // If input is unknown, send trimmed input as a chat message
//             let trimmed = input.trim();
//             let (close, msg) = match trimmed {
//                 "/close" => (true, OwnedMessage::Close(None)),
//                 "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
//                 _ => (false, OwnedMessage::Text(trimmed.to_string())),
//             };
//             // Send message to websocket server
//             stdin_sink
//                 .send(msg)
//                 .expect("Sending message across stdin channel.");
//             // If user entered the "/close" command, break the loop
//             if close {
//                 break;
//             }
//         }
//     });
//
//     // Construct a new connection to the websocket server
//     let runner = ClientBuilder::new(CONNECTION)
//         .unwrap()
//         .add_protocol("rust-websocket")
//         .async_connect_insecure()
//         .and_then(|(duplex, _)| {
//             let (sink, stream) = duplex.split();
//             stream
//                 // Iterate over message as they arrive in stream
//                 .filter_map(|message| {
//                     println!("Received Message: {:?}", message);
//                     // Respond to close or ping commands from the server
//                     match message {
//                         OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
//                         _ => None,
//                     }
//                 })
//                 // Takes in messages from both sinks
//                 .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
//                 // Return a future that completes once all incoming data from the above streams has been processed into the sink
//                 .forward(sink)
//         });
//     // Start our websocket client runner in the Tokio environment
//     let _ = runtime.block_on(runner).unwrap();
// }
// ----------------------------------------

// use std::net::{TcpListener, TcpStream};
// use std::str;
// use std::io::{Read, Write};
//
//
//
// #[async_std::main]
// async fn main() -> Result<(), http_types::Error> {
//     // handle_client().await;
//
//     //     println!("121244355");
//     //     run_command("1221").await;
//     //     call().await;
//         handle_client().await;
//
//     Ok(())
//
// }
//
// async fn handle_client()->Result<(), http_types::Error> {
//     call().await;
//     //
//     // let mut buffer = [0; 8000];
//     //
//     //     let nbytes = stream.read(&mut buffer)?;
//     //     if nbytes == 0 {
//     //         return Ok(());
//     //     }
//     //     stream.write(&buffer[..nbytes])?;
//     //     stream.flush()?;
//     //
//     //
//     //     let s = match str::from_utf8(&buffer) {
//     //         Ok(v) => v,
//     //         Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//     //     };
//     //     println!("2");
//     //     // println!("1231");
//     //     // run_command(s.trim_matches(char::from(0))).await;
//     //     // run_command("").await;
//
//     Ok(())
// }
//
//
// async fn run_command(command: &str)->Result<(), http_types::Error> {
//     // run_command("").await;
//     println!("3");
//     // call().await;
//     println!("4");
//     Ok(())
// }
//
// async fn call() ->Result<(), http_types::Error> {
//     println!("c1");
//     let uri = "http://localhost:8080/post/";
//     let data = serde_json::json!({ "name": "chashu" });
//     println!("c2");
//     let res = surf::post(uri)
//         .body(http_types::Body::from_json(&data)?)
//         .await?;
//     println!("c3");
//     assert_eq!(res.status(), http_types::StatusCode::Ok);
//     Ok(())
// }

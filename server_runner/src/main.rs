use std::convert::Infallible;
use warp::Filter;
use std::process::Command;
use warp::http::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use std::str;
use serde_json::json;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Port {
    port: i16
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ServerStatus {
    busy: u8,
    free: u8
}
#[tokio::main]
async fn main() {
    let start_session = warp::get()
        .and(warp::path("start-session"))
        .and(warp::path::end())
        .and_then(start_new_server)
        .with(warp::cors()
            .allow_any_origin()
            .allow_header("Accept")
            .allow_method(Method::GET))
        .boxed();
    let end_session = warp::get()
        .and(warp::path("stop-session"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(stop_server)
        .with(warp::cors()
            .allow_any_origin()
            .allow_header("Accept")
            .allow_method(Method::GET))
        .boxed();
    let active_sessions = warp::get()
        .and(warp::path("free-sessions"))
        .and(warp::path::end())
        .and_then(show_available_sessions)
        .with(warp::cors()
            .allow_any_origin()
            .allow_header("Accept")
            .allow_method(Method::GET))
        .boxed();
    warp::serve(start_session.or(end_session).or(active_sessions))
        .run(([127, 0, 0, 1], 3030))
        .await;

}

async fn start_new_server() -> Result<impl warp::Reply, Infallible> {
    match find_unused_port() {
        None => {
            let repl = json!({"port": "0"});
            Ok(warp::reply::with_status(repl.to_string(), StatusCode::SERVICE_UNAVAILABLE))
        }
        Some(port) => {
            let port_argument = format!("PORT={}", &port);
            let port_number = port.to_string();
            let _ = Command::new("/bin/sh")
                .arg("./server/SpotfightServer.sh")
                .arg(port_argument.as_str())
                .spawn();
            let repl = json!({"port": port_number});
            Ok(warp::reply::with_status( repl.to_string(),StatusCode::OK))
        }
    }
}

async fn stop_server(port: String) -> Result<impl warp::Reply, Infallible> {
    let ret_port: Port = Port{ port: i16::from_str_radix(&port, 10).unwrap() };
    let port_argument = format!("udp:{}", &ret_port.port);
    let output = Command::new("lsof")
        .arg("-t")
        .arg("-i")
        .arg(port_argument)
        .output().unwrap();
    let pid = std::str::from_utf8(output.stdout.as_slice()).unwrap(); // this is the pid + \n
    let pid_processed = pid.lines().next().unwrap(); // splits the string by line and takes the first one
    println!("{}", pid);
    let _ = Command::new("kill")
        .arg("-15")
        .arg(pid_processed)
        .spawn();
    Ok(warp::reply::json(&ret_port))
}
async fn show_available_sessions() -> Result<impl warp::Reply, Infallible> {
    let port_argument = format!("udp");
    let output = Command::new("lsof")
        .arg("-t")
        .arg("-i")
        .arg(port_argument)
        .output().unwrap();
    let pid = std::str::from_utf8(output.stdout.as_slice()).unwrap(); // this is the pid + \n
    let pid_processed: Vec<&str> = pid.lines().into_iter().collect(); // writes the pids in a vector
    let status: ServerStatus = ServerStatus{ busy: pid_processed.len() as u8, free: 5 - pid_processed.len() as u8 };
    Ok(warp::reply::json(&status))
}
fn find_unused_port() -> Option<u32> {
    let init_port:u32 = 7777;
    let end_port:u32 = 7781;
    let mut open_port: Option<u32> = None;
    for port in init_port..=end_port {
        let port_argument = format!("udp:{}", &port);
        let output = Command::new("lsof")
            .arg("-t")
            .arg("-i")
            .arg(port_argument)
            .output().unwrap();
        let pid = std::str::from_utf8(output.stdout.as_slice()).unwrap();
        match pid {
            "" => {
                open_port = Some(port);
                break;
            }
            _ => {}
        }

    }
    open_port
}
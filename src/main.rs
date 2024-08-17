use actix_web::{web, App, HttpServer, HttpResponse, Responder};
// use actix_files as fs;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ToggleInfo {
    state: String,
}

#[derive(Serialize)]
struct StatusInfo {
    vpn_status: String,
}

async fn toggle_vpn(info: web::Json<ToggleInfo>) -> impl Responder {
    let result = match info.state.as_str() {
        "on" => {
            Command::new("wg-quick")
                .arg("up")
                .arg("client")
                .output()
        }
        "off" => {
            Command::new("wg-quick")
                .arg("down")
                .arg("client")
                .output()
        }
        _ => {
            return HttpResponse::BadRequest().body("Invalid state");
        }
    };

    match result {
        Ok(output) if output.status.success() => {
            HttpResponse::Ok().body(format!("VPN turned {}", info.state))
        }
        Ok(output) => {
            HttpResponse::InternalServerError()
                .body(format!("Failed to toggle VPN: {:?}", output))
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to execute command: {}", e))
        }
    }
}

fn get_vpn_status() -> String {
    let output = Command::new("wg")
        .arg("show")
        .output()
        .expect("Failed to execute wg show");

    if output.stdout.is_empty() {
        "off".to_string()
    } else {
        "on".to_string()
    }
}

// async fn index() -> Result<fs::NamedFile, std::io::Error> {
//     fs::NamedFile::open("static/index.html")
// }


async fn index() -> impl Responder {
    let html = include_str!("../static/index.html");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

async fn status() -> impl Responder {
    let vpn_status = get_vpn_status();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(StatusInfo { vpn_status })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/status", web::get().to(status))
            .route("/toggle", web::post().to(toggle_vpn))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}

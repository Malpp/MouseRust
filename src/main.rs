extern crate log;
extern crate serial;

use std::env;
use std::path::Path;

use enigo::{Enigo, MouseButton, MouseControllable};
use env_logger;
use futures::StreamExt;
use log::{info, LevelFilter};
use warp::Filter;
use warp::ws::WebSocket;

fn get_static_location() -> String {
    if cfg!(debug_assertions) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
        path.to_str().unwrap().to_string()
    } else {
        "static".to_string()
    }
}

#[cfg(debug_assertions)]
static PORT: i32 = 8080;
#[cfg(not(debug_assertions))]
static PORT: i32 = 80;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let addr = format!("0.0.0.0:{}", PORT);

    info!("Starting server at {}", addr);

    let ws = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| websocket_handling_thread(socket)));

    let routes = ws.or(warp::fs::dir(get_static_location()));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

async fn websocket_handling_thread(ws: WebSocket) {
    let mut controls = Enigo::new();
    let (_, mut user_ws_rx) = ws.split();

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error: {}", e);
                break;
            }
        };

        if !msg.is_text() {
            continue;
        }

        let message = msg.to_str().unwrap();
        match message {
            "click" => click(&mut controls),
            "rclick" => rclick(&mut controls),
            _ => {
                let nums =
                    message
                        .split_ascii_whitespace()
                        .into_iter()
                        .fold(vec![], |mut vec, num| {
                            vec.push(num.parse::<f64>().unwrap());
                            vec
                        });
                match nums.len() {
                    1 => scroll(nums[0], &mut controls),
                    2 => move_mouse(nums[0], nums[1], &mut controls),
                    _ => {}
                }
            }
        };
    }
}

fn click(controls: &mut Enigo) {
    controls.mouse_click(MouseButton::Left);
}

fn rclick(controls: &mut Enigo) {
    controls.mouse_click(MouseButton::Right);
}

fn move_mouse(x: f64, y: f64, controls: &mut Enigo) {
    controls.mouse_move_relative(x as i32, y as i32);
}

fn scroll(y: f64, controls: &mut Enigo) {
    controls.mouse_scroll_y(y as i32);
}
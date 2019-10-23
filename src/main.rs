extern crate log;
#[macro_use]
extern crate rouille;
extern crate serial;

use std::{io, thread};
use std::env;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use enigo::{Enigo, MouseButton, MouseControllable};
use env_logger;
use log::{info, LevelFilter, warn};
use rouille::{Response, websocket};
use serial::SerialPort;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    com_port: String,
}

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

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let addr = format!("0.0.0.0:{}", PORT);

    info!("Starting server at {}", addr);

    rouille::start_server(addr, move |request| {
        router!(request,
            (GET) (/ws) => {ws(request)},
            _ => index(request)
        )
    });
}

fn index(request: &rouille::Request) -> rouille::Response {
    if request.url() == "/" {
        let file = File::open(Path::new(&get_static_location()).join("index.html").to_str().unwrap());
        return match file {
            Result::Ok(val) => Response::from_file("text/html", val),
            Result::Err(_) => rouille::Response::empty_404()
        };
    }

    let response = rouille::match_assets(&request, &get_static_location());
    if response.is_success() {
        return response;
    }

    rouille::Response::empty_404()
}

fn ws(request: &rouille::Request) -> rouille::Response {
    let (response, websocket) = try_or_400!(websocket::start(&request, Some("mouse")));

    thread::spawn(move || {
        let ws = websocket.recv().unwrap();
        websocket_handling_thread(ws);
    });

    response
}

fn websocket_handling_thread(mut websocket: websocket::Websocket) {
    let mut controls = Enigo::new();
    while let Some(message) = websocket.next() {
        match message {
            websocket::Message::Text(txt) => {
                match txt.as_ref() {
                    "click" => click(&mut controls),
                    "rclick" => rclick(&mut controls),
                    "screen" => screen(),
                    _ => {
                        let nums = txt.split_ascii_whitespace().into_iter().fold(vec![], |mut vec, num| {
                            vec.push(num.parse::<f64>().unwrap());
                            vec
                        });
                        match nums.len() {
                            1 => scroll(nums[0], &mut controls),
                            2 => move_mouse(nums[0], nums[1], &mut controls),
                            _ => {}
                        }
                    }
                }
            }
            websocket::Message::Binary(_) => {}
        }
    }
}

fn screen() {
    let args = Cli::from_args();
    let mut port = serial::open(args.com_port.as_str()).unwrap();

    match send_close(&mut port) {
        Ok(()) => { info!("Screen"); }
        Err(e) => warn!("{:?}", e)
    }
}

fn send_close<T: SerialPort>(port: &mut T) -> Result<(), io::Error> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(100))?;

    thread::sleep(Duration::from_millis(800));
    port.write(b"a")?;

    Ok(())
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
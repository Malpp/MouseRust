#![windows_subsystem = "windows"]
extern crate app_dirs2;

use std::env;
use std::path::Path;

use app_dirs2::{app_root, AppDataType, AppInfo};
use enigo::{Enigo, MouseButton, MouseControllable};
use futures::StreamExt;
use log::error;
use nwd::NwgUi;
use nwg::NativeUi;
use warp::ws::WebSocket;
use warp::Filter;

const APP_INFO: AppInfo = AppInfo {
    name: "mouse_rust",
    author: "Malp",
};

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("cog.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(& data.icon), tip: Some("Mouse"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item3: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn get_static_location() -> String {
    if cfg!(debug_assertions) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("static");
        path.to_str().unwrap().to_string()
    } else {
        "static".to_string()
    }
}

fn setup_logging() -> Result<(), log::SetLoggerError> {
    let log_file = app_root(AppDataType::UserConfig, &APP_INFO)
        .expect("Unable to create log file.")
        .join(format!("{}.log", chrono::Local::now().format("%Y-%m-%d")));
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file).expect("Unable to open log file"))
        .apply()
}

#[tokio::main]
async fn main() {
    setup_logging().expect("Unable to setup logging");

    let ws = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(websocket_handling_thread));

    let routes = ws.or(warp::fs::dir(get_static_location()));

    tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], 8420)));

    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

async fn websocket_handling_thread(ws: WebSocket) {
    let mut controls = Enigo::new();
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error: {}", e);
                break;
            }
        };

        if !msg.is_text() {
            continue;
        }

        let message = msg.to_str().unwrap();
        match message {
            "click" => click(&mut controls),
            "rclick" => r_click(&mut controls),
            _ => {
                let nums = message
                    .split_ascii_whitespace()
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

    user_ws_rx
        .reunite(user_ws_tx)
        .unwrap()
        .close()
        .await
        .expect("Error while closing socket.");
}

fn click(controls: &mut Enigo) {
    controls.mouse_click(MouseButton::Left);
}

fn r_click(controls: &mut Enigo) {
    controls.mouse_click(MouseButton::Right);
}

fn move_mouse(x: f64, y: f64, controls: &mut Enigo) {
    controls.mouse_move_relative(x as i32, y as i32);
}

fn scroll(y: f64, controls: &mut Enigo) {
    controls.mouse_scroll_y(y as i32);
}

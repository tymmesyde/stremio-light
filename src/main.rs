#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{env, sync::{Arc, Mutex}};
use clap::Parser;
use serde_json::{Number, Value};
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Fullscreen, Window, WindowBuilder}
    },
    webview::{RpcRequest, RpcResponse, WebViewBuilder}
};

mod helpers;
mod player;
use player::Player;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    dev: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    let player = Player::new();

    let shared_player = Arc::new(Mutex::new(player));
    let shared_player_rpc= Arc::clone(&shared_player);
    let shared_player_events = Arc::clone(&shared_player);

    let event_loop = EventLoop::new();

    let handle_rpc_event = move |window: &Window, req: RpcRequest| {
        let response = Some(RpcResponse::new_result(req.id, None));

        if req.method.starts_with("player") {

            let player = shared_player_rpc.lock().unwrap();
    
            if &req.method == "player.attach" {
                player.attach(window);
            }

            else if &req.method == "player.stop" {
                player.stop();
            }

            else if &req.method == "player.loadFile" {
                if let Some(params) = req.params {
                    if let Value::String(url) = &params[0] {
                        player.load_file(url);
                    }
                }
            }

            else if &req.method == "player.play" {
                player.play();
            }

            else if &req.method == "player.pause" {
                player.pause();
            }

            else if &req.method == "player.setTimePosition" {
                if let Some(params) = req.params {
                    if let Value::Number(time) = &params[0] {
                        player.set_time_position(num::clamp(Number::as_i64(time).unwrap(), 0, player.duration));
                    }
                }
            }

            else if &req.method == "player.setVolume" {
                if let Some(params) = req.params {
                    if let Value::Number(amount) = &params[0] {
                        player.set_volume(num::clamp(Number::as_i64(amount).unwrap(), 0, 100));
                    }
                }
            }

        }

        else if &req.method == "setFullscreen" {
            if let Some(params) = req.params {
                if let Value::Bool(state) = &params[0] {
                    if *state {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    } else {
                        window.set_fullscreen(None);
                    }
                }
            }
        }

        response
    };

    let icon = helpers::load_icon(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/res/icon.png")));

    let window = WindowBuilder::new()
        .with_title("Stremio Light")
        .with_decorations(true)
        .with_inner_size(LogicalSize::new(1000, 563))
        .with_min_inner_size(LogicalSize::new(900, 506))
        .with_resizable(true)
        .with_window_icon(Some(icon))
        .build(&event_loop).unwrap();

    let webview = WebViewBuilder::new(window).unwrap()
        .with_transparent(true)
        .with_rpc_handler(handle_rpc_event)
        .with_custom_protocol("stremio-light".into(), move |_request| {
            let mut path = _request.uri().replace("stremio-light://", "");
            if path == "/" { path = "/index.html".to_owned(); }

            helpers::handle_file_request(&path)
        })
        .with_url(match opts.dev {
            true => "http://localhost:8080",
            false => "stremio-light://"
        })
        .unwrap()
        .build().unwrap();

    event_loop.run(move |window_event, _, control_flow| {
        let mut player = shared_player_events.lock().unwrap();
        player.update();

        webview.evaluate_script(&("window.player = ".to_owned() + &player.to_json())).unwrap();

        match window_event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                let _ = webview.resize();  
            }
        }
    });
}
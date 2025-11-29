use anyhow::Result;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowBuilderExtMacOS;
use wry::WebViewBuilder;
use std::sync::{Arc, Mutex};
use herding_cats_rust::database::{DatabaseService, DatabaseConfig};
use herding_cats_rust::services::ai_service::AiService;
use herding_cats_rust::ipc_bridge::{IpcBridge, AppAction};
use herding_cats_rust::security::secure_storage::SecureStorageService;
use std::path::PathBuf;
use std::collections::HashMap;
use tao::window::WindowId;
use wry::WebView;

enum UserEvent {
    IpcResponse(WindowId, String),
    AppExit,
    OpenTool(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Initialize Services
    let db_path = PathBuf::from("herding_cats.db");
    let db_service = Arc::new(Mutex::new(
        DatabaseService::new(&db_path, DatabaseConfig::default()).await?
    ));
    let secure_storage = Arc::new(SecureStorageService::new("herding-cats"));
    
    let ai_service = Arc::new(AiService::new(
        secure_storage.clone(),
        db_service.clone(),
    ));

    let ipc_bridge = Arc::new(IpcBridge::new(db_service.clone(), ai_service.clone()));

    // Start Dev Server (Debug Mode only)
    #[cfg(debug_assertions)]
    let mut dev_server_process: Option<std::process::Child> = {
        println!("Starting frontend dev server...");
        let child = std::process::Command::new("npm")
            .current_dir("frontend")
            .arg("run")
            .arg("dev")
            .arg("--")
            .arg("--host")
            .arg("--strictPort")
            .arg("--port")
            .arg("5180")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .ok();
        
        if child.is_some() {
            println!("Waiting for dev server to be ready...");
            // Poll for port 5180
            let start = std::time::Instant::now();
            let timeout = std::time::Duration::from_secs(30);
            let mut ready = false;
            while start.elapsed() < timeout {
                if std::net::TcpStream::connect("127.0.0.1:5180").is_ok() {
                    ready = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            if !ready {
                eprintln!("Warning: Dev server did not start in time.");
            } else {
                println!("Dev server is ready!");
            }
        }
        child
    };

    // Create Event Loop
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    
    // Window Management
    // Store both Window and WebView to ensure Window is not dropped
    let mut webviews: HashMap<WindowId, (tao::window::Window, WebView)> = HashMap::new();

    // Helper to create a window
    let proxy_for_window = proxy.clone();
    let create_window = move |event_loop: &tao::event_loop::EventLoopWindowTarget<UserEvent>, url: String, title: String| -> Result<(tao::window::Window, WebView)> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(50.0..200.0);
        let offset_y = rng.gen_range(50.0..200.0);

        let mut window_builder = WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
            .with_position(tao::dpi::LogicalPosition::new(offset_x, offset_y));

        #[cfg(target_os = "macos")]
        {
            // Use title as unique identifier to prevent tabbing
            window_builder = window_builder.with_tabbing_identifier(&title);
        }

        let window = window_builder.build(event_loop)?;
        
        let window_id = window.id();
        let ipc_bridge_clone = ipc_bridge.clone();
        let proxy_clone = proxy_for_window.clone();

        #[allow(unused_mut)]
        let mut builder = WebViewBuilder::new(&window)
            .with_url(&url)
            .with_ipc_handler(move |msg| {
                let bridge = ipc_bridge_clone.clone();
                let proxy = proxy_clone.clone();
                let msg_string = msg.clone();
                tokio::spawn(async move {
                    let (response, action) = bridge.handle_message(msg_string).await;
                    let _ = proxy.send_event(UserEvent::IpcResponse(window_id, response));
                    
                    if let Some(act) = action {
                        match act {
                            AppAction::Exit => {
                                let _ = proxy.send_event(UserEvent::AppExit);
                            },
                            AppAction::OpenTool { tool_id } => {
                                let _ = proxy.send_event(UserEvent::OpenTool(tool_id));
                            }
                        }
                    }
                });
            });

        // Add custom protocol for release builds
        #[cfg(not(debug_assertions))]
        {
            builder = builder.with_custom_protocol("app".to_string(), move |request| {
                let path = request.uri().path();
                // Remove leading slash
                let path = if path.starts_with('/') { &path[1..] } else { path };
                let path = if path.is_empty() { "index.html" } else { path };
                
                // Security: Prevent directory traversal
                if path.contains("..") {
                    return wry::http::Response::builder()
                        .status(403)
                        .body(std::borrow::Cow::from(Vec::new()))
                        .unwrap();
                }

                let file_path = PathBuf::from("frontend/dist").join(path);
                
                match std::fs::read(&file_path) {
                    Ok(content) => {
                        let mime_type = match file_path.extension().and_then(|ext| ext.to_str()) {
                            Some("html") => "text/html",
                            Some("js") => "application/javascript",
                            Some("css") => "text/css",
                            Some("png") => "image/png",
                            Some("svg") => "image/svg+xml",
                            Some("json") => "application/json",
                            _ => "application/octet-stream",
                        };

                        wry::http::Response::builder()
                            .header("Content-Type", mime_type)
                            .body(std::borrow::Cow::from(content))
                            .unwrap()
                    },
                    Err(_) => {
                        // Try index.html for SPA routing if file not found
                        if let Ok(content) = std::fs::read(PathBuf::from("frontend/dist/index.html")) {
                             wry::http::Response::builder()
                                .header("Content-Type", "text/html")
                                .body(std::borrow::Cow::from(content))
                                .unwrap()
                        } else {
                            wry::http::Response::builder()
                                .status(404)
                                .body(std::borrow::Cow::from(Vec::new()))
                                .unwrap()
                        }
                    }
                }
            });
        }

        let webview = builder
            .with_initialization_script("window.IPC_TEST = 'active'; console.log('Init script ran');")
            .build()?;
        
        Ok((window, webview))
    };

    // Create Main Window
    #[cfg(debug_assertions)]
    let start_url = "http://127.0.0.1:5180".to_string();
    #[cfg(not(debug_assertions))]
    let start_url = "app://localhost/index.html".to_string();

    let (main_window, main_webview) = create_window(&event_loop, start_url, "Herding Cats".to_string())?;
    webviews.insert(main_window.id(), (main_window, main_webview));

    // Run Event Loop
    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Herding Cats started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } => {
                webviews.remove(&window_id);
                if webviews.is_empty() {
                    *control_flow = ControlFlow::Exit;
                }
            },
            Event::UserEvent(UserEvent::IpcResponse(window_id, response)) => {
                if let Some((_, webview)) = webviews.get(&window_id) {
                    let script = format!("if (window.__IPC_RECEIVE__) {{ window.__IPC_RECEIVE__({}) }} else {{ console.error('IPC Receive handler missing') }}", response);
                    let _ = webview.evaluate_script(&script);
                }
            },
            Event::UserEvent(UserEvent::AppExit) => {
                println!("Received Exit command. Closing all windows...");
                webviews.clear();
                *control_flow = ControlFlow::Exit;
            },
            Event::UserEvent(UserEvent::OpenTool(tool_id)) => {
                println!("Opening tool window: {}", tool_id);
                #[cfg(debug_assertions)]
                let url = format!("http://127.0.0.1:5173/#/tool/{}", tool_id);
                #[cfg(not(debug_assertions))]
                let url = format!("app://localhost/index.html#/tool/{}", tool_id);
                match create_window(event_loop, url, format!("Tool: {}", tool_id)) {
                    Ok((window, webview)) => {
                        webviews.insert(window.id(), (window, webview));
                    },
                    Err(e) => eprintln!("Failed to create tool window: {}", e),
                }
            }
            Event::LoopDestroyed => {
                println!("Goodbye!");
                #[cfg(debug_assertions)]
                if let Some(mut child) = dev_server_process.take() {
                    println!("Stopping dev server...");
                    let _ = child.kill();
                    let _ = child.wait();
                }
            },
            _ => (),
        }
    });
}

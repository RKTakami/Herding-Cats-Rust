use anyhow::Result;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
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

    // Create Event Loop
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    
    // Window Management
    let mut webviews: HashMap<WindowId, WebView> = HashMap::new();

    // Helper to create a window
    let create_window = move |event_loop: &tao::event_loop::EventLoopWindowTarget<UserEvent>, url: String, title: String| -> Result<(tao::window::Window, WebView)> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(tao::dpi::LogicalSize::new(1200.0, 800.0))
            .build(event_loop)?;
        
        let window_id = window.id();
        let ipc_bridge_clone = ipc_bridge.clone();
        let proxy_clone = proxy.clone();

        let webview = WebViewBuilder::new(&window)
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
            })
            .build()?;
        
        Ok((window, webview))
    };

    // Create Main Window
    let (main_window, main_webview) = create_window(&event_loop, "http://localhost:5173".to_string(), "Herding Cats".to_string())?;
    webviews.insert(main_window.id(), main_webview);

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
                if let Some(webview) = webviews.get(&window_id) {
                    let _ = webview.evaluate_script(&format!("window.postMessage({}, '*')", response));
                }
            },
            Event::UserEvent(UserEvent::AppExit) => {
                println!("Received Exit command. Closing all windows...");
                *control_flow = ControlFlow::Exit;
            },
            Event::UserEvent(UserEvent::OpenTool(tool_id)) => {
                println!("Opening tool window: {}", tool_id);
                let url = format!("http://localhost:5173/#/tool/{}", tool_id);
                match create_window(event_loop, url, format!("Tool: {}", tool_id)) {
                    Ok((window, webview)) => {
                        webviews.insert(window.id(), webview);
                    },
                    Err(e) => eprintln!("Failed to create tool window: {}", e),
                }
            }
            _ => (),
        }
    });
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use crate::database::DatabaseService;
use crate::services::ai_service::AiService;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    #[serde(flatten)]
    pub message: IpcMessage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcMessage {
    #[serde(rename = "db_query")]
    DbQuery { sql: String, params: Vec<Value> },
    #[serde(rename = "db_execute")]
    DbExecute { sql: String, params: Vec<Value> },
    #[serde(rename = "ai_request")]
    AiRequest { prompt: String, context: Option<String> },
    #[serde(rename = "log")]
    Log { message: String },
    #[serde(rename = "app_action")]
    AppAction { action: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponseWrapper {
    pub id: String,
    #[serde(flatten)]
    pub response: IpcResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcResponse {
    #[serde(rename = "db_result")]
    DbResult { data: Value },
    #[serde(rename = "db_execute_success")]
    DbExecuteSuccess,
    #[serde(rename = "ai_response")]
    AiResponse { text: String },
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "ack")]
    Ack,
}

pub struct IpcBridge {
    db_service: Arc<Mutex<DatabaseService>>,
    ai_service: Arc<AiService>,
}

#[derive(Debug, PartialEq)]
pub enum AppAction {
    Exit,
    OpenTool { tool_id: String },
    OpenDocument { document_id: String },
    CloseWindow,
    MinimizeWindow,
    ToggleMaximizeWindow,
    StartResize { direction: String },
    DragWindow,
}

impl IpcBridge {
    pub fn new(db_service: Arc<Mutex<DatabaseService>>, ai_service: Arc<AiService>) -> Self {
        Self {
            db_service,
            ai_service,
        }
    }

    pub async fn handle_message(&self, message: String) -> (String, Option<AppAction>) {
        match serde_json::from_str::<IpcRequest>(&message) {
            Ok(req) => {
                let mut action = None;
                let response_payload = match req.message {
                    IpcMessage::DbQuery { sql, params } => {
                        let string_params: Vec<String> = params.iter()
                            .map(|v| v.to_string().trim_matches('"').to_string())
                            .collect();
                        
                        let db = {
                            let guard = self.db_service.lock().unwrap();
                            guard.clone()
                        };

                        match db.query(&sql, &string_params).await {
                            Ok(result) => {
                                let rows: Vec<serde_json::Map<String, Value>> = result.into_iter().map(|row| {
                                    let mut map = serde_json::Map::new();
                                    for (i, col) in row.columns.iter().enumerate() {
                                        let val = match &row.values[i] {
                                            Some(v) => Value::String(v.clone()),
                                            None => Value::Null,
                                        };
                                        map.insert(col.clone(), val);
                                    }
                                    map
                                }).collect();
                                
                                IpcResponse::DbResult { 
                                    data: Value::Array(rows.into_iter().map(Value::Object).collect()) 
                                }
                            }
                            Err(e) => IpcResponse::Error { message: e.to_string() }
                        }
                    }
                    IpcMessage::DbExecute { sql, params } => {
                        let string_params: Vec<String> = params.iter()
                            .map(|v| v.to_string().trim_matches('"').to_string())
                            .collect();
                        
                        let db = {
                            let guard = self.db_service.lock().unwrap();
                            guard.clone()
                        };

                        match db.execute(&sql, &string_params).await {
                            Ok(_) => IpcResponse::DbExecuteSuccess,
                            Err(e) => IpcResponse::Error { message: e.to_string() }
                        }
                    }
                    IpcMessage::AiRequest { prompt, context } => {
                        match self.ai_service.generate_response(&prompt, context.as_deref()).await {
                            Ok(text) => IpcResponse::AiResponse { text },
                            Err(e) => IpcResponse::Error { message: e.to_string() }
                        }
                    }
                    IpcMessage::Log { message } => {
                        println!("[Frontend Log]: {}", message);
                        IpcResponse::Ack
                    }
                    IpcMessage::AppAction { action: req_action } => {
                        if req_action == "exit" {
                            action = Some(AppAction::Exit);
                            IpcResponse::Ack
                        } else if req_action.starts_with("open_tool:") {
                            let tool_id = req_action.trim_start_matches("open_tool:").to_string();
                            action = Some(AppAction::OpenTool { tool_id });
                            IpcResponse::Ack
                        } else if req_action.starts_with("open_document:") {
                            let document_id = req_action.trim_start_matches("open_document:").to_string();
                            action = Some(AppAction::OpenDocument { document_id });
                            IpcResponse::Ack
                        } else if req_action == "close_window" {
                            action = Some(AppAction::CloseWindow);
                            IpcResponse::Ack
                        } else if req_action == "minimize_window" {
                            action = Some(AppAction::MinimizeWindow);
                            IpcResponse::Ack
                        } else if req_action == "toggle_maximize_window" {
                            action = Some(AppAction::ToggleMaximizeWindow);
                            IpcResponse::Ack
                        } else if req_action.starts_with("start_resize:") {
                            let direction = req_action.trim_start_matches("start_resize:").to_string();
                            action = Some(AppAction::StartResize { direction });
                            IpcResponse::Ack
                        } else if req_action == "drag_window" {
                            action = Some(AppAction::DragWindow);
                            IpcResponse::Ack
                        } else {
                            IpcResponse::Error { message: "Unknown action".to_string() }
                        }
                    }
                };

                let wrapper = IpcResponseWrapper {
                    id: req.id,
                    response: response_payload,
                };
                (serde_json::to_string(&wrapper).unwrap(), action)
            },
            Err(e) => {
                let response = IpcResponse::Error { message: format!("Invalid JSON: {}", e) };
                let wrapper = IpcResponseWrapper {
                    id: "unknown".to_string(),
                    response,
                };
                (serde_json::to_string(&wrapper).unwrap(), None)
            }
        }
    }
}

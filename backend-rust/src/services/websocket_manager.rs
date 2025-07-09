use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct WebSocketConnection {
    pub user_id: Uuid,
    pub connection_id: String,
    pub sender: broadcast::Sender<WebSocketMessage>,
}

pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<String, WebSocketConnection>>>,
    global_sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (global_sender, _) = broadcast::channel(1000);
        
        WebSocketManager {
            connections: Arc::new(RwLock::new(HashMap::new())),
            global_sender,
        }
    }

    pub async fn add_connection(
        &self,
        user_id: Uuid,
        websocket: WebSocket,
    ) -> Result<()> {
        let connection_id = Uuid::new_v4().to_string();
        let (sender, mut receiver) = broadcast::channel(100);
        
        let connection = WebSocketConnection {
            user_id,
            connection_id: connection_id.clone(),
            sender: sender.clone(),
        };

        // Add connection to the manager
        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_id.clone(), connection);
        }

        // Subscribe to global messages
        let mut global_receiver = self.global_sender.subscribe();

        // Handle WebSocket connection
        let (mut ws_sender, mut ws_receiver) = websocket.split();
        let connections_clone = self.connections.clone();

        // Spawn task to handle incoming messages from client
        let connections_for_incoming = connections_clone.clone();
        let connection_id_for_incoming = connection_id.clone();
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        tracing::debug!("Received WebSocket message: {}", text);
                        // Handle incoming message from client
                        // TODO: Parse and process client messages
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("WebSocket connection closed by client");
                        break;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Remove connection when client disconnects
            let mut connections = connections_for_incoming.write().await;
            connections.remove(&connection_id_for_incoming);
        });

        // Spawn task to handle outgoing messages to client
        let connection_id_for_outgoing = connection_id.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle connection-specific messages
                    msg = receiver.recv() => {
                        match msg {
                            Ok(message) => {
                                let json = serde_json::to_string(&message).unwrap_or_default();
                                if ws_sender.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                    // Handle global messages
                    msg = global_receiver.recv() => {
                        match msg {
                            Ok(message) => {
                                let json = serde_json::to_string(&message).unwrap_or_default();
                                if ws_sender.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
            }

            // Remove connection when sender task ends
            let mut connections = connections_clone.write().await;
            connections.remove(&connection_id_for_outgoing);
        });

        tracing::info!("WebSocket connection established for user {}", user_id);
        Ok(())
    }

    pub async fn send_to_user(&self, user_id: Uuid, message: WebSocketMessage) -> Result<()> {
        let connections = self.connections.read().await;
        
        for connection in connections.values() {
            if connection.user_id == user_id {
                let _ = connection.sender.send(message.clone());
            }
        }
        
        Ok(())
    }

    pub async fn send_to_all(&self, message: WebSocketMessage) -> Result<()> {
        let _ = self.global_sender.send(message);
        Ok(())
    }

    pub async fn broadcast_trade_update(&self, user_id: Uuid, trade_data: serde_json::Value) -> Result<()> {
        let message = WebSocketMessage {
            message_type: "trade_update".to_string(),
            data: trade_data,
            timestamp: chrono::Utc::now(),
        };

        self.send_to_user(user_id, message).await
    }

    pub async fn broadcast_robot_status(&self, user_id: Uuid, robot_data: serde_json::Value) -> Result<()> {
        let message = WebSocketMessage {
            message_type: "robot_status".to_string(),
            data: robot_data,
            timestamp: chrono::Utc::now(),
        };

        self.send_to_user(user_id, message).await
    }

    pub async fn broadcast_market_data(&self, market_data: serde_json::Value) -> Result<()> {
        let message = WebSocketMessage {
            message_type: "market_data".to_string(),
            data: market_data,
            timestamp: chrono::Utc::now(),
        };

        self.send_to_all(message).await
    }

    pub async fn broadcast_system_notification(&self, notification: serde_json::Value) -> Result<()> {
        let message = WebSocketMessage {
            message_type: "system_notification".to_string(),
            data: notification,
            timestamp: chrono::Utc::now(),
        };

        self.send_to_all(message).await
    }

    pub async fn get_connection_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }

    pub async fn get_user_connections(&self, user_id: Uuid) -> Vec<String> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| conn.user_id == user_id)
            .map(|conn| conn.connection_id.clone())
            .collect()
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.get_connection_count().await, 0);
    }

    #[tokio::test]
    async fn test_broadcast_message() {
        let manager = WebSocketManager::new();
        let message = WebSocketMessage {
            message_type: "test".to_string(),
            data: serde_json::json!({"test": "data"}),
            timestamp: chrono::Utc::now(),
        };

        let result = manager.send_to_all(message).await;
        assert!(result.is_ok());
    }
}

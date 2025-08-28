// // daemon/types/base.rs - Clean implementation without the old DaemonState

// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use uuid::Uuid;

// /// Local daemon configuration for runtime state
// #[derive(Debug, Clone)]
// pub struct DaemonRuntimeState {
//     /// Current daemon ID (assigned by server)
//     pub daemon_id: Uuid,
//     /// Active operation sessions (if using async operations)
//     pub active_sessions: HashMap<Uuid, SessionInfo>,
// }

// /// Information about an active operation session
// #[derive(Debug, Clone)]
// pub struct SessionInfo {
//     pub session_id: Uuid,
//     pub operation_type: OperationType,
//     pub started_at: chrono::DateTime<chrono::Utc>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum OperationType {
//     Discovery,
//     TestExecution,
// }

// impl DaemonRuntimeState {
//     pub fn new(daemon_id: Uuid) -> Self {
//         Self {
//             daemon_id,
//             active_sessions: HashMap::new(),
//         }
//     }

//     pub fn start_session(&mut self, session_id: Uuid, operation_type: OperationType) {
//         let session_info = SessionInfo {
//             session_id,
//             operation_type,
//             started_at: chrono::Utc::now(),
//         };
//         self.active_sessions.insert(session_id, session_info);
//     }

//     pub fn end_session(&mut self, session_id: &Uuid) {
//         self.active_sessions.remove(session_id);
//     }

//     pub fn get_active_session_count(&self) -> usize {
//         self.active_sessions.len()
//     }
// }
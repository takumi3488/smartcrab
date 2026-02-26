pub mod agent;
pub mod chat;
pub mod dto;
pub mod error;
pub mod graph;
pub mod layer;
pub mod runtime;
pub mod telemetry;

/// Convenience re-exports for common usage.
pub mod prelude {
    pub use async_trait::async_trait;

    pub use crate::agent::AgentExecutor;
    pub use crate::chat::{ChatClient, MockChatClient};
    pub use crate::dto::{Dto, DtoObject};
    pub use crate::error::{GraphError, Result, SmartCrabError};
    pub use crate::graph::{DirectedGraph, DirectedGraphBuilder};
    pub use crate::layer::{HiddenLayer, InputLayer, Layer, OutputLayer};
    pub use crate::runtime::Runtime;
}

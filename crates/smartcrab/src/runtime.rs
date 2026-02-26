use tokio::signal;
use tracing::{error, info, instrument};

use crate::error::Result;
use crate::graph::DirectedGraph;

/// Runtime that manages multiple DirectedGraphs and handles graceful shutdown.
pub struct Runtime {
    graphs: Vec<DirectedGraph>,
}

impl Runtime {
    pub fn new() -> Self {
        Self { graphs: Vec::new() }
    }

    /// Add a DirectedGraph to be executed.
    pub fn add_graph(mut self, graph: DirectedGraph) -> Self {
        self.graphs.push(graph);
        self
    }

    /// Run all DirectedGraphs concurrently and wait for completion or shutdown signal.
    #[instrument(skip(self), fields(graph_count = self.graphs.len()))]
    pub async fn run(self) -> Result<()> {
        info!(count = self.graphs.len(), "starting runtime");

        let mut handles = Vec::new();
        for graph in self.graphs {
            let name = graph.name().to_owned();
            let handle = tokio::spawn(async move {
                info!(graph = %name, "spawning graph");
                if let Err(e) = graph.run().await {
                    error!(graph = %name, error = %e, "graph failed");
                    return Err(e);
                }
                Ok(())
            });
            handles.push(handle);
        }

        tokio::select! {
            _ = async {
                for handle in handles {
                    if let Ok(Err(e)) = handle.await {
                        error!(error = %e, "graph error");
                    }
                }
            } => {
                info!("all graphs completed");
            }
            _ = shutdown_signal() => {
                info!("shutdown signal received, stopping");
            }
        }

        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::graph::DirectedGraphBuilder;
    use crate::layer::{InputLayer, Layer, OutputLayer};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Msg {
        v: String,
    }

    struct In;
    impl Layer for In {
        fn name(&self) -> &str {
            "In"
        }
    }
    #[async_trait]
    impl InputLayer for In {
        type Output = Msg;
        async fn run(&self) -> Result<Msg> {
            Ok(Msg { v: "ok".into() })
        }
    }

    struct Out;
    impl Layer for Out {
        fn name(&self) -> &str {
            "Out"
        }
    }
    #[async_trait]
    impl OutputLayer for Out {
        type Input = Msg;
        async fn run(&self, _input: Msg) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_runtime_runs_multiple_graphs() {
        let graph1 = DirectedGraphBuilder::new("graph1")
            .add_input(In)
            .add_output(Out)
            .add_edge("In", "Out")
            .build()
            .unwrap();

        struct In2;
        impl Layer for In2 {
            fn name(&self) -> &str {
                "In2"
            }
        }
        #[async_trait]
        impl InputLayer for In2 {
            type Output = Msg;
            async fn run(&self) -> Result<Msg> {
                Ok(Msg { v: "ok2".into() })
            }
        }
        struct Out2;
        impl Layer for Out2 {
            fn name(&self) -> &str {
                "Out2"
            }
        }
        #[async_trait]
        impl OutputLayer for Out2 {
            type Input = Msg;
            async fn run(&self, _input: Msg) -> Result<()> {
                Ok(())
            }
        }

        let graph2 = DirectedGraphBuilder::new("graph2")
            .add_input(In2)
            .add_output(Out2)
            .add_edge("In2", "Out2")
            .build()
            .unwrap();

        let runtime = Runtime::new().add_graph(graph1).add_graph(graph2);
        let result = runtime.run().await;
        assert!(result.is_ok());
    }
}

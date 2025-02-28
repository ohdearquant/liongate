//! Lion Workflow Engine
//!
//! A capability-based workflow engine for the Lion microkernel architecture. This crate provides
//! a secure, fault-tolerant, and efficient workflow engine with support for various workflow
//! patterns like directed acyclic graphs, events, and distributed sagas.
//!
//! # Features
//!
//! - Capability-based security: workflows and steps require specific capabilities to execute
//! - Fault tolerance: checkpointing, at-least-once event delivery, saga compensation, etc.
//! - Efficient execution: prioritized scheduling, cooperative preemption, backpressure
//! - Graph-based workflows: directed acyclic graph (DAG) execution with dynamic updates
//! - Event-driven patterns: publish-subscribe with at-least-once/exactly-once semantics
//! - Saga pattern: distributed transactions with compensation for partial failures
//!
//! # Getting Started
//!
//! ```rust,no_run
//! use lion_workflow::{WorkflowDefinition, Node, Edge, NodeId, WorkflowBuilder, EdgeId};
//! use lion_workflow::{WorkflowExecutor, ExecutorConfig};
//! use lion_workflow::engine::executor::NodeHandler;
//! use lion_workflow::state::{StateMachineManager, CheckpointManager, MemoryStorage};
//! use lion_workflow::engine::scheduler::{WorkflowScheduler, SchedulerConfig};
//! use std::sync::Arc;
//!
//! // Define a simple workflow
//! let mut builder = WorkflowBuilder::new("Example Workflow");
//! let node1 = Node::new(NodeId::new(), "Start".to_string());
//! let node1_id = node1.id.clone(); // Clone ID before moving the node
//! let node2 = Node::new(NodeId::new(), "Process".to_string());
//! let node2_id = node2.id.clone(); // Clone ID before moving the node
//! let node3 = Node::new(NodeId::new(), "End".to_string());
//! let node3_id = node3.id.clone(); // Clone ID before moving the node
//!
//! let workflow = builder
//!     .add_node(node1).unwrap()
//!     .add_node(node2).unwrap()
//!     .add_node(node3).unwrap()
//!     .add_edge(Edge::new(EdgeId::new(), node1_id.clone(), node2_id.clone())).unwrap()
//!     .add_edge(Edge::new(EdgeId::new(), node2_id.clone(), node3_id.clone())).unwrap()
//!     .build();
//!
//! // Create execution components
//! let workflow_def = Arc::new(workflow);
//! let scheduler = Arc::new(WorkflowScheduler::new(SchedulerConfig::default()));
//! let state_manager = Arc::new(StateMachineManager::<MemoryStorage>::new());
//! let executor = WorkflowExecutor::new(scheduler, state_manager, ExecutorConfig::default());
//!
//! // Start the executor
//! tokio::runtime::Runtime::new().unwrap().block_on(async {
//!     executor.start().await.unwrap();
//!     
//!     // Execute the workflow
//!     let instance_id = executor.execute_workflow(workflow_def).await.unwrap();
//!     println!("Workflow instance started: {}", instance_id);
//! });
//! ```

/// Core model types and definitions for workflows
pub mod model;

/// State management and persistence
pub mod state;

/// Workflow execution engine
pub mod engine;

/// Common workflow patterns
pub mod patterns;

/// Utility modules for serialization and other helpers
pub mod utils;

// Re-export important types
pub use engine::{
    context::ExecutionContext, context::NodeResult, executor::ExecutorConfig,
    executor::WorkflowExecutor, scheduler::SchedulerConfig, scheduler::SchedulingPolicy,
    scheduler::TaskStatus,
};
pub use model::{
    Edge, EdgeId, Node, NodeId, NodeStatus, WorkflowBuilder, WorkflowDefinition, WorkflowError,
    WorkflowId,
};
pub use patterns::event::{Event, EventBroker};
pub use state::{
    CheckpointManager, FileStorage, MemoryStorage, StateMachineManager, StorageBackend,
    WorkflowState,
};

/// Error types from across the workflow engine
pub mod error {
    pub use crate::engine::context::ContextError;
    pub use crate::engine::executor::ExecutorError;
    pub use crate::engine::scheduler::SchedulerError;
    pub use crate::model::WorkflowError;
    pub use crate::patterns::{EventError, SagaError};
    pub use crate::state::{CheckpointError, StateMachineError, StorageError};
}

/// Create a new workflow definition
pub fn create_workflow(name: &str) -> WorkflowBuilder {
    WorkflowBuilder::new(name)
}

#[cfg(test)]
pub mod integration_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Edge, EdgeId, Node, NodeId};

    #[test]
    fn test_create_workflow() {
        let builder = create_workflow("Test Workflow");
        let node1 = Node::new(NodeId::new(), "Node 1".to_string());
        let node2 = Node::new(NodeId::new(), "Node 2".to_string());

        let node1_id = node1.id.clone();
        let node2_id = node2.id.clone();

        let workflow = builder
            .add_node(node1)
            .unwrap()
            .add_node(node2)
            .unwrap()
            .add_edge(Edge::new(EdgeId::new(), node1_id, node2_id))
            .unwrap()
            .build();

        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.nodes.len(), 2);
        assert_eq!(workflow.edges.len(), 1);
    }
}

#![allow(clippy::new_without_default)]
#![allow(clippy::bool_to_int_with_if)]

use thiserror::Error;

mod agent;
pub use agent::{IceAgent, IceAgentEvent, IceAgentStats, IceConnectionState, IceCreds};

mod candidate;
pub use candidate::{Candidate, CandidateKind};

mod pair;

/// Errors from the ICE agent.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum IceError {
    #[error("ICE bad candidate: {0}")]
    BadCandidate(String),
}
use crate::defines::{MAX_HIDDEN_LAYERS, MAX_NODES_PER_LAYER};
use std::fmt;

#[derive(Debug)]
pub enum AannError {
    NetworkTooManyNodesPerLayer,
    NetworkTooManyHiddenLayers,
    NetworkZeroNodesPerLayer,
    NetworkZeroHiddenLayers,
    Generic(String),
}

impl fmt::Display for AannError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AannError::NetworkTooManyNodesPerLayer => write!(
                f,
                "Too many nodes in one or more layers, expect <= {}",
                MAX_NODES_PER_LAYER
            ),
            AannError::NetworkTooManyHiddenLayers => {
                write!(f, "Too many hidden layers, expect <= {}", MAX_HIDDEN_LAYERS)
            }
            AannError::NetworkZeroNodesPerLayer => write!(f, "Not enough nodes, expect > 0"),
            AannError::NetworkZeroHiddenLayers => write!(f, "Not enough hidden layers, expect > 0"),
            AannError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AannError {}

mod defines;
mod errors;
mod network;

pub use defines::{Count, Decimal};
pub use defines::{MAX_BIASES, MAX_HIDDEN_LAYERS, MAX_NODES_PER_LAYER, MAX_WEIGHTS};
pub use errors::AannError;
pub use network::Network;

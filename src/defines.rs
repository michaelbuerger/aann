pub type Count = usize; // default type for counts
pub type Decimal = f64;

pub const MAX_NODES_PER_LAYER: Count = 15;
pub const MAX_HIDDEN_LAYERS: Count = 5;

const MAX_NON_INPUT_NODES: Count = MAX_NODES_PER_LAYER * (MAX_HIDDEN_LAYERS + 1); // max number of nodes (excl. inputs) based on above maxes
pub const MAX_WEIGHTS: Count = MAX_NON_INPUT_NODES * MAX_NODES_PER_LAYER;
pub const MAX_BIASES: Count = MAX_NON_INPUT_NODES;

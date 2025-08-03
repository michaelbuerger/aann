use aann::{Count, Decimal, MAX_BIASES, MAX_NODES_PER_LAYER, MAX_WEIGHTS, Network};

/* ======== MODIFY THESE ======== */
const NUM_INPUT_NODES: Count = 2;
const NUM_OUTPUT_NODES: Count = 2;
const NUM_HIDDEN_NODES: Count = 3;
const NUM_HIDDEN_LAYERS: Count = 2;
/* ============================== */

// complicated but helps ensure array lengths below are correct
const NUM_WEIGHTS: Count = NUM_INPUT_NODES * NUM_HIDDEN_NODES
    + (if NUM_HIDDEN_LAYERS > 1 {
        (NUM_HIDDEN_LAYERS - 1) * (NUM_HIDDEN_NODES * NUM_HIDDEN_NODES)
    } else {
        0
    })
    + NUM_OUTPUT_NODES * NUM_HIDDEN_NODES;
const NUM_BIASES: Count = NUM_HIDDEN_NODES * NUM_HIDDEN_LAYERS + NUM_OUTPUT_NODES;

/* ======== MODIFY THESE ======== */
const INPUTS: [Decimal; NUM_INPUT_NODES] = [1.1, 0.17];
const WEIGHTS: [Decimal; NUM_WEIGHTS] = [
    1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 1.1, 1.2, 1.3, 0.25, 0.35, 0.45, 2.1, 2.7, 2.9, 0.9, 0.8, 0.7,
    0.6, 0.5, 0.4,
];
const BIASES: [Decimal; NUM_BIASES] = [-5.0, -6.0, -3.0, -1.0, 5.0, -7.0, 1.0, 2.0];
/* ============================== */

/* Entrypoint for executable target */
fn main() {
    // initialize Network object with layout information, panic on error
    let net = Network::new(
        NUM_INPUT_NODES,
        NUM_OUTPUT_NODES,
        NUM_HIDDEN_NODES,
        NUM_HIDDEN_LAYERS,
    )
    .unwrap();

    // full arrays to pass to calculate function
    let mut inputs_full = [0.0; MAX_NODES_PER_LAYER];
    let mut weights_full = [0.0; MAX_WEIGHTS];
    let mut biases_full = [0.0; MAX_BIASES];

    inputs_full[0..INPUTS.len()].copy_from_slice(&INPUTS);
    weights_full[0..WEIGHTS.len()].copy_from_slice(&WEIGHTS);
    biases_full[0..BIASES.len()].copy_from_slice(&BIASES);

    let output: [Decimal; MAX_NODES_PER_LAYER] =
        net.calculate(&inputs_full, &weights_full, &biases_full);

    for num in output {
        print!("{}, ", num);
    }
    print!("\n");
}

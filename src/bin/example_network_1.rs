use aann::{Count, Decimal, MAX_BIASES, MAX_NODES_PER_LAYER, MAX_WEIGHTS, Network};

// USER CONTROLLED CONSTANTS
const NUM_INPUT_NODES: Count = 2;
const NUM_OUTPUT_NODES: Count = 1;
const NUM_HIDDEN_NODES: Count = 3;
const NUM_HIDDEN_LAYERS: Count = 1;

// complicated but helps ensure array lengths below are correct
const NUM_WEIGHTS: Count = NUM_INPUT_NODES * NUM_HIDDEN_NODES
    + (if NUM_HIDDEN_LAYERS > 1 {
        (NUM_HIDDEN_LAYERS - 1) * (NUM_HIDDEN_NODES * NUM_HIDDEN_NODES)
    } else {
        0
    })
    + NUM_OUTPUT_NODES * NUM_HIDDEN_NODES;
const NUM_BIASES: Count = NUM_HIDDEN_NODES * NUM_HIDDEN_LAYERS + NUM_OUTPUT_NODES;

// specific slices we care about
const INPUTS: [Decimal; NUM_INPUT_NODES] = [0.25, 0.75];
const WEIGHTS: [Decimal; NUM_WEIGHTS] = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const BIASES: [Decimal; NUM_BIASES] = [-1.0, -2.0, -3.0, -4.0];

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

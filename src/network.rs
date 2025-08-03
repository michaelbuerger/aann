use crate::AannError;
use crate::defines::*;

#[derive(Debug)]
enum NetworkCalcState {
    HiddenLayer(Count),
    OutputLayer,
    Done,
}

#[derive(Debug)]
pub struct Network {
    input_nodes: Count,
    output_nodes: Count,
    hidden_nodes: Count,
    hidden_layers: Count,
}

impl Network {
    pub fn new(
        input_nodes: Count,
        output_nodes: Count,
        hidden_nodes: Count,
        hidden_layers: Count,
    ) -> Result<Network, AannError> {
        // too tall
        if input_nodes > MAX_NODES_PER_LAYER
            || output_nodes > MAX_NODES_PER_LAYER
            || hidden_nodes > MAX_NODES_PER_LAYER
        {
            return Err(AannError::NetworkTooManyNodesPerLayer);
        }

        if input_nodes == 0 || output_nodes == 0 || hidden_nodes == 0 {
            return Err(AannError::NetworkZeroNodesPerLayer);
        }

        // too many hidden layers
        if hidden_layers > MAX_HIDDEN_LAYERS {
            return Err(AannError::NetworkTooManyHiddenLayers);
        }

        // no hidden layers
        if hidden_layers == 0 {
            return Err(AannError::NetworkZeroHiddenLayers);
        }

        // passed all validations
        return Ok(Network {
            input_nodes: input_nodes,
            output_nodes: output_nodes,
            hidden_nodes: hidden_nodes,
            hidden_layers: hidden_layers,
        });
    }

    // ReLU
    fn activation_function(x: Decimal) -> Decimal {
        if x < 0.0 {
            return 0.0;
        }
        return x;
    }

    // TODO: create non-impl helper func OR alt. struct etc. TO help make weights and bias user friendly THEN produce weights and biases arrays OR call this method
    pub fn calculate(
        &self,
        inputs: &[Decimal; MAX_NODES_PER_LAYER],
        weights: &[Decimal; MAX_WEIGHTS],
        biases: &[Decimal; MAX_BIASES],
    ) -> [Decimal; MAX_NODES_PER_LAYER] {
        let mut calculation_state = NetworkCalcState::HiddenLayer(0);
        let mut propogation_array = [0.0; MAX_NODES_PER_LAYER];
        let mut propogation_length: Count = self.input_nodes;
        propogation_array[0..self.input_nodes].copy_from_slice(&inputs[0..self.input_nodes]);

        let mut weights_index: Count = 0;
        let mut biases_index: Count = 0;

        loop {
            //println!("{:?}", calculation_state); // TODO: remove
            match calculation_state {
                NetworkCalcState::HiddenLayer(n) => {
                    let mut next_propogation_array = [0.0; MAX_NODES_PER_LAYER];
                    for h_index in 0..self.hidden_nodes {
                        let mut acc: Decimal = 0.0;
                        for i in 0..propogation_length {
                            acc += propogation_array[i] * weights[weights_index];
                            weights_index += 1;
                        }
                        acc += biases[biases_index];
                        biases_index += 1;
                        next_propogation_array[h_index] = Network::activation_function(acc);
                    }

                    propogation_length = self.hidden_nodes;
                    propogation_array[0..propogation_length]
                        .copy_from_slice(&next_propogation_array[0..propogation_length]);
                    if n + 1 >= self.hidden_layers {
                        calculation_state = NetworkCalcState::OutputLayer;
                    } else {
                        calculation_state = NetworkCalcState::HiddenLayer(n + 1);
                    }
                }
                NetworkCalcState::OutputLayer => {
                    let mut next_propogation_array = [0.0; MAX_NODES_PER_LAYER];
                    for h_index in 0..self.output_nodes {
                        let mut acc: Decimal = 0.0;
                        for i in 0..propogation_length {
                            acc += propogation_array[i] * weights[weights_index];
                            weights_index += 1;
                        }
                        acc += biases[biases_index];
                        biases_index += 1;
                        next_propogation_array[h_index] = Network::activation_function(acc);
                    }

                    propogation_length = self.output_nodes;
                    propogation_array[0..propogation_length]
                        .copy_from_slice(&next_propogation_array[0..propogation_length]);
                    calculation_state = NetworkCalcState::Done;
                }
                NetworkCalcState::Done => break,
            };
        }

        let mut outputs = [0.0; MAX_NODES_PER_LAYER];
        outputs[0..propogation_length].copy_from_slice(&propogation_array[0..propogation_length]);

        return outputs;
    }
}

use crate::structure::*;
// Logic gate structure
pub struct LogicGate {
    pub input: Vec<Pin>,
    pub output: Pin,
    pub num_input: usize,
    pub r#type: LogicGates
}

impl LogicGate {
    // Constructor to build a new logic gate
    pub fn new_gate(gate_type: u32, num_inputs: usize, bus: bool, bits:usize) -> LogicGate {

        let gate = if !bus {
            // Create a simple logic gate
            LogicGate{
                // Check if number bits is > 1 and return an error
                //...

                // As many inputs vec elements as inputs
                input: vec![Pin {connection: ConnectionType::Simple(Signal::Undefined), r#type: (PinType::NotSource)}; num_inputs],
                output: Pin {connection: ConnectionType::Simple(Signal::Undefined), r#type: (PinType::Source)},
                num_input: num_inputs,
                r#type: match gate_type {
                    0 => LogicGates::And,
                    1 => LogicGates::Or,
                    2 => LogicGates::Not, 
                    3 => LogicGates::Nand,
                    4 => LogicGates::Nor,
                    5 => LogicGates::Xor,
                    6 => LogicGates::Xnor,
                    _ => panic!("Invalid gate"),
                }
            }
            
        } else {
            // Create a bus logic gate
            LogicGate{
                // Check if number of bits are %2 
                //...
                
                // Inner vec elements represent the bit numbers, while outer
                // vec elements represent the number of bus
                input: vec![ Pin {connection: ConnectionType::Bus(vec![Signal::Undefined; bits]), r#type: PinType::NotSource}; num_inputs],
                output: Pin {connection: ConnectionType::Bus(vec![Signal::Undefined; bits]), r#type: PinType::Source},
                num_input: num_inputs,
                r#type: match gate_type {
                    0 => LogicGates::And,
                    1 => LogicGates::Or,
                    2 => LogicGates::Not, 
                    3 => LogicGates::Nand,
                    4 => LogicGates::Nor,
                    5 => LogicGates::Xor,
                    6 => LogicGates::Xnor,
                    _ => panic!("Input non valido per la porta logica"),
                }
            }
        };
        gate
    }

    // Update the input of the gate
    pub fn set_input(&mut self, mut signals: Vec<Signal>) {
        // Fill any missing signals with Signal::Undefined
        if signals.len() < self.num_input {
            signals.resize(self.num_input, Signal::Undefined);
        } else if signals.len() > self.num_input {
            panic!("Too many signals provided for the number of input pins.");
        }

        for (i, signal) in signals.into_iter().enumerate() {
            self.input[i].connection = ConnectionType::Simple(signal);
        }
    }

    // Set input for bus logic gates (each pin has a vector of signals)
    pub fn set_bus_input(&mut self, mut signals: Vec<Vec<Signal>>, bits: usize) {
        // Fill any missing input buses with `Signal::Undefined` of the correct length (number of bits)
        if signals.len() < self.input.len() {
            signals.resize(self.input.len(), vec![Signal::Undefined; bits]);
        } else if signals.len() > self.input.len() {
            panic!("Too many signal vectors provided for the number of input pins.");
        }

        for (i, signal_vec) in signals.into_iter().enumerate() {
            // Set each pin's connection to a bus with the provided signal vector
            self.input[i].connection = ConnectionType::Bus(signal_vec);
        }
    }
    

    // Get the output of a gate
    pub fn get_output(&mut self) {
        // Track some conditions for each gate
        let mut all_on = true;      // For AND/NAND
        let mut all_off = true;     // For NOR
        let mut any_on = false;     // For OR/XOR
        let mut on_count = 0;       // For XOR/XNOR
        let mut any_undefined = false; // To handle Undefined signals
    
        // Go through all input pins and check their signals
        for pin in 0..self.num_input {
            match self.input[pin].connection {
                ConnectionType::Simple(signal) => {
                    match signal {
                        Signal::On => {
                            // For AND/NAND and NOR
                            all_off = false;   // Not all off for NOR
                            on_count += 1;     // For XOR/XNOR
                            any_on = true;     // For OR/XOR
                        }
                        Signal::Off => {
                            // For AND/NAND: Check if all inputs are off
                            all_on = false;    // Not all on for AND/NAND
                            all_off = true;    // For NOR
                        }
                        Signal::Undefined => {
                            any_undefined = true;  // At least one input is Undefined
                            all_on = false;
                            all_off = false;
                        }
                    }
                }
                _ => {
                    panic!("Unexpected connection type, expected Simple.");
                }
            }
        }
    
        // Now apply the logic gate
        match self.r#type {
            LogicGates::And => {
                if all_on {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                }
            }
            
            LogicGates::Or => {
                if any_on {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                }
            }
    
            LogicGates::Not => {
                // The gate requires only one input
                if self.num_input != 1 {
                    panic!("Not gate requires exactly one input");
                }
    
                match self.input[0].connection {
                    ConnectionType::Simple(signal) => {
                        self.output.connection = match signal {
                            Signal::On => ConnectionType::Simple(Signal::Off),
                            Signal::Off => ConnectionType::Simple(Signal::On),
                            Signal::Undefined => ConnectionType::Simple(Signal::Undefined),
                        };
                    }
                    _ => {
                        panic!("Unexpected connection type for Not gate, expected Simple.");
                    }
                }
            }
    
            LogicGates::Nand => {
                if all_on {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                }
            }
    
            LogicGates::Nor => {
                if all_off {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                }
            }
    
            LogicGates::Xor => {
                if on_count % 2 == 1 {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                }
            }
    
            LogicGates::Xnor => {
                if on_count % 2 == 0 {
                    self.output.connection = ConnectionType::Simple(Signal::On);
                } else if any_undefined {
                    self.output.connection = ConnectionType::Simple(Signal::Undefined);
                } else {
                    self.output.connection = ConnectionType::Simple(Signal::Off);
                }
            }
    
            _ => panic!("Gate type not handled"),
        }
    }
}

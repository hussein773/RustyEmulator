use std::clone;

use crate::structure::*;
use crate::circuit::*;
use ggez::{graphics::{Canvas, DrawParam, Image}, Context, GameResult};
use ggez::glam::Vec2;

// Logic gate structure
#[derive(Debug)]
pub struct LogicGate {
    pub input: Vec<Pin>,
    pub output: Pin,
    pub num_input: usize,
    pub r#type: LogicGates,
    pub position: ggez::glam::Vec2,  
}

impl LogicGate {
    // Constructor to build a new logic gate
    pub fn new_gate(gate_type: u32, num_inputs: usize, bus: bool, bits:usize) -> LogicGate {

        let gate = if !bus {
            // Create a simple logic gate
            LogicGate{
                //TODO: ignore the number of bits and create a simple gate
                //...

                // As many inputs vec elements as inputs
                input: vec![Pin{value: PinValue::Single(Signal::Undefined), r#type: PinType::NotSource}; num_inputs],
                output: Pin {value: PinValue::Single(Signal::Undefined), r#type: PinType::Source},
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
                },
                position: Vec2::new(0.0, 0.0), 
            }
            
        } else {
            // Create a bus logic gate
            LogicGate{
                // Check if number of bits are %2 
                //...
                
                // Inner vec elements represent the bit numbers, while outer
                // vec elements represent the number of bus
                input: vec![Pin{value:PinValue::Multiple(vec![Signal::Undefined; bits]), r#type: PinType::NotSource}],
                output: Pin {value:PinValue::Multiple(vec![Signal::Undefined; bits]), r#type:PinType::Source},
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
                },
                position: Vec2::new(0.0, 0.0), 
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
            self.input[i].value = PinValue::Single(signal);
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
            self.input[i].value = PinValue::Multiple(signal_vec);
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
            match self.input[pin].value {
                //* Check if it's a 1 bit gate
                PinValue::Single(signal) => {
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
                    self.output.value = PinValue::Single(Signal::On);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::Off);
                }
            }
            
            LogicGates::Or => {
                if any_on {
                    self.output.value = PinValue::Single(Signal::On);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::Off);
                }
            }
    
            LogicGates::Not => {
                // The gate requires only one input
                if self.num_input != 1 {
                    panic!("Not gate requires exactly one input");
                }
    
                match self.input[0].value {
                    PinValue::Single(signal) => {
                        self.output.value = match signal {
                            Signal::On => PinValue::Single(Signal::Off),
                            Signal::Off => PinValue::Single(Signal::On),
                            Signal::Undefined => PinValue::Single(Signal::Undefined),
                        };
                    }
                    _ => {
                        panic!("Unexpected connection type for Not gate, expected Simple.");
                    }
                }
            }
    
            LogicGates::Nand => {
                if all_on {
                    self.output.value = PinValue::Single(Signal::Off);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::On);
                }
            }
    
            LogicGates::Nor => {
                if all_off {
                    self.output.value = PinValue::Single(Signal::On);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::Off);
                }
            }
    
            LogicGates::Xor => {
                if on_count % 2 == 1 {
                    self.output.value = PinValue::Single(Signal::On);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::Off);
                }
            }
    
            LogicGates::Xnor => {
                if on_count % 2 == 0 {
                    self.output.value = PinValue::Single(Signal::On);
                } else if any_undefined {
                    self.output.value = PinValue::Single(Signal::Undefined);
                } else {
                    self.output.value = PinValue::Single(Signal::Off);
                }
            }
    
            _ => panic!("Gate type not handled"),
        }
    }

}

impl clone::Clone for LogicGate {
    fn clone(&self) -> Self {
        LogicGate {
            input: self.input.clone(),
            output: self.output.clone(),
            num_input: self.num_input,
            r#type: self.r#type.clone(),
            position: self.position.clone(),
        }
    }
    
}

impl PartialEq for LogicGate {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input &&
        self.output == other.output &&
        self.num_input == other.num_input &&
        self.r#type == other.r#type &&
        self.position == other.position 
    }
    
}

impl LogicElement for LogicGate {
    fn get_pin_mut(&mut self, pin_index: usize) -> Option<&mut Pin> {
            if pin_index < self.input.len() {
                self.input.get_mut(pin_index)
            } else if pin_index == self.input.len() {
                Some(&mut self.output)
            } else {
                None
            }
        }
}
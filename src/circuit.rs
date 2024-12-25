use std::path::Component;
use std::pin;
use crate::logic_gates::*;
use crate::structure::*;

#[derive(Debug)]
pub enum LogicElements {
    Gates(LogicGate),
    Source,
    Clock,
    Adders,
    Multiplexers,
    Demultiplexers,
    ShiftRegisters,
    FlipFlops,
    LatchRegisters,
}

impl LogicElements {
    pub fn set_input(&mut self, signal: Vec<Signal>) {
        match self {
            // Handle the case when it's a single input gate
            LogicElements::Gates(logic_gate) => {
                logic_gate.set_input(signal);  // Call the method for a single input gate
            }

            // Handle the case when it's a Source element
            LogicElements::Source => {
                todo!()
            }

            // Handle the case when it's a Clock element
            LogicElements::Clock => {
                todo!()
            }

            // Handle cases where the logic element is a flip flop or latch register
            LogicElements::FlipFlops | LogicElements::LatchRegisters => {
                todo!()
            }

            // Fallback case: for any other variant of LogicElements
            _ => {
                panic!("set_input is not supported for this LogicElement variant");
            }
        }
    }

    // Function to elaborate the output
    pub fn get_output(&mut self) {
        match self {
            LogicElements::Gates(logic_gate) => {
                logic_gate.get_output();
            }
            _ => todo!(),
        }
    }

    pub fn get_pin(&mut self, ioc:usize, pid:usize) -> &mut Pin{
        match self {
            LogicElements::Gates(logic_gate) => logic_gate.get_pin(ioc, pid),
            _ => todo!("rest of the componens"),
        }
    }
}

impl Clone for LogicElements {
    fn clone(&self) -> Self {
        match self {
            LogicElements::Gates(gate) => LogicElements::Gates(gate.clone()),
            LogicElements::Source => LogicElements::Source,
            LogicElements::Clock => LogicElements::Clock,
            LogicElements::Adders => LogicElements::Adders,
            LogicElements::Multiplexers => LogicElements::Multiplexers,
            LogicElements::Demultiplexers => LogicElements::Demultiplexers,
            LogicElements::ShiftRegisters => LogicElements::ShiftRegisters,
            LogicElements::FlipFlops => LogicElements::FlipFlops,
            LogicElements::LatchRegisters => LogicElements::LatchRegisters,
        }
    }
}


pub struct Circuit {
    components: Vec<LogicElements>,
    connections: Vec<Vec<(usize,usize,usize)>>,
    component_id: usize
}
impl Circuit {
    // Create a new circuit
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            connections: Vec::new(),
            component_id: 1,
        }
    }
    

    //* Function to connect pins of different components together
    pub fn connect(&mut self, from_cid: usize, from_ioc: usize, from_pid: usize, to_cid: usize, to_ioc: usize, to_pid: usize) {
        // Create tuples for both pins
        let from_pin_id = (from_cid, from_ioc, from_pid);
        let to_pin_id = (to_cid, to_ioc, to_pid);

        // Try to merge the connections
        let mut merged = false;

        for connection in &mut self.connections {
            // If either pin is already part of the connection, merge the connection
            if connection.iter().any(|&pin_id| pin_id == from_pin_id) {
                connection.push(to_pin_id);
                merged = true;
                break;
            } else if connection.iter().any(|&pin_id| pin_id == to_pin_id) {
                connection.push(from_pin_id);
                merged = true;
                break;
            }
        }

        // If no merge happened, create a new connection with both pins
        if !merged {
            self.connections.push(vec![from_pin_id, to_pin_id]);
        }
    }

    // Function to add logic elements to the circuit
    pub fn add_element(&mut self, mut component: LogicElements) {
        // Set the id of the component
        match &mut component {
            LogicElements::Gates(logic_gate) => logic_gate.set_gate_id(self.component_id),
            //TODO ADD THE REST OF THE COMPONENTS
            _ => todo!(),
        }
        self.component_id += 1;
        //println!("added component {:?}", component);
        self.components.push(component);
    }

    // Function to delete an element from the circuit
    pub fn remove_element(&mut self, id: usize) {
        self.components.remove(id);
    }

    pub fn simulate(&mut self) {
        // Check if all the connections are correct (contain a single source pin)
        for (index, connection) in self.connections.iter().enumerate() {
            // Buffer to hold the source pins
            let mut source_pins: Vec<&(usize, usize, usize)> = connection.iter().filter(|&&(cid, ioc, pid)| ioc == 0).collect();
            if source_pins.len() > 1 {
                panic!(
                    "Short circuit detected in connection {}: {:?}. More than one source pin (ioc = 0) found.",
                    index, connection
                );
            }
    
            // We have a single source pin
            // Propagate the signal of the source pin to the other pin in the connection
            let source = source_pins.pop();
    
            if let Some(&(cid, ioc, pid)) = source {
                // Access the source pin using the get_pin method
                let source_value = self.components[cid-1].get_pin(ioc, pid).value.clone(); // Clone the value
                
                /*println!(
                    "Propagating signal from source pin: cid = {}, ioc = {}, pid = {}, value = {:?}",
                    cid, ioc, pid, source_value
                );*/
    
                // Propagate the source signal to all other pins in the connection
                for &(target_cid, target_ioc, target_pid) in connection {
                    if target_cid == cid && target_ioc == ioc && target_pid == pid {
                        // Skip the source pin itself
                        continue;
                    }
    
                    // Get the target pin and set its value
                    self.components[target_cid-1]
                        .get_pin(target_ioc, target_pid)
                        .value = source_value.clone(); 
                }
            } else {
                // If no source pin exists then set all pins to undefined
                println!(
                    "No source pin found in connection {}: {:?}. Setting all pins to undefined.",
                    index, connection
                );
                for &(target_cid, target_ioc, target_pid) in connection {
                    self.components[target_cid-1]
                        .get_pin(target_ioc, target_pid)
                        .value = PinValue::Single(Signal::Undefined);
                }
            }
            // After propagating the signals get the output of each component
            for component in &mut self.components{
                component.get_output();
            }
        }
    }

    // TODO: function need to be modified to account for possible components with more than 1 output
    pub fn display_outputs(&self){
        for (index, component) in self.components.iter().enumerate(){
            let mut out = self.components[index].clone();
            println!("output is {:?}", out.get_pin(0, 1).value)
        }
    }
}


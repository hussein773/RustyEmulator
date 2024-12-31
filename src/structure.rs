// structure.rs

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Signal {
    Off, 
    On, 
    Undefined, 
}

#[derive(Debug, Clone)]
pub enum LogicGates {
    And,
    Or,
    Not,
    Nand, 
    Nor,
    Xor, 
    Xnor,
}

#[derive(Debug, Clone)]
pub enum PinValue {
    Single(Signal),
    Multiple(Vec<Signal>),
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub value: PinValue,
    pub cid: usize,
    pub pid: usize,
    pub ioc: usize,     //* The IOC (Input Output Control) parameter indicates what's the purpouse of the pin, 0 = output, 1 = input, 2.. = control pins
}

#[derive(Debug, Clone)]
pub struct WireSegment {
    pub start: (i32, i32), // Start coordinate of the segment
    pub end: (i32, i32),   // End coordinate of the segment
}

#[derive(Debug, Clone)]
pub struct Wire {
    pub pins: Vec<(usize, usize, usize)>, // Logical connections (component ID, I/O category, pin ID)
    pub segments: Vec<WireSegment>,      // Physical segments of the wire
}
impl Wire {
    pub fn iter(&self) -> std::slice::Iter<'_, (usize, usize, usize)> {
        self.pins.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (usize, usize, usize)> {
        self.pins.iter_mut()
    }
}


//TODO: Helper function to get / set the value of a pin
/*impl Pin {
    pub fn get_value(&self) -> PinValue {
        match &self.value {
            PinValue::Single(signal) => PinValue::Single(*signal),
            PinValue::Multiple(signals) => PinValue::Multiple(signals.clone()),
        }
    }

    pub fn set_value(&self, value: Signal) -> PinValue {
        Pin {
            value,
            r#type: self.r#type.clone(),
        }
    }
}*/


impl PartialEq for PinValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PinValue::Single(signal1), PinValue::Single(signal2)) => signal1 == signal2,
            (PinValue::Multiple(vec1), PinValue::Multiple(vec2)) => vec1 == vec2,
            _ => false,
        }
    }
}

impl PartialEq for Pin {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && 
        self.pid == other.pid &&
        self.cid == other.cid &&
        self.ioc == other.ioc
    } 
}


impl PartialEq for LogicGates {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogicGates::And, LogicGates::And) => true,
            (LogicGates::Or, LogicGates::Or) => true,
            (LogicGates::Not, LogicGates::Not) => true,
            (LogicGates::Nand, LogicGates::Nand) => true,
            (LogicGates::Nor, LogicGates::Nor) => true,
            (LogicGates::Xor, LogicGates::Xor) => true,
            (LogicGates::Xnor, LogicGates::Xnor) => true,
            _ => false,
        }
    }
    
}
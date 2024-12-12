// components.rs
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
pub enum PinType {
    Source,
    NotSource,
}

#[derive(Debug, Clone)]
pub enum PinValue {
    Single(Signal),
    Multiple(Vec<Signal>),
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub value: PinValue,
    pub r#type: PinType,
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

#[derive(Debug, Clone)]
pub struct Bus_Pin {
    pub value: Vec<Signal>,
    pub r#type: PinType,
}

//* Helper function to get / set the value of a bus pin
impl Bus_Pin {
    pub fn get_value(&self) -> Vec<Signal> {
        self.value.clone()
    }

    pub fn set_value(&self, value: Vec<Signal>) -> Bus_Pin {
        Bus_Pin {
            value,
            r#type: self.r#type.clone(),
        }
    }
}

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
        self.value == other.value && self.r#type == other.r#type
    } 
}

impl PartialEq for Bus_Pin {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.r#type == other.r#type
    } 

}

impl PartialEq for PinType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PinType::Source, PinType::Source) => true,
            (PinType::NotSource, PinType::NotSource) => true,
            _ => false,
        }
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
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
pub enum ConnectionType {
    Simple(Signal),
    Bus(Vec<Signal>),
}

#[derive(Debug, Clone)]
pub enum PinType {
    Source,
    NotSource,
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub connection: ConnectionType,
    pub r#type: PinType,
}
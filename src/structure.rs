use std::fmt;

// structure.rs
use ggez::{graphics::Rect, mint::Point2};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Signal {
    Off, 
    On, 
    Undefined, 
}
impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Signal::Off => "OFF",
            Signal::On => "ON",
            Signal::Undefined => "UNDEFINED",
        };
        write!(f, "{}", s)
    }
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

impl fmt::Display for PinValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PinValue::Single(signal) => write!(f, "{}", signal),
            PinValue::Multiple(signals) => {
                let formatted_signals: Vec<String> = signals.iter().map(|s| s.to_string()).collect();
                write!(f, "[{}]", formatted_signals.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub value: PinValue,
    pub cid: usize,
    pub pid: usize,
    pub ioc: usize,     //* The IOC (Input Output Control) parameter indicates what's the purpouse of the pin, 0 = output, 1 = input, 2.. = control pins
    pub hitbox: Hitbox,
    //TODO: instead of 3 separate parameters unite them in a single tuple
}

#[derive(Debug, Clone, PartialEq)]
pub enum HitboxType{
    Pin(usize, usize, usize),
    Wire,
    Component,
}

#[derive(Debug, Clone)]
pub struct Hitbox{
    pub rect: Rect, 
    pub r#type: HitboxType,
}

#[derive(Debug, Clone)]
pub struct WireSegment {
    pub start: Point2<f32>,   
    pub end: Point2<f32>,     
    pub hitbox: Hitbox,       
}

#[derive(Debug, Clone)]
pub struct Wire {
    pub pins: Vec<(usize, usize, usize)>, // Logical connections (component ID, I/O category, pin ID)
    pub segments: Vec<WireSegment>,       // The segments of the wire
}
impl Wire {
    pub fn iter(&self) -> std::slice::Iter<'_, (usize, usize, usize)> {
        self.pins.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (usize, usize, usize)> {
        self.pins.iter_mut()
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

pub struct UnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(), // Each node is its own parent
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}
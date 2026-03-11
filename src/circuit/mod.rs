pub mod circuit;
pub mod history;

use num_complex::Complex64;
use crate::gates::Gate;

pub use history::OpcodeHistory;

pub struct Circuit {
    pub qubits: usize,
    pub state: Vec<Complex64>,
    pub commands: Vec<Box<dyn Gate>>,
    pub register: Vec<usize>
}

#[macro_export]
macro_rules! gate {
    ($gate_name:ident, 1, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, target: usize) {
            use crate::gates::Gate;
            let g = $gate_struct { target }; 
            g.apply(self);
        }
    };

    ($gate_name:ident, 2, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, control: usize, target: usize) {
            use crate::gates::Gate;
            let g = $gate_struct { control, target }; 
            g.apply(self);
        }
    };

    ($gate_name:ident, 3, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, control1: usize, control2: usize, target: usize) {
            use crate::gates::Gate;
            use crate::gates::$gate_struct; 
            
            let g = $gate_struct { control1, control2, target }; 
            g.apply(self);
        }
    };
}

#[macro_export]
macro_rules! gateo {
    ($gate_name:ident, 1, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, target: usize) {
            use crate::gates::$gate_struct;
            self.commands.push(Box::new($gate_struct { target }));
        }
    };

    ($gate_name:ident, 2, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, control: usize, target: usize) {
            use crate::gates::$gate_struct;
            self.commands.push(Box::new($gate_struct { control, target }));
        }
    };
    
    ($gate_name:ident, 3, $gate_struct:ident) => {
        pub fn $gate_name(&mut self, control1: usize, control2: usize, target: usize) {
            use crate::gates::$gate_struct;
            self.commands.push(Box::new($gate_struct { control1, control2, target }));
        }
    };
}
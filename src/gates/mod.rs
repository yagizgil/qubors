use crate::circuit::Circuit;
pub mod ccnot;
pub mod cnot;
pub mod hadamard;
pub mod x;
pub mod z;

pub use ccnot::CCNot;
pub use cnot::CNot;
pub use hadamard::Hadamard;
pub use x::X;
pub use z::Z;

pub trait Gate {
    fn apply(&self, circuit: &mut Circuit);
    fn gclone(&self) -> Box<dyn Gate>;
}

#[macro_export]
macro_rules! gclone {
    ($gate:ident,1) => {
        fn gclone(&self) -> Box<dyn Gate> {
            Box::new($gate {target: self.target})
        }
    };

    ($gate:ident,2) => {
        fn gclone(&self) -> Box<dyn Gate> {
            Box::new($gate {control: self.control, target: self.target})
        }
    };

    ($gate:ident,3) => {
        fn gclone(&self) -> Box<dyn Gate> {
            Box::new($gate {control1: self.control1, control2: self.control1, target: self.target})
        }
    };
}

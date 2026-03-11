use crate::gates::Gate;
use num_complex::Complex64;

use crate::gclone;

pub struct Hadamard {
    pub target: usize,
}

impl Gate for Hadamard {
    fn apply(&self, circuit: &mut crate::circuit::Circuit) {
        let size = circuit.state.len();
        let step = 1 << self.target; 
        let inv_sqrt2 = Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0);

        for i in (0..size).step_by(2 * step) {
            for j in 0..step {
                let i0 = i + j;
                let i1 = i + j + step;

                let a = circuit.state[i0];
                let b = circuit.state[i1];

                circuit.state[i0] = (a + b) * inv_sqrt2;
                circuit.state[i1] = (a - b) * inv_sqrt2;
            }
        }
    }

    gclone!(Hadamard,1);
}
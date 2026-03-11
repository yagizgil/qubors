use crate::{circuit::Circuit, gate, gateo, gates::Gate};
use num_complex::Complex64;
use rand::{RngExt};

use crate::gates::{CNot, Hadamard, X, Z};

impl Circuit {
    pub fn new(_qubits: usize) -> Self {
        let size = 1 << _qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        state[0] = Complex64::new(1.0, 0.0);

        Circuit {
            qubits: _qubits,
            state: state,
            commands: Vec::new()
        }
    }

    pub fn reset(&mut self, i: usize) {
        self.state = vec![num_complex::Complex64::new(0.0, 0.0); self.state.len()];
        self.state[i] = num_complex::Complex64::new(1.0, 0.0);
    }

    gate!(h, 1, Hadamard);
    gateo!(_h, 1, Hadamard);

    gate!(x, 1, X);
    gateo!(_x, 1, X);

    gate!(cx, 2, CNot);
    gateo!(_cx, 2, CNot);

    gate!(z, 1, Z);
    gateo!(_z, 1, Z);

    gate!(ccx, 3, CCNot);
    gateo!(_ccx, 3, CCNot);

    pub fn measure(&mut self) -> usize {
        let mut rng = rand::rng();
        let rval: f64 = rng.random();

        let mut cum = 0.0;
        let mut col = 0;

        for (i, amplitude) in self.state.iter().enumerate() {
            cum += amplitude.norm_sqr();

            if rval <= cum {
                col = i;
                break;
            }
        }

        self.reset(col);
        col
    }

    pub fn execute(&mut self) -> usize {

        let commands: Vec<Box<dyn Gate>> = self.commands
        .iter()
        .map(|g| g.gclone())
        .collect();

        for _g in commands {
            _g.apply(self);
        }

        self.measure()
    }

    pub fn execute_res(&mut self) -> usize {
        self.reset(0);
        self.execute()
    }

    pub fn clear_commands(&mut self) {
        self.commands = Vec::new();
    }
}

use crate::{circuit::Circuit, gate, gateo, gates::Gate};
use num_complex::Complex64;
use rand::RngExt;

use crate::gates::{CNot, Hadamard, X, Z};

impl Circuit {
    pub fn new(_qubits: usize) -> Self {
        let size = 1 << _qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        state[0] = Complex64::new(1.0, 0.0);

        Circuit {
            qubits: _qubits,
            state: state,
            commands: Vec::new(),
            register: vec![0; _qubits],
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

    pub fn execute(&mut self) -> usize {
        self.register.fill(0);

        let commands: Vec<Box<dyn Gate>> = self.commands.iter().map(|g| g.gclone()).collect();

        for _g in commands {
            _g.apply(self);
        }

        let final_state_index = self.measure();
        for i in 0..self.qubits {
            self.register[i] = (final_state_index >> i) & 1;
        }

        let mut final_res = 0;
        for (i, &val) in self.register.iter().enumerate() {
            if val == 1 {
                final_res |= 1 << i;
            }
        }

        final_res
    }

    pub fn execute_res(&mut self) -> usize {
        self.reset(0);
        self.execute()
    }

    #[inline]
    pub fn clear_commands(&mut self) {
        self.commands = Vec::new();
    }

    pub fn measure(&mut self) -> usize {
        for (i, amplitude) in self.state.iter().enumerate() {
            if amplitude.norm_sqr() > 0.999 {
                return i;
            }
        }

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

    pub fn measure_to(&mut self, qubit: usize, c_bit: usize) -> usize {
        let result_index = self.measure();

        for i in 0..self.qubits {
            self.register[i] = (result_index >> i) & 1;
        }

        (result_index >> qubit) & 1
    }

    pub fn _measure(&mut self, qubit: usize, c_bit: usize) {
        //self.measure_to(qubit, c_bit);

        use crate::gates::conditional::MeasureGate;
        self.commands.push(Box::new(MeasureGate { qubit, c_bit }));
    }

    pub fn c_if<G: Gate + 'static>(&mut self, c_bit: usize, val: usize, gate: G) {
        use crate::gates::conditional::IfGate;
        self.commands.push(Box::new(IfGate {
            c_bit,
            val,
            target_gate: Box::new(gate),
        }));
    }

    pub fn c_choice<G1, G2>(&mut self, c_bit: usize, g1: G1, g0: G2)
    where
        G1: Gate + 'static,
        G2: Gate + 'static,
    {
        use crate::gates::conditional::ChoiceGate;
        self.commands.push(Box::new(ChoiceGate {
            c_bit,
            if_1: Box::new(g1),
            if_0: Box::new(g0),
        }));
    }
}

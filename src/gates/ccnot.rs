use crate::gates::Gate;
use crate::gclone;

pub struct CCNot {
    pub control1: usize,
    pub control2: usize,
    pub target: usize,
}

impl Gate for CCNot {
    fn apply(&self, circuit: &mut crate::circuit::Circuit) {
        let c1_mask = 1 << self.control1;
        let c2_mask = 1 << self.control2;
        let step = 1 << self.target;
        let size = circuit.state.len();

        for i in (0..size).step_by(2 * step) {
            for j in 0..step {
                let i0 = i + j;
                let i1 = i + j + step;

                if (i0 & c1_mask) != 0 && (i0 & c2_mask) != 0 {
                    circuit.state.swap(i0, i1);
                }
            }
        }
    }

    
    gclone!(CCNot,3);
}
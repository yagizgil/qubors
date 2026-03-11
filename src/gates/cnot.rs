use crate::gates::Gate;
use crate::gclone;

pub struct CNot{
    pub control: usize,
    pub target: usize,
}

impl Gate for CNot {
    fn apply(&self, circuit: &mut crate::circuit::Circuit) {
        let size = circuit.state.len();
        let step = 1 << self.target;
        let control_mask = 1 << self.control;

        for i in (0..size).step_by(2 * step) {
            for j in 0..step {
                let i0 = i + j;
                let i1 = i + j + step;

                if (i0 & control_mask) != 0 {
                    circuit.state.swap(i0, i1);
                }
            }
        }
    }

    
    gclone!(CNot,2);
}

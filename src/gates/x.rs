use crate::gates::Gate;
use crate::gclone;

pub struct X{
    pub target: usize,
}

impl Gate for X {
    fn apply(&self, circuit: &mut crate::circuit::Circuit) {
        let size = circuit.state.len();
        let step = 1 << self.target;

        for i in (0..size).step_by(2 * step) {
            for j in 0..step {
                let i0 = i + j;
                let i1 = i + j + step;

                circuit.state.swap(i0, i1);
            }
        }
    }

    gclone!(X,1);
}
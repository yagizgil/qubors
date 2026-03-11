use crate::gates::Gate;
use crate::gclone;

pub struct Z {
    pub target: usize,
}

impl Gate for Z {
    fn apply(&self, circuit: &mut crate::circuit::Circuit) {
        let target = self.target;
        let size = circuit.state.len();
        let step = 1 << target;

        for i in (0..size).step_by(2 * step) {
            for j in 0..step {
                let i1 = i + j + step; 
                circuit.state[i1] = -circuit.state[i1];
            }
        }
    }

    
    gclone!(Z,1);
}
use crate::circuit::Circuit;
use crate::gates::Gate;
use crate::implgate;
pub struct IfGate {
    pub c_bit: usize,
    pub val: usize,
    pub target_gate: Box<dyn Gate>,
}

impl Gate for IfGate {
    fn apply(&self, circuit: &mut Circuit) {
        if circuit.register[self.c_bit] == self.val {
            self.target_gate.apply(circuit);
        }
    }

    fn gclone(&self) -> Box<dyn Gate> {
        Box::new(IfGate {
            c_bit: self.c_bit,
            val: self.val,
            target_gate: self.target_gate.gclone(),
        })
    }

    fn get_target(&self) -> usize {
        self.target_gate.get_target()
    }

    fn name(&self) -> String {
        format!(
            "if(c[{}]=={})->{}",
            self.c_bit,
            self.val,
            self.target_gate.name()
        )
    }
}

pub struct ChoiceGate {
    pub c_bit: usize,
    pub if_1: Box<dyn Gate>,
    pub if_0: Box<dyn Gate>,
}

impl Gate for ChoiceGate {
    fn apply(&self, circuit: &mut Circuit) {
        if circuit.register[self.c_bit] == 1 {
            self.if_1.apply(circuit);
        } else {
            self.if_0.apply(circuit);
        }
    }

    fn gclone(&self) -> Box<dyn Gate> {
        Box::new(ChoiceGate {
            c_bit: self.c_bit,
            if_1: self.if_1.gclone(),
            if_0: self.if_0.gclone(),
        })
    }

    fn get_target(&self) -> usize {
        self.if_1.get_target()
    }

    fn name(&self) -> String {
        format!(
            "choice(c[{}])?{}:{}",
            self.c_bit,
            self.if_1.name(),
            self.if_0.name()
        )
    }
}

pub struct MultiIfGate {
    pub c_bits: Vec<usize>,
    pub target_gate: Box<dyn Gate>,
}

impl Gate for MultiIfGate {
    fn apply(&self, circuit: &mut Circuit) {
        if self.c_bits.iter().all(|&b| circuit.register[b] == 1) {
            self.target_gate.apply(circuit);
        }
    }

    fn gclone(&self) -> Box<dyn Gate> {
        Box::new(MultiIfGate {
            c_bits: self.c_bits.clone(),
            target_gate: self.target_gate.gclone(),
        })
    }

    fn get_target(&self) -> usize {
        self.target_gate.get_target()
    }

    fn name(&self) -> String {
        format!("if(c{:?}==1)->{}", self.c_bits, self.target_gate.name())
    }
}

pub struct MeasureGate {
    pub qubit: usize,
    pub c_bit: usize,
}

impl Gate for MeasureGate {
    fn apply(&self, circuit: &mut Circuit) {
        circuit.measure_to(self.qubit, self.c_bit);
    }

    fn gclone(&self) -> Box<dyn Gate> {
        Box::new(MeasureGate {
            qubit: self.qubit,
            c_bit: self.c_bit,
        })
    }

    fn get_target(&self) -> usize {
        self.qubit
    }
    fn name(&self) -> String {
        format!("measure(q{} -> c{})", self.qubit, self.c_bit)
    }
}

use crate::{Circuit, gates::Gate};

pub struct OpcodeHistory {
    commands: Vec<Box<dyn Gate>>,
}

impl OpcodeHistory {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.commands = Vec::new()
    }

    #[inline]
    pub fn push<T>(&mut self, g: T)
    where
        T: Into<Box<dyn Gate>>,
    {
        self.commands.push(g.into());
    }

    #[inline]
    pub fn push_all(&mut self, circuit: &Circuit) {
        self.commands
            .extend(circuit.commands.iter().map(|g| g.gclone()));
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&dyn Gate> {
        self.commands.get(index).map(|g| g.as_ref())
    }

    pub fn get_by_qubit(&self, qubit: usize) -> Vec<&dyn Gate> {
        self.commands
            .iter()
            .map(|g| g.as_ref())
            .filter(|g| g.get_target() == qubit)
            .collect()
    }

    pub fn get_str_list(&self) -> Vec<String> {
        self.commands
            .iter()
            .map(|g| g.name()) 
            .collect()
    }
}

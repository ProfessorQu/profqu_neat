pub trait Gene {
    fn get_innovation_number(&self) -> u32;
    fn set_innovation_number(&mut self, new: u32);
}
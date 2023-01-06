/// The trait that defines get_innovation_number and set_innovation_number for [`NodeGene`](crate::genome::NodeGene) and [`ConnectionGene`](crate::genome::ConnectionGene)
pub trait Gene {
    fn get_innovation_number(&self) -> u32;
    fn set_innovation_number(&mut self, new: u32);
}
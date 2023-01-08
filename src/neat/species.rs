use crate::data_structures::RandomHashSet;

use super::Client;

#[derive(PartialEq, Debug, Clone)]
pub struct Species {
    clients: Vec<Client>,
}


const FRACT_MULT: f32 = 1e6;

/// A really stupid workaround for float not implementing 'Eq'
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct PseudoFloat {
    float: String
}

impl PseudoFloat {
    /// Create a new PseudoFloat
    pub fn new(float: f32) -> Self {
        Self {
            float: float.to_string()
        }
    }
}

impl From<f32> for PseudoFloat {
    fn from(item: f32) -> Self {
        Self {
            float: item.to_string()
        }
    }
}

impl Into<f32> for PseudoFloat {
    fn into(self) -> f32 {
        self.float.parse().expect("String stored in PseudoFloat is not a f32")
    }
}
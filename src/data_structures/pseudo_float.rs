/// A really stupid workaround for float not implementing 'Eq'
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PseudoFloat {
    float: u32
}

impl PseudoFloat {
    /// Create a new PseudoFloat
    pub fn new(float: f32) -> Self {
        Self {
            float: float.to_bits()
        }
    }

    /// Parse into a string
    pub fn parse(&self) -> f32 {
        f32::from_bits(self.float)
    }
}

impl From<f32> for PseudoFloat {
    fn from(item: f32) -> Self {
        Self {
            float: item.to_bits()
        }
    }
}

impl Into<f32> for PseudoFloat {
    fn into(self) -> f32 {
        f32::from_bits(self.float)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        for _ in 0..10 {
            let float: f32 = rand::random();
            let pseudo: PseudoFloat = float.into();
            let float2: f32 = pseudo.into();

            assert_eq!(float, float2);
        }
    }

    #[test]
    fn test_into() {
        for _ in 0..10 {
            let pseudo = PseudoFloat::new(rand::random());
            let float: f32 = pseudo.clone().into();
            let pseudo2 = float.into();

            assert_eq!(pseudo, pseudo2);
        }
    }
}
use std::fmt;

/// A workaround for float not implementing `Eq`
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct PseudoFloat {
    float: u32
}

impl PseudoFloat {
    /// Create a new PseudoFloat from a float
    /// ```rust
    /// use profqu_neat::data_structures::PseudoFloat;
    /// 
    /// let float = 3.1415;
    /// let pseudo = PseudoFloat::new(float);
    /// 
    /// assert_eq!(float, pseudo.into());
    /// ```
    pub fn new(float: f32) -> Self {
        Self {
            float: float.to_bits()
        }
    }

    /// Parse into a float from the bits
    /// ```rust
    /// use profqu_neat::data_structures::PseudoFloat;
    /// 
    /// let float = 3.1415;
    /// let pseudo = PseudoFloat::new(float);
    /// 
    /// assert_eq!(float, pseudo.parse());
    /// ```
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

impl From<PseudoFloat> for f32 {
    fn from(val: PseudoFloat) -> Self {
        f32::from_bits(val.float)
    }
}

impl fmt::Debug for PseudoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f32::from_bits(self.float))
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
            let float: f32 = pseudo.into();
            let pseudo2 = float.into();

            assert_eq!(pseudo, pseudo2);
        }
    }
}
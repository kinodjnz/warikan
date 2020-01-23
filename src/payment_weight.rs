use crate::sum::Sum;
use crate::unital_magma::UnitalMagma;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentWeight(pub i32);

impl PaymentWeight {
    pub fn new(value: i32) -> Self {
        PaymentWeight(value)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PaymentWeightSum(pub i32);

impl UnitalMagma for PaymentWeightSum {
    fn mempty() -> Self {
        PaymentWeightSum(0)
    }
}

impl Sum<PaymentWeight> for PaymentWeightSum {
    fn add(self, weight: PaymentWeight) -> PaymentWeightSum {
        PaymentWeightSum(self.0 + weight.0)
    }
}

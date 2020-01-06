use crate::additive_category::AdditiveCategory;
use crate::unital_magma::UnitalMagma;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentWeight(pub i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PaymentWeightSum(i32);

impl PaymentWeightSum {
    pub fn payment_ratio_for_unit_weight(self) -> PaymentRatioForUnitWeight {
        let PaymentWeightSum(s) = self;
        PaymentRatioForUnitWeight(1.0 / (s as f64))
    }
}

impl UnitalMagma for PaymentWeightSum {
    fn mempty() -> Self {
        PaymentWeightSum(0)
    }
}

pub struct PaymentWeightSumCalculator;

impl AdditiveCategory<PaymentWeight, PaymentWeightSum> for PaymentWeightSumCalculator {
    fn add(sum: PaymentWeightSum, weight: PaymentWeight) -> PaymentWeightSum {
        PaymentWeightSum(sum.0 + weight.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentRatioForUnitWeight(pub f64);

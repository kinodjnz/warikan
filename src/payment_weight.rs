#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentWeight(pub i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PaymentWeightSum(i32);

impl PaymentWeightSum {
    pub fn add_weight(self, payment_weight: PaymentWeight) -> PaymentWeightSum {
        let PaymentWeightSum(mut s) = self;
        s += payment_weight.0;
        PaymentWeightSum(s)
    }

    pub fn payment_ratio_for_unit_weight(self) -> PaymentRatioForUnitWeight {
        let PaymentWeightSum(s) = self;
        PaymentRatioForUnitWeight(1.0 / (s as f64))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentRatioForUnitWeight(pub f64);

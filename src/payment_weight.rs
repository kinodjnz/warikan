use crate::charge_amount::ChargeAmount;
use crate::payment_amount::{PaymentAmountClassification, PaymentAmountPerUnit};

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

    pub fn payment_amount_per_unit(self, charge_amount: ChargeAmount) -> PaymentAmountPerUnit {
        let PaymentWeightSum(s) = self;
        PaymentAmountPerUnit((charge_amount.0 as f64) / ((s as f64) / 100.0))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentWeightForAmountClassification {
    for_larger: PaymentWeight,
    for_medium: PaymentWeight,
    for_smaller: PaymentWeight,
}

impl PaymentWeightForAmountClassification {
    pub fn new(
        for_larger: PaymentWeight,
        for_medium: PaymentWeight,
        for_smaller: PaymentWeight,
    ) -> PaymentWeightForAmountClassification {
        PaymentWeightForAmountClassification {
            for_larger,
            for_medium,
            for_smaller,
        }
    }

    pub fn payment_weight(&self, classification: PaymentAmountClassification) -> PaymentWeight {
        match classification {
            PaymentAmountClassification::Larger => self.for_larger,
            PaymentAmountClassification::Medium => self.for_medium,
            PaymentAmountClassification::Smaller => self.for_smaller,
        }
    }
}

use crate::payment_weight::PaymentWeight;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PaymentAmountClassification {
    Larger,
    Medium,
    Smaller,
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

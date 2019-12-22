use crate::charge_amount::ChargeAmount;
use crate::payment_amount::PaymentAmountPerUnit;

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

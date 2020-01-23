use crate::additive_ratio::AdditiveRatio;
use crate::charge_amount::ChargeAmount;
use crate::payment_amount::PaymentAmount;
use crate::payment_weight::{PaymentWeight, PaymentWeightSum};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentRatio(f64);

impl AdditiveRatio<PaymentWeight, PaymentWeightSum, PaymentAmount, ChargeAmount> for PaymentRatio {
    fn of_sums(weights: PaymentWeightSum, charge_amount: ChargeAmount) -> Self {
        PaymentRatio((charge_amount.0 as f64) / (weights.0 as f64))
    }

    fn apply_to(&self, weight: PaymentWeight) -> PaymentAmount {
        PaymentAmount(((weight.0 as f64) * self.0) as i32)
    }
}

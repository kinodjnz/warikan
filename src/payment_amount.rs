use crate::payment_weight::PaymentWeight;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmount(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentAmountPerUnitWeight(pub f64);

impl PaymentAmountPerUnitWeight {
    pub fn payment_amount(self, payment_weight: PaymentWeight) -> PaymentAmount {
        let payment_amount_f64 = self.0 * (payment_weight.0 as f64);
        PaymentAmount(payment_amount_f64 as i32)
    }
}

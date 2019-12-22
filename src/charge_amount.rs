use crate::payment_amount::PaymentAmountPerUnitWeight;
use crate::payment_weight::PaymentRatioForUnitWeight;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct ChargeAmount(pub i32);

impl ChargeAmount {
    pub fn payment_amount_per_unit_weight(
        self,
        payment_ratio: PaymentRatioForUnitWeight,
    ) -> PaymentAmountPerUnitWeight {
        PaymentAmountPerUnitWeight(payment_ratio.0 * (self.0 as f64))
    }
}

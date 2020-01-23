use crate::payment_amount::PaymentAmount;
use crate::sum::Sum;
use crate::unital_magma::UnitalMagma;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct ChargeAmount(pub i32);

impl UnitalMagma for ChargeAmount {
    fn mempty() -> Self {
        ChargeAmount(0)
    }
}

impl Sum<PaymentAmount> for ChargeAmount {
    fn add(self, payment: PaymentAmount) -> ChargeAmount {
        ChargeAmount(self.0 + payment.0)
    }
}

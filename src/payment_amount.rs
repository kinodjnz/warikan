use crate::participant::Participant;
use crate::payment_weight::PaymentWeight;
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmount(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentAmountPerUnit(pub f64);

impl PaymentAmountPerUnit {
    pub fn payment_amount(self, payment_weight: PaymentWeight) -> PaymentAmount {
        let PaymentAmountPerUnit(payment_amount_per_unit) = self;
        let payment_amount_f64 = payment_amount_per_unit * (payment_weight.0 as f64);
        PaymentAmount(payment_amount_f64 as i32)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmountForParticipant {
    participant: Participant,
    payment_amount: PaymentAmount,
}

impl PaymentAmountForParticipant {
    pub fn new(
        participant: Participant,
        payment_amount: PaymentAmount,
    ) -> PaymentAmountForParticipant {
        PaymentAmountForParticipant {
            participant,
            payment_amount,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmountsForParticipants(Vec<PaymentAmountForParticipant>);

impl FromIterator<PaymentAmountForParticipant> for PaymentAmountsForParticipants {
    fn from_iter<I: IntoIterator<Item = PaymentAmountForParticipant>>(iter: I) -> Self {
        PaymentAmountsForParticipants(Vec::from_iter(iter))
    }
}

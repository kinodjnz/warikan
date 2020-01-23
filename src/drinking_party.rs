use crate::additive_ratio::AdditiveRatio;
use crate::charge_amount::ChargeAmount;
use crate::participant::{Participants, PaymentAmountsForParticipants};
use crate::payment_amount_classification::PaymentWeightForAmountClassification;
use crate::payment_ratio::PaymentRatio;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DrinkingParty {
    name: String,
    participants: Participants,
    weight: PaymentWeightForAmountClassification,
}

impl DrinkingParty {
    pub fn new(
        name: String,
        participants: Participants,
        weight: PaymentWeightForAmountClassification,
    ) -> DrinkingParty {
        DrinkingParty {
            name,
            participants,
            weight,
        }
    }

    pub fn payment_amounts_for_participants(
        &self,
        charge_amount: ChargeAmount,
    ) -> PaymentAmountsForParticipants {
        let payment_weight_sum = self.participants.sum_payment_weight(&self.weight);
        let ratio = PaymentRatio::of_sums(payment_weight_sum, charge_amount);
        self.participants.payment_amounts(ratio, &self.weight)
    }
}

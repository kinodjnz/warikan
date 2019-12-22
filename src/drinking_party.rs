use crate::charge_amount::ChargeAmount;
use crate::participant::Participants;
use crate::payment_amount::PaymentAmountsForParticipants;
use crate::payment_amount_classification::PaymentWeightForAmountClassification;

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
        let payment_ratio_for_unit_weight = self
            .participants
            .sum_payment_weight(&self.weight)
            .payment_ratio_for_unit_weight();
        let payment_amount_per_unit =
            charge_amount.payment_amount_per_unit(payment_ratio_for_unit_weight);
        self.participants
            .payment_amounts(payment_amount_per_unit, &self.weight)
    }
}

use crate::payment_amount::{
    PaymentAmountForParticipant, PaymentAmountPerUnit, PaymentAmountsForParticipants,
};
use crate::payment_amount_classification::{
    PaymentAmountClassification, PaymentWeightForAmountClassification,
};
use crate::payment_weight::{PaymentWeight, PaymentWeightSum};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participants(pub Vec<Participant>);

impl Participants {
    pub fn sum_payment_weight(
        &self,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentWeightSum {
        let Participants(participants) = self;
        participants
            .iter()
            .fold(PaymentWeightSum::default(), |sum, participant| {
                participant.sum_payment_weight(weight, sum)
            })
    }

    pub fn payment_amounts(
        &self,
        payment_amount_per_unit: PaymentAmountPerUnit,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountsForParticipants {
        let Participants(participants) = self;
        participants
            .iter()
            .map(|participant| participant.payment_amount(payment_amount_per_unit, weight))
            .collect::<PaymentAmountsForParticipants>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participant {
    name: String,
    payment_amount_classification: PaymentAmountClassification,
}

impl Participant {
    pub fn new(
        name: String,
        payment_amount_classification: PaymentAmountClassification,
    ) -> Participant {
        Participant {
            name,
            payment_amount_classification,
        }
    }

    fn payment_weight(&self, weight: &PaymentWeightForAmountClassification) -> PaymentWeight {
        weight.payment_weight(self.payment_amount_classification)
    }

    pub fn sum_payment_weight(
        &self,
        weight: &PaymentWeightForAmountClassification,
        audend: PaymentWeightSum,
    ) -> PaymentWeightSum {
        audend.add_weight(self.payment_weight(weight))
    }

    pub fn payment_amount(
        &self,
        payment_amount_per_unit: PaymentAmountPerUnit,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountForParticipant {
        let payment_weight = self.payment_weight(weight);
        PaymentAmountForParticipant::new(
            self.clone(),
            payment_amount_per_unit.payment_amount(payment_weight),
        )
    }
}

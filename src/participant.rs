use crate::additive_ratio::AdditiveRatio;
use crate::payment_amount::PaymentAmount;
use crate::payment_amount_classification::{
    PaymentAmountClassification, PaymentWeightForAmountClassification,
};
use crate::payment_ratio::PaymentRatio;
use crate::payment_weight::{PaymentWeight, PaymentWeightSum};
use crate::sum::Sum;
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participants(pub Vec<Participant>);

impl Participants {
    pub fn sum_payment_weight(
        &self,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentWeightSum {
        let Participants(participants) = self;
        PaymentWeightSum::of_iter(
            participants
                .iter()
                .map(|participant| participant.payment_weight(weight)),
        )
    }

    pub fn payment_amounts(
        &self,
        ratio: PaymentRatio,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountsForParticipants {
        let Participants(participants) = self;
        participants
            .iter()
            .map(|participant| participant.payment_amount(ratio, weight))
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

    pub fn payment_amount(
        &self,
        ratio: PaymentRatio,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountForParticipant {
        let payment_weight = self.payment_weight(weight);
        PaymentAmountForParticipant::new(self.clone(), ratio.apply_to(payment_weight))
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

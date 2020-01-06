use crate::additive_category::AdditiveCategory;
use crate::payment_amount::{PaymentAmount, PaymentAmountPerUnitWeight};
use crate::payment_amount_classification::{
    PaymentAmountClassification, PaymentWeightForAmountClassification,
};
use crate::payment_weight::{PaymentWeight, PaymentWeightSum, PaymentWeightSumCalculator};
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participants(pub Vec<Participant>);

impl Participants {
    pub fn sum_payment_weight(
        &self,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentWeightSum {
        let Participants(participants) = self;
        PaymentWeightSumCalculator::sum(
            participants
                .iter()
                .map(|participant| participant.payment_weight(weight)),
        )
    }

    pub fn payment_amounts(
        &self,
        payment_amount_per_unit_weight: PaymentAmountPerUnitWeight,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountsForParticipants {
        let Participants(participants) = self;
        participants
            .iter()
            .map(|participant| participant.payment_amount(payment_amount_per_unit_weight, weight))
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
        payment_amount_per_unit_weight: PaymentAmountPerUnitWeight,
        weight: &PaymentWeightForAmountClassification,
    ) -> PaymentAmountForParticipant {
        let payment_weight = self.payment_weight(weight);
        PaymentAmountForParticipant::new(
            self.clone(),
            payment_amount_per_unit_weight.payment_amount(payment_weight),
        )
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

mod additive_category;
mod charge_amount;
mod drinking_party;
mod participant;
mod payment_amount;
mod payment_amount_classification;
mod payment_weight;
mod unital_magma;

use crate::charge_amount::ChargeAmount;
use crate::drinking_party::DrinkingParty;
use crate::participant::{Participant, Participants, PaymentAmountsForParticipants};
use crate::payment_amount_classification::{
    PaymentAmountClassification, PaymentWeightForAmountClassification,
};
use crate::payment_weight::PaymentWeight;

fn drinking_party_service(
    drinking_party: DrinkingParty,
    charge_amount: ChargeAmount,
) -> PaymentAmountsForParticipants {
    drinking_party.payment_amounts_for_participants(charge_amount)
}

fn main() {
    let drinking_party = DrinkingParty::new(
        "hoge party".into(),
        Participants(vec![
            Participant::new("member1".into(), PaymentAmountClassification::Larger),
            Participant::new("member2".into(), PaymentAmountClassification::Medium),
            Participant::new("member3".into(), PaymentAmountClassification::Smaller),
        ]),
        PaymentWeightForAmountClassification::new(
            PaymentWeight(150),
            PaymentWeight(100),
            PaymentWeight(50),
        ),
    );
    let charge_amount = ChargeAmount(5000);
    println!(
        "{:?}",
        drinking_party_service(drinking_party, charge_amount)
    );
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DrinkingParty {
    name: String,
    participants: Participants,
    rate: PaymentRateForAmountClassification,
}

impl DrinkingParty {
    fn payment_amounts_for_participants(
        &self,
        charge_amount: ChargeAmount,
    ) -> PaymentAmountsForParticipants {
        let payment_amount_per_unit = self
            .participants
            .sum_payment_rate(&self.rate)
            .payment_amount_per_unit(charge_amount);
        self.participants
            .payment_amounts(payment_amount_per_unit, &self.rate)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participants(Vec<Participant>);

impl Participants {
    pub fn sum_payment_rate(&self, rate: &PaymentRateForAmountClassification) -> PaymentRateSum {
        let Participants(participants) = self;
        participants
            .iter()
            .fold(PaymentRateSum::default(), |sum, participant| {
                participant.sum_payment_rate(rate, sum)
            })
    }

    pub fn payment_amounts(
        &self,
        payment_amount_per_unit: PaymentAmountPerUnit,
        rate: &PaymentRateForAmountClassification,
    ) -> PaymentAmountsForParticipants {
        let Participants(participants) = self;
        participants
            .iter()
            .map(|participant| participant.payment_amount(payment_amount_per_unit, rate))
            .collect::<PaymentAmountsForParticipants>()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Participant {
    name: String,
    payment_amount_classification: PaymentAmountClassification,
}

impl Participant {
    pub fn payment_rate(&self, rate: &PaymentRateForAmountClassification) -> PaymentRate {
        rate.payment_rate(&self.payment_amount_classification)
    }

    pub fn sum_payment_rate(
        &self,
        rate: &PaymentRateForAmountClassification,
        audend: PaymentRateSum,
    ) -> PaymentRateSum {
        audend.add(self.payment_rate(rate))
    }

    pub fn payment_amount(
        &self,
        payment_amount_per_unit: PaymentAmountPerUnit,
        rate: &PaymentRateForAmountClassification,
    ) -> PaymentAmountForPariticipant {
        let payment_rate = self.payment_rate(rate);
        PaymentAmountForPariticipant {
            participant: self.clone(),
            payment_amount: payment_amount_per_unit.payment_amount(payment_rate),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentRate {
    percent: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct ChargeAmount(i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PaymentRateSum(i32);

impl PaymentRateSum {
    pub fn add(&self, payment_rate: PaymentRate) -> PaymentRateSum {
        let PaymentRateSum(mut s) = self;
        s += payment_rate.percent;
        PaymentRateSum(s)
    }

    pub fn payment_amount_per_unit(&self, charge_amount: ChargeAmount) -> PaymentAmountPerUnit {
        let PaymentRateSum(s) = self;
        PaymentAmountPerUnit((charge_amount.0 as f64) / ((*s as f64) / 100.0))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PaymentAmountPerUnit(f64);

impl PaymentAmountPerUnit {
    pub fn payment_amount(&self, payment_rate: PaymentRate) -> PaymentAmount {
        let PaymentAmountPerUnit(payment_amount_per_unit) = self;
        let payment_amount_f64 = *payment_amount_per_unit * ((payment_rate.percent as f64) / 100.0);
        PaymentAmount(payment_amount_f64 as i32)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PaymentAmountClassification {
    Larger,
    Medium,
    Smaller,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentRateForAmountClassification {
    for_larger: PaymentRate,
    for_medium: PaymentRate,
    for_smaller: PaymentRate,
}

impl PaymentRateForAmountClassification {
    fn payment_rate(&self, classification: &PaymentAmountClassification) -> PaymentRate {
        match classification {
            PaymentAmountClassification::Larger => self.for_larger,
            PaymentAmountClassification::Medium => self.for_medium,
            PaymentAmountClassification::Smaller => self.for_smaller,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmount(i32);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmountForPariticipant {
    participant: Participant,
    payment_amount: PaymentAmount,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PaymentAmountsForParticipants(Vec<PaymentAmountForPariticipant>);

impl std::iter::FromIterator<PaymentAmountForPariticipant> for PaymentAmountsForParticipants {
    fn from_iter<I: IntoIterator<Item = PaymentAmountForPariticipant>>(iter: I) -> Self {
        PaymentAmountsForParticipants(Vec::from_iter(iter))
    }
}

fn drinking_party_service(
    drinking_party: DrinkingParty,
    charge_amount: ChargeAmount,
) -> PaymentAmountsForParticipants {
    drinking_party.payment_amounts_for_participants(charge_amount)
}

fn main() {
    let drinking_party = DrinkingParty {
        name: "hoge party".into(),
        participants: Participants(vec![
            Participant {
                name: "member1".into(),
                payment_amount_classification: PaymentAmountClassification::Larger,
            },
            Participant {
                name: "member2".into(),
                payment_amount_classification: PaymentAmountClassification::Medium,
            },
            Participant {
                name: "member3".into(),
                payment_amount_classification: PaymentAmountClassification::Smaller,
            },
        ]),
        rate: PaymentRateForAmountClassification {
            for_larger: PaymentRate { percent: 150 },
            for_medium: PaymentRate { percent: 100 },
            for_smaller: PaymentRate { percent: 50 },
        },
    };
    let charge_amount = ChargeAmount(5000);
    println!(
        "{:?}",
        drinking_party_service(drinking_party, charge_amount)
    );
}

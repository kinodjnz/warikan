use crate::sum::Sum;

pub trait AdditiveRatio<
    SourceObject,
    SourceSum: Sum<SourceObject>,
    MorphedObject,
    MorphedSum: Sum<MorphedObject>,
>
{
    fn of_sums(source_sum: SourceSum, morphed_sum: MorphedSum) -> Self;

    fn apply_to(&self, source: SourceObject) -> MorphedObject;
}

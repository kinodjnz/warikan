use crate::unital_magma::UnitalMagma;

pub trait AdditiveCategory<SourceObject, Sum: UnitalMagma> {
    fn add(augend: Sum, source: SourceObject) -> Sum;

    fn sum<I: Iterator<Item = SourceObject>>(iter: I) -> Sum {
        iter.fold(UnitalMagma::mempty(), Self::add)
    }
}

use crate::unital_magma::UnitalMagma;

pub trait Sum<SourceObject>: UnitalMagma + Sized {
    fn add(self, source: SourceObject) -> Self;

    fn of_iter<I: Iterator<Item = SourceObject>>(iter: I) -> Self {
        iter.fold(UnitalMagma::mempty(), Self::add)
    }
}

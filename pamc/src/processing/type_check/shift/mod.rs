use super::*;

mod impl_shift;

pub trait ShiftDbIndices {
    type Output;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        amount: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, F::ShiftError>;

    fn upshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(UpshiftFn(amount), 0, registry)
            .safe_unwrap()
    }

    fn upshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(UpshiftFn(amount), cutoff, registry)
            .safe_unwrap()
    }

    fn downshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift(amount, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift(
        self,
        amount: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(amount, 0, registry)
    }

    fn downshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(amount, cutoff, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(DownshiftFn(amount), cutoff, registry)
    }
}

pub trait ShiftFn: Copy {
    type ShiftError;
    fn try_apply(&self, i: DbIndex, cutoff: usize) -> Result<DbIndex, Self::ShiftError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpshiftFn(pub usize);

impl ShiftFn for UpshiftFn {
    type ShiftError = Infallible;
    fn try_apply(&self, i: DbIndex, cutoff: usize) -> Result<DbIndex, Infallible> {
        if i.0 < cutoff {
            Ok(i)
        } else {
            Ok(DbIndex(i.0 + self.0))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DownshiftFn(pub usize);

impl ShiftFn for DownshiftFn {
    type ShiftError = DbIndexTooSmallForDownshiftError;
    fn try_apply(
        &self,
        i: DbIndex,
        cutoff: usize,
    ) -> Result<DbIndex, DbIndexTooSmallForDownshiftError> {
        if i.0 < cutoff {
            Ok(i)
        } else if i.0 < self.0 {
            Err(DbIndexTooSmallForDownshiftError {
                db_index: i,
                downshift_amount: self.0,
            })
        } else {
            Ok(DbIndex(i.0 - self.0))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIndexTooSmallForDownshiftError {
    db_index: DbIndex,
    downshift_amount: usize,
}

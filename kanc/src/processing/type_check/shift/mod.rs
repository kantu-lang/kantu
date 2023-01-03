use super::*;

mod impl_shift;

pub trait ShiftDbIndices {
    type Output;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, F::ShiftError>;

    fn upshift(self, f: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Upshift(f), 0, registry)
            .safe_unwrap()
    }

    fn upshift_with_cutoff(
        self,
        f: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Upshift(f), cutoff, registry)
            .safe_unwrap()
    }

    fn downshift(self, f: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift(f, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift(
        self,
        f: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(f, 0, registry)
    }

    fn downshift_with_cutoff(
        self,
        f: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(f, cutoff, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift_with_cutoff(
        self,
        f: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Downshift(f), cutoff, registry)
    }
}

pub trait ShiftFn: Copy {
    type ShiftError;
    fn try_apply(&self, i: DbIndex, cutoff: usize) -> Result<DbIndex, Self::ShiftError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Upshift(pub usize);

impl ShiftFn for Upshift {
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
pub struct Downshift(pub usize);

impl ShiftFn for Downshift {
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

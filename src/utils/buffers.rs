use std::{
    cell::RefCell,
    mem,
    ops::{Deref, DerefMut},
};

impl<T, const N: usize> Reusable for smallvec::SmallVec<T, N> {
    fn reuse(&mut self) -> Option<Self> {
        if self.capacity() > N {
            self.clear();
            Some(std::mem::take(self))
        } else {
            None
        }
    }
}

impl<T> Reusable for Vec<T> {
    fn reuse(&mut self) -> Option<Self> {
        if self.capacity() > 0 {
            self.clear();
            Some(std::mem::take(self))
        } else {
            None
        }
    }
}

pub trait Reusable: Default {
    /// Expected implementation:
    /// ```
    /// if self.is_worth_to_reuse() {
    ///     self.clear();
    ///     Some(std::mem::take(self))
    /// } else{
    ///     None
    /// }
    /// ```
    fn reuse(&mut self) -> Option<Self>;
}

pub struct ReusablePool<T: Reusable> {
    to_reuse: RefCell<Vec<T>>,
}

pub struct Reused<'a, T: Reusable> {
    context: &'a ReusablePool<T>,
    data: T,
}

impl<T: Reusable> Default for ReusablePool<T> {
    fn default() -> Self {
        ReusablePool {
            to_reuse: RefCell::new(Vec::new()),
        }
    }
}

impl<T: Reusable> ReusablePool<T> {
    pub fn take_buffer(&self) -> Reused<T> {
        Reused {
            context: self,
            data: self.take_buffer_owned(),
        }
    }

    /// The buffer will not be reused
    pub fn take_buffer_owned(&self) -> T {
        self.to_reuse
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| T::default())
    }
}

impl<T: Reusable> Reused<'_, T> {
    pub fn take_owned(mut self) -> T {
        mem::take(&mut self.data)
    }
}

impl<T: Reusable> Deref for Reused<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Reusable> DerefMut for Reused<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Reusable> Drop for Reused<'_, T> {
    fn drop(&mut self) {
        if let Some(reused) = self.reuse() {
            self.context.to_reuse.borrow_mut().push(reused)
        }
    }
}

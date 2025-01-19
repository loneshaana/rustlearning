use std::{borrow::Borrow, ops::Deref};

/// Cow : Clone on write.
/// Cow is the enum which either can be Borrowed or Owned.
///
/// Cow is a smart pointer providing clone-on-write functionality it can enclose and provide immutable access to borrowed
/// data and clone the data lazily when mutation or ownership is required.
/// the type is designed to work with general borrowed data via the Borrow trait
///
/// Cow implements Deref which means that you can call non-mutating methods directly on the data it encloses.
/// If mutation is desired to_mut will obtain a mutable reference to an owned value, cloning if necessary
///

pub enum Cow<'a, B: ?Sized + 'a>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

impl<'a, B: ?Sized> Borrow<B> for Cow<'a, B>
where
    B: ToOwned,
{
    fn borrow(&self) -> &B {
        &**self
    }
}

impl<'a, B: ?Sized + ToOwned> Deref for Cow<'_, B> {
    type Target = B;
    fn deref(&self) -> &B {
        match *self {
            Cow::Borrowed(b) => b,
            Cow::Owned(ref o) => o.borrow(),
        }
    }
}

impl<B: ?Sized + ToOwned> Clone for Cow<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            Cow::Borrowed(b) => Cow::Borrowed(b),
            Cow::Owned(ref o) => {
                let b: &B = o.borrow();
                Cow::Owned(b.to_owned())
            }
        }
    }
}

// impl<T> ToOwned for T
// where
//     T: Clone,
// {
//     type Owned = T;
//     fn to_owned(&self) -> Self::Owned {
//         self.clone()
//     }

//     fn clone_into(&self, target: &mut Self::Owned) {
//         target.clone_from(self);
//     }
// }

impl<B: ?Sized + ToOwned> Cow<'_, B> {
    pub fn is_borrowed(&self) -> bool {
        match *self {
            Cow::Borrowed(_) => true,
            _ => false,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Cow::Borrowed(borrowed) => {
                *self = Cow::Owned(borrowed.to_owned());
                match *self {
                    Cow::Borrowed(_) => unreachable!(),
                    Cow::Owned(ref mut owned) => owned,
                }
            }
            Cow::Owned(ref mut owned) => owned,
        }
    }

    pub fn into_owned(self) -> <B as ToOwned>::Owned {
        match self {
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
            Cow::Owned(owned) => owned,
        }
    }
}

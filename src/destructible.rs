/* Some resources must be marked for destruction explicitly, while other
 * resources may be collected automatically by Bitprim when the explicit
 * destructor is called on this end.
 * Objects that may need to be destroyed explicitly by us are Destructible.
 * Putting them inside a DestructibleBox marks them to be explicitly destroyed.
 */

use std::ops::Deref;

pub trait Destructible {
    fn destruct(&self);
}

macro_rules! derive_destructible {
  ($struct:ty, $ptr:ty, $destructor:ident) => (
    impl Destructible for $struct {
      fn destruct(&self) {
        unsafe{ $destructor(self.raw) }
      }
    }

    extern {
      pub fn $destructor(res: $ptr);
    }
  )
}

pub struct DestructibleBox<T: Destructible> {
    pub contents: Box<T>,
}

impl<T: Destructible> DestructibleBox<T> {
    pub fn new(it: T) -> DestructibleBox<T> {
        DestructibleBox {
            contents: Box::new(it),
        }
    }
}

impl<T: Destructible> Drop for DestructibleBox<T> {
    fn drop(&mut self) {
        println!("Destruction is here");
        self.contents.destruct()
    }
}

impl<T: Destructible> Deref for DestructibleBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.contents
    }
}

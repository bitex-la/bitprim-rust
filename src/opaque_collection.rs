pub trait OpaqueCollection {
    type Item;
    fn len(&self) -> u64;
    fn get(&self, n: u32) -> Self::Item;
    fn is_empty(&self) -> bool;
}

pub struct OpaqueCollectionIterator<'a, T: 'a + OpaqueCollection> {
    pub collection: &'a T,
    pub iter: u32,
}

impl<'a, I, T: OpaqueCollection<Item = I>> Iterator for OpaqueCollectionIterator<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter;
        if current == self.collection.len() as u32 {
            self.iter = 0;
            None
        } else {
            self.iter += 1;
            Some(self.collection.get(current))
        }
    }
}

macro_rules! derive_opaque_collection {
    ($collection:ty, $collection_ptr:ty,
     $item:tt, $item_ptr:ty,
     $extern_count:ident, $extern_nth:ident) => (
         impl OpaqueCollection for $collection {
             type Item = $item;

             fn len(&self) -> u64 {
                 unsafe{ $extern_count(self.raw) }
             }

             fn get(&self, n: u32) -> $item {
                 $item::new(unsafe{ $extern_nth(self.raw, n) })
             }

             fn is_empty(&self) -> bool {
                 self.len() == 0
             }
         }

         impl<'a> IntoIterator for &'a $collection {
             type Item = $item;
             type IntoIter = OpaqueCollectionIterator<'a, $collection>;

             fn into_iter(self) -> Self::IntoIter {
                 OpaqueCollectionIterator{collection: self, iter: 0}
             }
         }

         extern {
             pub fn $extern_count(list: $collection_ptr) -> u64;
             pub fn $extern_nth(list: $collection_ptr, n: u32)
                 -> $item_ptr;
         }
         )
}

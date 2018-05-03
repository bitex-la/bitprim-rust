macro_rules! opaque_resource {
  ( 
    $(#[$($meta:meta),*])*
    $enum:ident, $ptr:ident, $struct:ident {
      $($name:ident: $type:ty, default: $default:expr;)*
    }
    $(pub_fields: {
      $($pub_name:ident: $pub_type:ty),*
    })*
  ) => (
    pub enum $enum {}
    pub type $ptr = *mut $enum;

    $(#[$($meta),*])*
    pub struct $struct{
      pub raw: $ptr,
      $($name: $type,)*
      $($($pub_name: $pub_type),*)*
    }

    impl $struct {
      pub fn new( raw: $ptr, $($($pub_name: $pub_type),*)*) -> $struct {
        $struct{raw, $($name: $default,)* $($($pub_name),*)*}
      }
    }
  )
}

macro_rules! opaque_drop {
  ($struct:ty, $ptr:ty, $destructor:ident) => (
    impl $struct {
      pub fn destruct(&self) {
        unsafe{ $destructor(self.raw) }
      }
    }

    impl Drop for $struct {
      fn drop(&mut self) {
        if self.destruct_on_drop {
          self.destruct()
        }
      }
    }

    extern {
      pub fn $destructor(res: $ptr);
    }
  )
}

macro_rules! opaque_droppable_resource {
  (
    $(#[$($meta:meta),*])*
    $enum:ident, $ptr:ident, $struct:ident {
      $($name:ident: $type:ty, default: $default:expr;)*
    }
    $(pub_fields: { $($pub_name:ident: $pub_type:ty),* },)*
    drop: $destructor:ident
  ) => (
    opaque_resource!{
      $(#[$($meta),*])*
      $enum, $ptr, $struct {
        destruct_on_drop: bool, default: true;
        $($name: $type, default: $default;)*
      }
      $(pub_fields: { $($pub_name: $pub_type),* },)*
    }
    opaque_drop!{ $struct, $ptr, $destructor }
  )
}

macro_rules! opaque_collection {
  ($collection:ty, $collection_ptr:ty,
   $item:tt, $item_ptr:ty,
   $extern_count:ident, $extern_nth:ident) => (
    impl $collection {
      pub fn len(&self) -> u64 {
        unsafe{ $extern_count(self.raw) }
      }

      pub fn get(&self, n: u32) -> $item {
        $item::new(unsafe{ $extern_nth(self.raw, n) })
      }
    }

    impl Iterator for $collection {
      type Item = $item;

      fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter;
        if current == self.len() as u32 {
          self.iter = 0;
          None
        } else {
          self.iter += 1;
          Some(self.get(current))
        }
      }
    }

    extern { 
      pub fn $extern_count(list: $collection_ptr) -> u64;
      pub fn $extern_nth(list: $collection_ptr, n: u32)
        -> $item_ptr;
    }
  )
}

macro_rules! opaque_collection_destructible_inherit {
  ($collection:ty, $collection_ptr:ty,
   $item:tt, $item_ptr:ty,
   $extern_count:ident, $extern_nth:ident) => (
    impl $collection {
      pub fn len(&self) -> u64 {
        unsafe{ $extern_count(self.raw) }
      }

      pub fn get(&self, n: u32) -> $item {
        $item::destructible(unsafe{ $extern_nth(self.raw, n) }, self.destruct_on_drop)
      }
    }

    impl Iterator for $collection {
      type Item = $item;

      fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter;
        if current == self.len() as u32 {
          self.iter = 0;
          None
        } else {
          self.iter += 1;
          Some(self.get(current))
        }
      }
    }

    extern { 
      pub fn $extern_count(list: $collection_ptr) -> u64;
      pub fn $extern_nth(list: $collection_ptr, n: u32)
        -> $item_ptr;
    }
  )
}

macro_rules! opaque_collection_destructible {
  ($collection:ty, $collection_ptr:ty,
   $item:tt, $item_ptr:ty,
   $extern_count:ident, $extern_nth:ident) => (
    impl $collection {
      pub fn len(&self) -> u64 {
        unsafe{ $extern_count(self.raw) }
      }

      pub fn get(&self, n: u32) -> $item {
        $item::destructible(unsafe{ $extern_nth(self.raw, n) }, false)
      }
    }

    impl Iterator for $collection {
      type Item = $item;

      fn next(&mut self) -> Option<Self::Item> {
        let current = self.iter;
        if current == self.len() as u32 {
          self.iter = 0;
          None
        } else {
          self.iter += 1;
          Some(self.get(current))
        }
      }
    }

    extern { 
      pub fn $extern_count(list: $collection_ptr) -> u64;
      pub fn $extern_nth(list: $collection_ptr, n: u32)
        -> $item_ptr;
    }
  )
}


/* Most functions in the bitprim API have async and sync variants.
   Both variants receive exactly the same arguments and produce the same values.
   Async versions get their produced values as arguments to a handler.
   Sync versions receive pointers to store the returned values.
*/
macro_rules! async_and_sync_methods {
  ( $struct:tt, $ptr:ty,
    $({ 
        $extern_async:ident: $async:ident,
        $extern_sync:ident: $sync:ident,
        in: $in:tt,
        out: $out:tt
      }),*
  ) => {
    $(
      extern_async_and_sync_methods! {
        $struct, $ptr, { 
          $extern_async: $async,
          $extern_sync: $sync,
          in: $in,
          out: $out
        }
      }

      impl_sync!{ $struct, $ptr, $extern_sync, $sync, $in, $out }
      impl_async!{
          $struct, {
          $extern_async: $async,
          self: {
            outer: (this, $struct),
            inner: (_this_raw, $ptr, this.clone())
          },
          in: $in,
          out: $out
        }
      }
    )*
  }
}

macro_rules! extern_async_and_sync_methods {
  ( $struct:tt, $ptr:ty,
    $({ 
        $extern_async:ident: $async:ident,
        $extern_sync:ident: $sync:ident,
        in: $in:tt,
        out: $out:tt
      }),*
  ) => {
    $(
      extern_async!{
        $extern_async: $async,
        self: {
          outer: (this, $struct),
          inner: (this_raw, $ptr, $struct{raw: this_raw, ..this})
        },
        in: $in,
        out: $out
      }
      extern_sync!{ $ptr, $extern_sync, $in, $out }
    )*
  }
}

macro_rules! async_methods {
  ( $struct:ty, $($async_mapping:tt),* ) => {
    $(
      impl_async!{ $struct, $async_mapping }
      extern_async! $async_mapping
    )*
  }
}

macro_rules! impl_sync {
  ($struct:ty, $ptr:ty, $extern_sync:ident, $sync:ident,
   [$($in:tt),*],
   [$($out:tt),*]
  ) => {
    impl $struct {
      pub fn $sync(&self, $(out_name!($in): out_outer_type!($in)),*) -> Result<($(out_outer_type!($out)),*)> {
        $(out_uninitialized!($out);)*
        match unsafe{ $extern_sync(self.raw, $(to_raw!($in),)* $(out_name_as_mut_ref!($out)),* ) } {
          ExitCode::Success => Ok(($(out_value!($out)),*)),
          result => bail!(ErrorKind::ErrorExitCode(result))
        }
      }
    }
  }
}

macro_rules! impl_async {
  ( $struct:ty, {
      $extern_async:ident: $async:ident,
      $self:ident: {
        outer: ($self_ident:ident, $self_ty:ty),
        inner: ($self_inner:ident, $self_inner_ty:ty, $self_inner_expr:expr)
      },
      in: [$($in:tt),*],
      out: [$($out:tt),*]
    }
  ) => {
    impl $struct {
      pub fn $async<H>(&$self, $(out_name!($in): out_outer_type!($in),)* handler: H)
        where H: FnOnce($self_ty, ExitCode, $(out_outer_type!($out)),*)
      {
        extern fn raw_handler<H>(
          $self_inner: $self_inner_ty,
          raw_context: *mut c_void,
          error: ExitCode,
          $(out_name!($out): out_inner_type!($out)),*
          ) where H: FnOnce($self_ty, ExitCode, $(out_outer_type!($out)),*)
        {
          unsafe {
            let (handler, $self_ident) = ::std::ptr::read(raw_context as *mut (H, $self_ty));
            handler($self_inner_expr, error, $(out_value!($out)),*);
          };
        }
        let raw_context = Box::into_raw(Box::new((handler, $self.clone()))) as *mut c_void;
        unsafe{
          $extern_async($self.raw, raw_context, $(to_raw!($in),)* Some(raw_handler::<H>) );
        }
      }
    }
  }
}

/* Builds an external FFI definition for an async function called $async.
   Asyncronous functions always receive a pointer to a given opaque struct,
   the context, and maybe some more values. Then they call a handler
   which also receives a pointer to an opaque struct, the context, and
   the interesting output values.
*/
macro_rules! extern_async {
  ( $extern_async:ident: $async:ident,
    $self:ident: {
      outer: ($self_ident:ident, $self_ty:ty),
      inner: ($self_inner:ident, $self_inner_ty:ty, $self_inner_expr:expr)
    },
    in: [$($in:tt),*],
    out: [$($out:tt),*]
  ) => {
    extern {
      pub fn $extern_async(
        $self_inner: $self_inner_ty,
        context: *mut c_void,
        $(out_name!($in): out_inner_type!($in),)*
        handler: Option<unsafe extern fn(
          $self_inner: $self_inner_ty,
          *mut c_void,
          ExitCode,
          $(out_inner_type!($out),)*
          )>);
    }
  }
}

/* Builds an external FFI definition for an sync function called $sync.
   These functions always receive a pointer to a given opaque struct,
   some extra params, and pointers to store return values.
*/
macro_rules! extern_sync {
  ($main_type:ty, $sync:ident,
   [$($in:tt),*],
   [$($out:tt),*]
  ) => {
    extern {
      pub fn $sync(
        this: $main_type,
        $(out_name!($in): out_inner_type!($in),)*
        $(out_name!($out): *mut out_inner_type!($out)),*
      ) -> ExitCode;
    }
  }
}

macro_rules! out_name {
  (($name:ident, $inner:ty)) => { $name };
  (($name:ident, $inner:ty, $outer:expr)) => { $name }
}
macro_rules! out_inner_type {
  (($name:ident, $inner:ty)) => { $inner };
  (($name:ident, $inner:ty, $outer:expr)) => { $inner }
}
macro_rules! out_outer_type {
  (($name:ident, $inner:ty)) => { $inner };
  (($name:ident, $inner:ty, $outer:ty)) => { $outer }
}
macro_rules! out_value {
  (($out:ident, $inner:ty)) => { $out };
  (($out:ident, $inner:ty, $outer:tt)) => { $outer::new($out) }
}
macro_rules! to_raw {
  (($out:ident, $inner:ty)) => { $out };
  (($out:ident, $inner:ty, $outer:tt)) => { $out.raw }
}

macro_rules! out_uninitialized {
  (($name:ident, $inner:ty)) => {
    let mut $name = unsafe{ mem::uninitialized() };
  };
  (($name:ident, $inner:ty, $outer:ty)) => {
    let mut $name = unsafe{ mem::uninitialized() };
  }
}

macro_rules! out_name_as_mut_ref {
  (($name:ident, $inner:ty)) => { &mut $name };
  (($name:ident, $inner:ty, $outer:ty)) => { &mut $name };
}

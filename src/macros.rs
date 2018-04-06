macro_rules! opaque_resource_mapper {
  ( $enum:ident, $ptr:ident, $struct:ident {
      $($name:ident: $type:ty),*
    }
    async_and_sync { $($async_and_sync_mapping:tt),* }
    impl { $($impl:tt)* }
    extern $extern:tt
  ) => {
    pub enum $enum {}
    pub type $ptr = *mut $enum;
    #[derive(Clone)]
    pub struct $struct{
      raw: $ptr,
      $($name: $type),*
    }

    $(extern_async_and_sync!{$ptr $async_and_sync_mapping})*

    impl $struct {
      pub fn new( raw: $ptr, $($name: $type),*) -> $struct {
        $struct{raw, $($name),*}
      }

      $(impl_async_and_sync!{$struct, $ptr $async_and_sync_mapping})*

      $($impl)*
    }

    extern $extern
  }
}

macro_rules! impl_async_and_sync {
  ( $struct:ident, $ptr:ident {
      $extern_async:ident: $async:ident,
      $extern_sync:ident: $sync:ident,
      in: $in:tt,
      out: $out:tt
    }
  ) => {
    impl_sync!{ $ptr, $extern_sync, $sync, $in, $out }
    impl_async!{ $struct, $ptr, $extern_async, $async, $in, $out }
  }
}

macro_rules! impl_sync {
  ($ptr:ty, $extern_sync:ident, $sync:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$($out:tt),*]
  ) => {
    pub fn $sync(&self, $($in: $in_type),*) -> Result<($(out_outer_type!($out)),*)> {
      $(out_uninitialized!($out);)*
      match unsafe{ $extern_sync(self.raw, $($in,)* $(out_name_as_mut_ref!($out)),* ) } {
        ExitCode::Success => Ok(($(out_value!($out)),*)),
        result => bail!(ErrorKind::ErrorExitCode(result))
      }
    }
  }
}

macro_rules! impl_async {
  ($struct:tt, $ptr:ty, $extern_async:ident, $async:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$($out:tt),*]
  ) => {
    pub fn $async<H>(&self, $($in: $in_type,)* handler: H)
      where H: FnOnce($struct, ExitCode, $(out_outer_type!($out)),*)
    {
      extern fn raw_handler<H>(
        raw: $ptr,
        raw_context: *mut c_void,
        error: ExitCode,
        $(out_name!($out): out_inner_type!($out)),*
        ) where H: FnOnce($struct, ExitCode, $(out_outer_type!($out)),*)
      {
        unsafe {
          let mut context = Box::from_raw(raw_context as *mut Option<(H, $struct)>);
          let (handler, this) = context.take().unwrap();
          handler($struct{raw, ..this}, error, $(out_value!($out)),*);
        };
      }

      let raw_context = Box::into_raw(Box::new(Some((handler, self.clone())))) as *mut c_void;
      unsafe{ $extern_async(self.raw, raw_context, $($in,)* Some(raw_handler::<H>) )}
    }
  }
}

/* Most functions in the bitprim API have async and sync variants.
   Both variants receive exactly the same arguments and produce the same values.
   Async versions get their produced values as arguments to a handler.
   Sync versions receive pointers to store the returned values.
*/
macro_rules! extern_async_and_sync {
  ( $main_type:ident {
      $extern_async:ident: $async:ident,
      $extern_sync:ident: $sync:ident,
      in: $in:tt,
      out: $out:tt
    }
  ) => {
    extern_async!{ $main_type, $extern_async, $in, $out }
    extern_sync!{ $main_type, $extern_sync, $in, $out }
  }
}

/* Builds an external FFI definition for an async function called $async.
   Asyncronous functions always receive a pointer to a given opaque struct,
   the context, and maybe some more values. Then they call a handler
   which also receives a pointer to an opaque struct, the context, and
   the interesting output values.
*/
macro_rules! extern_async {
  ($main_type:ty, $async:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$($out:tt),*]
  ) => {
    extern {
      pub fn $async(
        this: $main_type,
        context: *mut c_void,
        $($in: $in_type,)*
        handler: Option<unsafe extern fn(
          $main_type,
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
   [$(($in:ident, $in_type:ty)),*],
   [$($out:tt),*]
  ) => {
    extern {
      pub fn $sync(
        this: $main_type,
        $($in: $in_type,)*
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


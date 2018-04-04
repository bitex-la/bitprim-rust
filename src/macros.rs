/*

*/
macro_rules! opaque_resource_mapper {
  ( $enum:ident, $ptr:ident, $struct:ident {
      $($name:ident: $type:ty),*
    }
    async_and_sync_mappings { $($async_and_sync_mapping:tt),* }
    impl { $($impl:tt)* }
  ) => {
    pub enum $enum {}
    pub type $ptr = *mut $enum;
    #[derive(Clone)]
    pub struct $struct{
      raw: $ptr,
      $($name: $type),*
    }

    $(extern_async_and_sync_from_mapping!{$ptr $async_and_sync_mapping})*

    impl $struct {
      pub fn new( raw: $ptr, $($name: $type),*) -> $struct {
        $struct{raw, $($name),*}
      }

      $(impl_async_and_sync_from_mapping!{$ptr $async_and_sync_mapping})*

      $($impl)*
    }
  }
}

macro_rules! impl_async_and_sync_from_mapping {
  ( $main_type:ident {
      $extern_async:ident: $async:ident,
      $extern_sync:ident: $sync:ident,
      in: $in:tt,
      out: $out:tt
    }
  ) => {
    impl_sync_from_mapping!{ $main_type, $extern_sync, $sync, $in, $out }
  }
}
macro_rules! impl_sync_from_mapping {
  ($main_type:ty, $extern_sync:ident, $sync:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$(($out:ident, $out_inner:ty, $out_outer:ty, $out_conv:path)),*]
  ) => {
    pub fn $sync(&self, $($in: $in_type),*) -> Result<($($out_outer),*)> {
      $(let mut $out = unsafe{ mem::uninitialized() };)*
      match unsafe{ $extern_sync(self.raw, $($in,)* $(&mut $out),*) } {
        ExitCode::Success => Ok(($($out_conv($out)),*)),
        result => bail!(ErrorKind::ErrorExitCode(result))
      }
    }
  }
}

/* Most functions in the bitprim API have async and sync variants.
   Both variants receive exactly the same arguments and produce the same values.
   Async versions get their produced values as arguments to a handler.
   Sync versions receive pointers to store the returned values.
*/
macro_rules! extern_async_and_sync {
  ($main_type:ty, $async:ident, $sync:ident, $in:tt, $out:tt) => {
    extern_async!{ $main_type, $async, $in, $out }
    extern_sync!{ $main_type, $sync, $in, $out }
  }
}

macro_rules! extern_async_and_sync_from_mapping {
  ( $main_type:ident {
      $extern_async:ident: $async:ident,
      $extern_sync:ident: $sync:ident,
      in: $in:tt,
      out: $out:tt
    }
  ) => {
    extern_async_from_mapping!{ $main_type, $extern_async, $in, $out }
    extern_sync_from_mapping!{ $main_type, $extern_sync, $in, $out }
  }
}

macro_rules! extern_async_from_mapping {
  ($main_type:ty, $async:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$(($out:ident, $out_inner:ty, $out_outer:ty, $out_conv:path)),*]
  ) => {
    extern {
      pub fn $async(
        this: $main_type,
        context: *mut c_void,
        $($in: $in_type,)*
        handler: Option<unsafe extern fn(
          this: $main_type,
          context: *mut c_void,
          exit_code: ExitCode,
          $($out: $out_inner),*
          )>);
    }
  }
}

macro_rules! extern_sync_from_mapping {
  ($main_type:ty, $sync:ident,
   [$(($in:ident, $in_type:ty)),*],
   [$(($out:ident, $out_inner:ty, $out_outer:ty, $out_conv:path)),*]
  ) => {
    extern {
      pub fn $sync(
        this: $main_type,
        $($in: $in_type,)*
        $($out: *mut $out_inner),*
      ) -> ExitCode;
    }
  }
}

/* DRY when defining multiple externals that have both sync and async versions */
macro_rules! extern_asyncs_and_syncs {
  ($main_type:ty, $(($async:ident, $sync:ident, $in:tt, $out: tt)),*) => {
    $(extern_async_and_sync!{$main_type, $async, $sync, $in, $out})*
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
   {$($in:ident: $in_type:ty),*},
   {$($out:ident: $out_type:ty),*}
  ) => {
    extern {
      pub fn $async(
        this: $main_type,
        context: *mut c_void,
        $($in: $in_type,)*
        handler: Option<unsafe extern fn(
          this: $main_type,
          context: *mut c_void,
          exit_code: ExitCode,
          $($out: $out_type,)*
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
   {$($in:ident: $in_type:ty),*},
   {$($out:ident: $out_type:ty),*}
  ) => {
    extern {
      pub fn $sync(
        this: $main_type,
        $($in: $in_type,)*
        $($out: *mut $out_type,)*
      ) -> ExitCode;
    }
  }
}


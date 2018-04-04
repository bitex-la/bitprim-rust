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


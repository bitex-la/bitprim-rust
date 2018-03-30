use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::io::AsRawFd;
use std::ffi::{CString, CStr};
use exit_code::ExitCode;
use chain::{Chain, ChainP};

pub enum ExecutorT {}
pub type ExecutorP = *mut ExecutorT;
pub struct Executor(ExecutorP);

extern "C" {
    pub fn executor_construct_fd(
        path: *const c_char,
        sout_fd: c_int,
        serr_fd: c_int,
    ) -> ExecutorP;
    pub fn executor_destruct(exec: ExecutorP);
    pub fn executor_run(exec: ExecutorP, ctx: *mut c_void, handler: RunHandler);
    pub fn executor_run_wait(exec: ExecutorP) -> ExitCode;
    pub fn executor_initchain(exec: ExecutorP) -> ExitCode;
    pub fn executor_stop(exec: ExecutorP) -> ExitCode;
    pub fn executor_stopped(exec: ExecutorP) -> ExitCode;
    pub fn executor_get_chain(exec: ExecutorP) -> ChainP;
    //pub fn executor_get_p2p(exec: ExecutorP) -> p2p_t;
    pub fn executor_version() -> *const c_char;
}

impl Executor {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Executor
    where O: AsRawFd, E: AsRawFd
  {
    let path = CString::new(config_path).expect("Invalid config path");
    let exec = unsafe{
      executor_construct_fd(path.as_ptr(), out.as_raw_fd(), err.as_raw_fd())
    };
    unsafe {
      executor_initchain(exec);
      executor_run_wait(exec);
    };
    Executor(exec)
  }

	pub fn get_chain(&self) -> Chain {
		println!("Getting chain");
		let chain_p = unsafe { executor_get_chain(self.0) };
		println!("got chain");
		Chain::new(chain_p)
	}

  pub fn version() -> String {
    unsafe {
      let s = executor_version();
      if s.is_null() { panic!("executor_version was null"); }
      CStr::from_ptr(s)
        .to_str()
        .expect("Version was not utf-8")
        .to_string()
    }
  }

  pub fn destruct(&self) {
    unsafe { executor_destruct(self.0) }
  }
}

pub type RunHandler = Option<
    unsafe extern "C" fn(exec: ExecutorP, ctx: *mut c_void, error: ExitCode)
>;
/*
FILE structs are not supported yet, so executor_construct is not implemented

extern "C" {
    pub fn executor_construct(
        path: *const c_char,
        sout: *mut FILE,
        serr: *mut FILE,
    ) -> *mut Executor;
}

Rust Bindgen suggests a FILE is a IO_FILE struct, as follows.

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut c_void,
    pub __pad2: *mut c_void,
    pub __pad3: *mut c_void,
    pub __pad4: *mut c_void,
    pub __pad5: usize,
    pub _mode: c_int,
    pub _unused2: [c_char; 20usize],
}

*/

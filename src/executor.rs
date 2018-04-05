use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::io::AsRawFd;
use std::ffi::{CString, CStr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, time};
use exit_code::ExitCode;
use chain::{Chain, ChainP};
use p2p::{P2p, P2pP};
use errors::*;

pub enum ExecutorT {}
pub type ExecutorP = *mut ExecutorT;

pub struct Executor{
	raw: ExecutorP,
	// Libbitcoin requires the main executor to be destroyed from the thread
	// that originally created it.
	// Also, we may have more than one executor, in separate threads, when
	// running async.
	// To address both issues we only destroy the original executor, and
	// keep count of any clones to wait until they're dropped before destruction.
	clones: Arc<AtomicUsize>,
	original: bool
}

impl Clone for Executor {
  fn clone(&self) -> Executor {
    self.clones.fetch_add(1, Ordering::SeqCst);
    Executor{ raw: self.raw, clones: self.clones.clone(), original: false }
  }
}

impl Drop for Executor {
    fn drop(&mut self) {
				if self.original {
					while self.clones.load(Ordering::SeqCst) > 0 {
					  thread::sleep(time::Duration::from_millis(100));
					}
					unsafe{ executor_destruct(self.raw) }
				}else{
					self.clones.fetch_sub(1, Ordering::SeqCst);
				}
    }
}

extern_async_and_sync!{ ExecutorP {
  executor_run: run,
  executor_run_wait: run_wait,
  in: [],
  out: []
}}

extern {
    pub fn executor_construct_fd(
        path: *const c_char,
        out_fd: c_int,
        err_fd: c_int,
    ) -> ExecutorP;
    pub fn executor_destruct(exec: ExecutorP);
    pub fn executor_initchain(exec: ExecutorP) -> ExitCode;
    pub fn executor_stop(exec: ExecutorP) -> ExitCode;
    pub fn executor_stopped(exec: ExecutorP) -> ExitCode;
    pub fn executor_get_chain(exec: ExecutorP) -> ChainP;
    pub fn executor_get_p2p(exec: ExecutorP) -> P2pP;
    pub fn executor_version() -> *const c_char;
}

impl Executor {
  pub fn new<O,E>(config_path: &str, out: &O, err: &E) -> Executor
    where O: AsRawFd, E: AsRawFd
  {
    let path = CString::new(config_path).expect("Invalid config path");
		let raw = unsafe{
      executor_construct_fd(path.as_ptr(), out.as_raw_fd(), err.as_raw_fd())
		};
    Executor{ raw, original: true, clones: Arc::new(AtomicUsize::new(0)) }
  }

  pub fn initchain(&self) -> Result<ExitCode> {
    let result = unsafe{ executor_initchain(self.raw) };
		match result {
			ExitCode::Success | ExitCode::ServiceStopped | ExitCode::OperationFailed => Ok(result),
			_ => bail!(ErrorKind::ErrorExitCode(result))
		}
  }

	pub fn run<H>(&self, handler: H) where H: FnOnce(Executor, ExitCode) {
		let raw_context = Box::into_raw(Box::new(Some((handler, self.clone())))) as *mut c_void;
    unsafe{
			executor_run(self.raw, raw_context, Some(Self::run_handler::<H>));
		};
	}

	extern fn run_handler<H>(_raw: ExecutorP, raw_context: *mut c_void, error: ExitCode)
		where H: FnOnce(Executor, ExitCode) {
		unsafe {
			let mut context = Box::from_raw(raw_context as *mut Option<(H, Executor)>);
      let (handler, this) = context.take().unwrap();
			handler(this, error)
		};
	}

	pub fn run_wait(&self) -> Result<ExitCode> {
		let result = unsafe{ executor_run_wait(self.raw) };
		match result {
			ExitCode::Success => Ok(result),
			_ => bail!(ErrorKind::ErrorExitCode(result))
		}
	}

	pub fn stop(&self) -> Result<ExitCode> {
		let result = unsafe{ executor_stop(self.raw) };
    match result {
			ExitCode::Success | ExitCode::ServiceStopped => Ok(result),
			_ => bail!(ErrorKind::ErrorExitCode(result))
		}
	}

	pub fn is_stopped(&self) -> bool {
    (unsafe{ executor_stopped(self.raw) }) == ExitCode::ServiceStopped
	}

	pub fn get_chain(&self) -> Chain {
		let raw = unsafe { executor_get_chain(self.raw) };
		Chain::new(raw, self.clone())
	}

	pub fn get_p2p(&self) -> P2p {
		P2p::new( unsafe { executor_get_p2p(self.raw) } )
	}

  pub fn version() -> String {
    unsafe {
      let s = executor_version();
      if s.is_null() { panic!("executor_version was null"); }
      CStr::from_ptr(s).to_string_lossy().into_owned()
    }
  }
}
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

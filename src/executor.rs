use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::io::AsRawFd;
use std::ffi::{CString, CStr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};
use exit_code::ExitCode;
use chain::{Chain, ChainP};
use p2p::{P2p, P2pP};
use errors::*;
use explorer::Explorer;
use std::thread::{current, ThreadId};

pub enum ExecutorT {}
unsafe impl Send for ExecutorT {}
pub type ExecutorP = *mut ExecutorT;

pub struct Executor{
	pub raw: ExecutorP,
	// Libbitcoin requires the main executor to be destroyed from the thread
	// that originally created it.
	// Also, we may have more than one executor, in separate threads, when
	// running async.
	// To address both issues we only destroy the original executor, and
	// keep count of any clones to wait until they're dropped before destruction.
	in_original_thread: Arc<AtomicUsize>,
	in_new_thread: Arc<AtomicUsize>,
	original_thread_id: ThreadId,
}

unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}

impl Clone for Executor {
  fn clone(&self) -> Executor {
    if self.original_thread_id == current().id() {
      self.in_original_thread.fetch_add(1, Ordering::SeqCst);
    }else {
      self.in_new_thread.fetch_add(1, Ordering::SeqCst);
    }

    Executor{
      raw: self.raw,
      original_thread_id: self.original_thread_id,
      in_original_thread: self.in_original_thread.clone(),
      in_new_thread: self.in_new_thread.clone()
    }
  }
}

impl Drop for Executor {
    fn drop(&mut self) {
      if self.original_thread_id == current().id() {
        if self.in_original_thread.load(Ordering::SeqCst) == 1 {
          while self.in_new_thread.load(Ordering::SeqCst) > 0 {
            thread::sleep(time::Duration::from_millis(100));
          }
          unsafe{ executor_destruct(self.raw) }
        } else {
          self.in_original_thread.fetch_sub(1, Ordering::SeqCst);
        }
      }else{
        self.in_new_thread.fetch_sub(1, Ordering::SeqCst);
      }
    }
}

extern_async_and_sync_methods!{ Executor, ExecutorP, {
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
    Executor{
      raw,
      original_thread_id: current().id(),
      in_original_thread: Arc::new(AtomicUsize::new(1)),
      in_new_thread: Arc::new(AtomicUsize::new(0)),
    }
  }

  pub fn version() -> String {
    unsafe {
      let s = executor_version();
      if s.is_null() { panic!("executor_version was null"); }
      CStr::from_ptr(s).to_string_lossy().into_owned()
    }
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
			ExitCode::Success | ExitCode::ServiceStopped => Ok(result),
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

  pub fn explorer(&self) -> Explorer {
    Explorer{ executor: self.clone() }
  }
}

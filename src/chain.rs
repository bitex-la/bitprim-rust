use std::mem;
use std::os::raw::{c_uint};
use exit_code::ExitCode;
use errors::*;

pub enum ChainT {}
pub type ChainP = *mut ChainT;
pub struct Chain(ChainP);

extern "C" {
  pub fn chain_get_last_height(chain: ChainP, out_heigth: *mut c_uint) -> ExitCode;
}

impl Chain {
	pub fn new(chain_p: ChainP) -> Chain { Chain(chain_p) }

  pub fn get_last_height(&self) -> Result<u32> {
	  let mut height = unsafe{ mem::uninitialized() };
    match unsafe{ chain_get_last_height(self.0, &mut height) } {
			ExitCode::Success => Ok(height),
			result => bail!(ErrorKind::ErrorExitCode(result))
		}
	}
}

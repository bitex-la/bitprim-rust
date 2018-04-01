#[macro_use] extern crate error_chain;
pub mod exit_code;
pub mod executor;
pub mod chain;
pub mod errors;

#[cfg(test)]
mod tests {
    use executor::Executor;
    use std::fs::File;
		use errors::*;
		use error_chain::ChainedError;
		use exit_code::ExitCode;
	  use std::{thread, time};
		use std::sync::atomic::{AtomicBool, Ordering};
		use std::sync::Arc;

		#[test]
		fn it_has_a_version(){
				assert_eq!(&Executor::version(), "\"v0.7.0\"");
		}

		macro_rules! assert_result {
			($name:ident $body:block) => (
    		#[test]
				fn $name(){
					let result : Result<()>= (||{ $body; Ok(()) })();
					match result {
						Err(e) => assert!(false, e.display_chain().to_string()),
						_ => assert!(true)
					}
				}
			)
		}

		assert_result!{ runs_sync {
				let f = File::create("/dev/null").unwrap();
				let executor = Executor::new("./test/btc-testnet.cfg", &f, &f);
				let init_result = executor.initchain();
				println!("Init result {:?}", init_result);
				executor.run_wait()?;
				loop {
					let last_height = executor.get_chain().get_last_height()?;
					if last_height > 1000 { break }
				}
				println!("Stopping");
				executor.stop()?;
				while !executor.is_stopped() {}
				println!("Destroyed");
		}}

		assert_result!{ runs_async {
				let f = File::create("/dev/null").unwrap();
				let executor = Executor::new("./test/btc-testnet.cfg", &f, &f);
				executor.initchain();
				let guard = Arc::new(AtomicBool::new(true));
				let other_guard = guard.clone();
				executor.run(|exec: Executor, exit_code: ExitCode| {
					if exit_code != ExitCode::Success {
						println!("Could not run executor: {:?}", exit_code);
					}else{
						let chain = exec.get_chain();
					  println!("Last height on handler {}", chain.get_last_height().expect("get_height"));
						while chain.get_last_height().expect("Get height") < 5000 {
								println!("Last height on handler {}", chain.get_last_height().expect("get_height"));
								thread::sleep(time::Duration::from_millis(1000));
						}
					}
					other_guard.store(false, Ordering::Relaxed);
				  println!("Exited thread");
				});
				
				println!("Height on main thread");
				let chain = executor.get_chain();
				println!("Last height on main thread {}", chain.get_last_height().expect("get_height"));
		}}
}

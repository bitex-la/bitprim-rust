#[macro_use] extern crate error_chain;
extern crate bitprim;

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use bitprim::{Executor, ExitCode};
use bitprim::errors::*;

macro_rules! assert_ok {
  ($name:ident $body:block) => (
    #[test]
    fn $name(){
      let result : Result<()>= (||{ $body; Ok(()) })();
      match result {
        Err(e) => assert!(false, format!("{}", e)),
        _ => assert!(true)
      }
    }
  )
}

fn build_test_executor() -> Result<Executor> {
  let f = File::create("/dev/null").unwrap();
  let exec = Executor::new("./tests/btc-testnet.cfg", &f, &f);
  exec.initchain()?;
  Ok(exec)
}

#[test]
fn it_has_a_version(){
    assert_eq!(&Executor::version(), "\"v0.7.0\"");
}

assert_ok!{ runs_1000_blocks_sync {
    let exec = build_test_executor()?;
    exec.run_wait()?;
    while exec.get_chain().get_last_height()? < 1000 {
      sleep(Duration::new(1,0));
    }
    exec.stop()?;
    while !exec.is_stopped() {
      sleep(Duration::new(1,0));
    }
}}

assert_ok!{ runs_1000_blocks_async {
    let exec = build_test_executor()?;
    exec.run(|exec, exit_code| {
      if exit_code != ExitCode::Success {
        assert!(false, format!("Async runner failed with {:?}", exit_code));
      }else{
        while exec.get_chain().get_last_height().expect("height fail") < 1000 {
          sleep(Duration::new(1,0));
        }
      }
    });
}}

/*
assert_ok!{ gets_last_height_async {
    let exec = build_test_executor()?;
    exec.get_chain().fetch_last_height
}}

assert_ok!{ polls_to_find_earliest_transaction {
}};

assert_ok!{ finds_earliest_transaction_by_subscribing {
}};
*/


extern crate error_chain;
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

fn build_400_blocks_executor() -> Result<Executor> {
  let exec = build_test_executor()?;
  exec.run_wait()?;
  while exec.get_chain().get_last_height()? < 400 {
    println!("Syncing {:?}", exec.get_chain().get_last_height()?);
    sleep(Duration::new(1,0));
  }
  Ok(exec)
}

#[test]
fn it_has_a_version(){
    assert_eq!(&Executor::version(), "\"v0.7.0\"");
}

assert_ok!{ runs_400_blocks_sync {
    let exec = build_400_blocks_executor()?;
    exec.stop()?;
    while !exec.is_stopped() {
      sleep(Duration::new(1,0));
    }
}}

assert_ok!{ runs_400_blocks_async {
    let exec = build_test_executor()?;
    exec.run(|exec, exit_code| {
      if exit_code != ExitCode::Success {
        assert!(false, format!("Async runner failed with {:?}", exit_code));
      }else{
        while exec.get_chain().get_last_height().expect("height fail") < 400 {
          sleep(Duration::new(1,0));
        }
      }
    });
}}

assert_ok!{ gets_last_height_async {
    let exec = build_400_blocks_executor()?;
    exec.run(|exec, _|{
      exec.get_chain().fetch_last_height(|_chain, exit, height|{
        println!("Async fetch last height: {}, {:?}", height, exit);
        assert!(height >= 400, "Height was not over 1000");
      })
    })
}}

assert_ok!{ gets_earliest_transaction_block {
  let exec = build_400_blocks_executor()?;
  let chain = exec.get_chain();
  let (block, _) = chain.get_block_by_height(381)?;
  let height = chain.get_block_height(block.hash())?;
  assert!(height == 381);
  assert!(block.hash().to_hex() ==
    "000000001a4c2c64beded987790ab0c00675b4bc467cd3574ad455b1397c967c");
  assert!(block.transaction_count() == 2);
}}

assert_ok!{ fetches_earliest_transaction_block {
  let exec = build_400_blocks_executor()?;
  let chain = exec.get_chain();
  chain.fetch_block_by_height(381, |new_chain, _, block, _height|{
    assert!(block.hash().to_hex() ==
      "000000001a4c2c64beded987790ab0c00675b4bc467cd3574ad455b1397c967c");
    assert!(block.transaction_count() == 2);
    new_chain.fetch_block_height(block.hash(), |_, _, height:u64|{
      assert!(height == 381);
    });
  })
}}

/*
assert_ok!{ finds_earliest_transaction_by_subscribing {
}};
*/


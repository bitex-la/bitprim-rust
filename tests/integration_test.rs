extern crate bitprim;
extern crate error_chain;
#[macro_use]
extern crate pretty_assertions;

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use bitprim::{Executor, ExitCode};
use bitprim::errors::*;
use bitprim::payment_address::PaymentAddress;
use bitprim::explorer::*;
use std::str::FromStr;

const CURRENCY: &str = "bch";

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
    let exec = Executor::new(&format!("./tests/{}-testnet.cfg", CURRENCY), &f, &f);
    exec.initchain()?;
    Ok(exec)
}

fn build_500_blocks_executor() -> Result<Executor> {
    let exec = build_test_executor().expect("Build executor");
    let _ = exec.run_wait();
    while exec.get_chain().get_last_height()? < 500 {
        println!("Syncing {:?}", exec.get_chain().get_last_height()?);
        sleep(Duration::new(1, 0));
    }
    Ok(exec)
}

#[test]
fn it_has_a_version() {
    assert_eq!(&Executor::version(), "\"v0.9.0\"");
}

assert_ok!{ runs_500_blocks_sync {
    let exec = build_500_blocks_executor()?;
    exec.stop()?;
    while !exec.is_stopped() {
      sleep(Duration::new(1,0));
    }
}}

assert_ok!{ runs_500_blocks_async {
    let exec = build_test_executor()?;
    exec.run(|exec, exit_code| {
      if exit_code != ExitCode::Success {
        assert!(false, format!("Async runner failed with {:?}", exit_code));
      }else{
        while exec.get_chain().get_last_height().expect("height fail") < 500 {
          sleep(Duration::new(1,0));
        }
      }
    });
}}

assert_ok!{ gets_last_height_async {
    let exec = build_500_blocks_executor()?;
    exec.run(|exec, _|{
      exec.get_chain().fetch_last_height(|_chain, exit, height|{
        assert!(height >= 500, "Height was not over 1000");
      })
    })
}}

assert_ok!{ gets_earliest_transaction_block {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  let (block, _) = chain.get_block_by_height(429)?;
  let height = chain.get_block_height(block.hash())?;
  assert!(height == 429);
  assert!(block.hash().to_hex() ==
    "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
  assert!(block.len() == 49);
}}

assert_ok!{ fetches_earliest_transaction_block {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  chain.fetch_block_by_height(429, |new_chain, _, block, _height|{
    assert!(block.hash().to_hex() ==
      "00000000e080223655db52d2c35a37f6aa17a3f2efefa6794fd9831374cff09f");
    assert!(block.len() == 49);
    new_chain.fetch_block_height(block.hash(), |_, _, height:u64|{
      assert!(height == 429);
    });
  })
}}

assert_ok!{ explores_an_address {
  let explorer = build_500_blocks_executor()?.explorer();
  let addr = PaymentAddress::from_str("mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE");
  let hist = explorer.address_history(addr.unwrap(), 100000, 1)?;

  assert_eq!(hist.len(), 25);

  assert_eq!(hist[18], AddressHistory::Received(Received{
    satoshis: 450648,
    transaction_hash:
      "58baf615ed9e95023acb05715d3885cc48700ab548072cb5a996056786931fe3".to_string(),
    position: 1,
    is_spent: false,
    block_height: 429
  }));

  assert_eq!(hist[17], AddressHistory::Received(Received{
    satoshis: 963007,
    transaction_hash:
      "8ff1a6d53806b2c6e0f9c82d8f1a32cee604e84ee400fc2c7f2a8d7b95ba328c".to_string(),
    position: 1,
    is_spent: true,
    block_height: 429
  }));
}}

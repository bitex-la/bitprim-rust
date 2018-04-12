extern crate error_chain;
extern crate bitprim;

use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use bitprim::{Executor, ExitCode};
use bitprim::errors::*;
use bitprim::transaction::Transaction;
use bitprim::payment_address::PaymentAddress;
use bitprim::history_semantic::HistorySemantic;
use std::sync::{Arc,Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

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

fn build_500_blocks_executor() -> Result<Executor> {
  let exec = build_test_executor()?;
  exec.run_wait()?;
  while exec.get_chain().get_last_height()? < 500 {
    println!("Syncing {:?}", exec.get_chain().get_last_height()?);
    sleep(Duration::new(1,0));
  }
  Ok(exec)
}

#[test]
fn it_has_a_version(){
    assert_eq!(&Executor::version(), "\"v0.7.0\"");
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
        println!("Async fetch last height: {}, {:?}", height, exit);
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

assert_ok!{ gets_unspents_for_an_address {
  let exec = build_500_blocks_executor()?;
  let chain = exec.get_chain();
  let addr = PaymentAddress::from_str("mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE");
  let hist = chain.get_history_semantic(addr, 100000, 1)?;
  //assert!(hist.len() == 19);
  let (block, _) = chain.get_block_by_height(441 as u64).unwrap();
  println!("Block 441 is {:?}. Count: {:?}", block.hash().to_hex(), block.len());
  for i in 0..block.len() {
    println!("About to get Position {:?}", i);
    let tx = block.nth(i as u32);
    println!("Position: {:?} hash is {:?}", i, tx.hash().to_hex());
    let inputs = tx.inputs();
    println!("Getting {:?} inputs", inputs.len());
    for j in 0..inputs.len() {
      println!("Input {:?}. Valid {:?}. Hex: {:?}", j, inputs.nth(j).is_valid(),
        inputs.nth(j).previous_output().hash().to_hex());
    }
  }

  println!("done");

  /*
  if let HistorySemantic::Received{
    satoshis, ref transaction_hash, position, is_spent } = hist[13]
  {
    println!("Satoshis: {:?}", satoshis); // 49.08808802 BTC
    //74c2146fe18fb7c652dc10a5b126d0754df44ad8c1d24ed399ef561001e05c43
    println!("Transaction Hash: {:?}", transaction_hash);
    println!("Position: {:?}", position); // 0
    println!("Spent: {:?}", is_spent); // True
  }else{
    assert!(false, "First item was not received")
  }
  */
}}

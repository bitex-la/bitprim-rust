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
use bitprim::hash::Hash;
use std::str::FromStr;

use bitprim::input_list::InputList;
use bitprim::output_list::OutputList;

#[cfg(feature="btc")]
const CURRENCY: &str = "btc";

#[cfg(feature="bch")]
const CURRENCY: &str = "bch";

#[cfg(feature="ltc")]
const CURRENCY: &str = "ltc";

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

  let hash1 = Hash::from_hex("58baf615ed9e95023acb05715d3885cc48700ab548072cb5a996056786931fe3").unwrap();
  let hash2 = Hash::from_hex("8ff1a6d53806b2c6e0f9c82d8f1a32cee604e84ee400fc2c7f2a8d7b95ba328c").unwrap();

  assert_eq!(hist[18], AddressHistory::Received(Received{
    satoshis: 450648,
    transaction_hash: hash1,
    position: 1,
    is_spent: false,
    block_height: 429,
    version: 1,
    locktime: 0,
    input_details: vec![ 
        InputDetail {
            prev_hash: "b0d95df6f5ff04e908f167efb8bf5b258ff62345b5ae555245fa4c6cdca72468".to_string(),
            prev_index: 0,
            sequence: 4294967295,
            script_sig: "47304402200c510cdabae063d4b378441fc3a7d61e4046a9c67e8b207a300fe934bf83bed30220385100118db78c5daaa4b0b740c76838e322b50bbf2ac9b47d2b4019614db1b8012103d13e48291037cf0027faaf6268364d2936ae37d0a644855af44a392f1f104569".to_string()
        }
    ],
    output_details: vec![
        OutputDetail {
            amount: 4907387250,
            script_pubkey: "76a914bc268dc123e6a2cfc1abf6172e551c11c23c5c8d88ac".to_string()
        },
        OutputDetail {
            amount: 450648,
            script_pubkey: "76a914185ec5c05012a9438a28aa20630097153907b8b988ac".to_string()
        }
    ]
  }));

  assert_eq!(hist[17], AddressHistory::Received(Received{
    satoshis: 963007,
    transaction_hash: hash2,
    position: 1,
    is_spent: true,
    block_height: 429,
    version: 1,
    locktime: 0,
    input_details: vec![ 
        InputDetail {
            prev_hash: "1ffc95acd2f54841a72523bfa24c2b2cc37cee2551a95071d40ce30c46062b53".to_string(),
            prev_index: 0,
            sequence: 4294967295,
            script_sig: "483045022100a828b48621524e59ae1379f7cd64baec562cdf0c8f056d99b13bf76f1fb9dacb022023294a454d4e2c74a42962ebd1cea9b2f3094d4a6848b0d0f95200c0465db6500121037aed90da521434cfb6c14b8a7854beb168b2aba03e38f08a8960e2fcdb3cb3eb".to_string()
        }
    ],
    output_details: vec![
        OutputDetail {
            amount: 4897843785,
            script_pubkey: "76a9148826518aa9ab2b3ac8de8203a30b0a2b6e7f818b88ac".to_string()
        },
        OutputDetail {
            amount: 963007,
            script_pubkey: "76a914185ec5c05012a9438a28aa20630097153907b8b988ac".to_string()
        }
    ]
  }));
}}

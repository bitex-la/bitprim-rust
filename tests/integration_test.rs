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
            prev_hash: Hash::from_hex("b0d95df6f5ff04e908f167efb8bf5b258ff62345b5ae555245fa4c6cdca72468").unwrap(),
            prev_index: 0,
            sequence: 4294967295,
            script_sig: "e90f307a208b7ec6a946401ed6a7c31f4478b3d463e0bada0c510c2002443047".to_string()
        }
    ],
    output_details: vec![
        OutputDetail {
            amount: 4907387250,
            script_pubkey: "00000000000000ac888d5c3cc2111c552e17f6abc1cfa2e623c18d26bc14a976".to_string()
        },
        OutputDetail {
            amount: 450648,
            script_pubkey: "00000000000000ac88b9b807391597006320aa288a43a91250c0c55e1814a976".to_string()
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
            prev_hash: Hash::from_hex("1ffc95acd2f54841a72523bfa24c2b2cc37cee2551a95071d40ce30c46062b53").unwrap(),
            prev_index: 0,
            sequence: 4294967295,
            script_sig: "3bb1996d058f0cdf2c56ecba64cdf77913ae594e522186b428a8002102453048".to_string()
        }
    ],
    output_details: vec![
        OutputDetail {
            amount: 4897843785,
            script_pubkey: "00000000000000ac888b817f6e2b0a0ba30382dec83a2baba98a51268814a976".to_string()
        },
        OutputDetail {
            amount: 963007,
            script_pubkey: "00000000000000ac88b9b807391597006320aa288a43a91250c0c55e1814a976".to_string()
        }
    ]
  }));
}}

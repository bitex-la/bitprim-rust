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
      exec.get_chain().fetch_last_height(|_chain, _exit, height|{
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

  if let AddressHistory::Received(ref received) = hist[20] {
      assert_eq!(received, &Received{
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
                script_sig: (&received.input_details[0].script_sig).to_string()
            }
        ],
        output_details: vec![
            OutputDetail {
                amount: 4907387250,
                address: (&received.output_details[0].address).to_string(),
                script_pubkey: (&received.output_details[0].script_pubkey).to_string()
            },
            OutputDetail {
                amount: 450648,
                address: (&received.output_details[1].address).to_string(),
                script_pubkey: (&received.output_details[1].script_pubkey).to_string()
            }
        ]
      });
  }

  if let AddressHistory::Received(ref received) = hist[19] {
      assert_eq!(received, &Received{
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
                script_sig: (&received.input_details[0].script_sig).to_string()
            }
        ],
        output_details: vec![
            OutputDetail {
                amount: 4897843785,
                address: (&received.output_details[0].address).to_string(),
                script_pubkey: (&received.output_details[0].script_pubkey).to_string()
            },
            OutputDetail {
                amount: 963007,
                address: (&received.output_details[1].address).to_string(),
                script_pubkey: (&received.output_details[1].script_pubkey).to_string()
            }
        ]
      });
  }

  let hex_raw_transaction = "010000000829e780298e0b5d81295caaa80567437bd1bcc9513670bd25f7608b05dc58fa81010000006a473044022061ccfd92a07253dd7d8d405f3ca22d8ea472e1da850d6d2cbe129dc4a32867a902206b31a1c0dd242a80db2ba2a9fcd155ee9d35d7ed465210c1e76d50d5429abcc3012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff464c3ffc0fc1554263d96312615c7ff591c20f5f2e4d330ed8be20b7193e8b55010000006a473044022005fcbf204c707c9b053d316a78f8ea2f27614eb9d8a449694b639368900da14c022015eebef633dc3cde0c50cabf9f92da27c18108f8287b3cac212ee30f3a765deb012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff5173a70a5383e2ca3a99466fb4d4d63ba5a5730e8d07fad968ea0b00795b35c9010000006b483045022100e2038858f4050b30462bc140c131089d1d56036d8d52b9fc255ff182b70a8e8a022014b85f6f2d7f383dbb734bb8327997e90540f26ad7581476acd5c374b5954e4f012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff52f84225541ba7cf8270b14df43df999662f6eba3279733ad25d7d12d7383f24010000006a473044022023036878e6737c90bc77b209cf1373413251c5d1ccd3399ec034cf62b0ecaf51022010e2f4c89334f5a5736e5af544ee81d921822527be3fcaddb9acec361b22166c012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff5544a2ff9c84124eaafee9351a2191c9d3f29732ffbcf4cb384d708bed1feb85010000006b483045022100ddfa15696503653d7043dcce76d70fcbe9c29cbdab7a1b6a8bcf8dbc22527a5a0220262b4de70beef81fa9e77cfb876c059333b32bcc776b276d58d00642948d6c18012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff81861c720d270cdf69810facd3616d81b4862e4f229a6cc48a7e838720acb5da010000006a47304402207fc52c9ec37f53b97fffaf477873f7b551a0cc09df7111bc232d4673f0a43a8402202f53152c7c68474bd6e4c529fa9f07d5dbcfac17dc4082cbeef1a6850ffb6189012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffffc928ea11a5c6cfc6d18d1b29202a7b932480f72e27633b1cefbebe2abaff7557010000006b483045022100ba444e66e60e7957460ae606b8789ce9bd42180a3243d1c0235c982a3d7959de022040218e10934ce6755ef03ef2b69a8bcb37e5408cf8c720f58e531dae6ad77e2f012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffffda470c40e08016293b582962c3731bede975f11db9e3027c450adb84b9cb57f2010000006a47304402200de6df14f3c2e113b3ab861bb0ee679b656be3a6eeaa29c739202f864e0454a402206f136174f28315a21ab2c4102ca2f13aa22634672fc01459a9b1df3068a7bce9012103fadcfdfe7f51a270b32f7b6b50b4f3a0110d25c9618671325032306718eb339effffffff0196c43e00000000001976a9140b3517e6562623042f7ae1fa9da19d3106841a8a88ac00000000";

  let chain = explorer.executor.get_chain();
  assert_eq!("0af3731e9405b81ee62fe672f9e4465a745880899762356b76ebb94ea236f262".to_string(), chain.broadcast(hex_raw_transaction).to_hex());
}}

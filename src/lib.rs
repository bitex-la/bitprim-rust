#[macro_use] extern crate error_chain;
pub mod exit_code;
pub mod executor;
pub mod chain;
pub mod errors;

#[cfg(test)]
mod tests {
    use executor::Executor;
    use std::fs::File;
    #[test]
    fn runs_executor() {
      let f = File::create("/dev/null").unwrap();
      let executor = Executor::new("/home/nubis/bitprim/test/btc-testnet.cfg", &f, &f);
      assert_eq!(&Executor::version(), "\"v0.7.0\"");
      let last_height = executor.get_chain().get_last_height();
      println!("Last height {:?}", last_height);
      executor.destruct();
    }
}

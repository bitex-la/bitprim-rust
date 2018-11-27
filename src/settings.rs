use executor::{Executor, ExecutorP};

opaque_resource!{
  #[derive(Debug, Clone)]
  CurrencyT, CurrencyP, Currency {}
}

opaque_resource!{
  #[derive(Debug, Clone)]
  SettingT, SettingP, Setting {}
}

impl Setting {
    pub fn currency() -> Currency {
        Currency::new(unsafe { node_settings_get_currency() })
    }

    pub fn network(executor: &Executor) -> i32 {
        unsafe { node_settings_get_network(executor.raw) }
    }
}

extern "C" {
    pub fn node_settings_get_currency() -> CurrencyP;
    pub fn node_settings_get_network(executor: ExecutorP) -> i32;
}


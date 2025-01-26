use std::time::Duration;

use env_logger::Env;
use nostr::message::RelayMessage;
use utils::{clear_nostr_log, Relay};

use crate::utils::dump_nostr_log;

mod utils;

#[test]
pub fn test_dm() {
    let env = Env::new().filter_or("TEST_LOG", "debug");
    let _ = env_logger::Builder::from_env(env).is_test(true).try_init();
    log::debug!("test dm");
    let mut relay = Relay::new();
    let mut client_a = relay.new_client();
    let mut client_b = relay.new_client();
    clear_nostr_log(&mut relay);
    client_a.subscribe_dm().unwrap();
    std::thread::sleep(Duration::from_secs(1));
    dump_nostr_log(&mut relay);
    log::info!("---------------------------");
    client_b._send_dm("test dm", &client_a.pubkey()).unwrap();
    std::thread::sleep(Duration::from_secs(3));
    dump_nostr_log(&mut relay);
    loop {
        match client_a._try_receive() {
            Ok(Some(event)) => {
                log::info!("receive event: {}", event.content());
                #[allow(deprecated)]
                if let "test dm" = event.content() {
                    return;
                }
            }
            Err(e) => log::error!("{:?}", e),
            Ok(None) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
}

use std::path::PathBuf;

use steamlocate::SteamDir;

const TERARRIA_APPID: &u32 = &105600;

// TODO rewrite this to not use steam-locator. It uses an outdated crate and
// could check some more reg keys on windows.
pub fn check_steam() -> PathBuf {
    let mut steam = SteamDir::locate().unwrap();
    match steam.app(TERARRIA_APPID) {
        None => panic!("Terraria is not installed"),
        Some(p) => p.path.to_owned(),
    }
}

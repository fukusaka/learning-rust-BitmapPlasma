use std::sync::Mutex;

use android_logger::{Config, FilterBuilder};
use lazy_static::lazy_static;
use log::Level;

lazy_static! {
    static ref initializedMutex: Mutex<bool> = Mutex::new(false);
}

// LogCat へログ出力する初期設定だが、一度だけ初期化したいが、定番の方法が分からないので、取り敢えずの実装を追加
pub fn use_android_logger() {
    let mut initialized = initializedMutex.lock().unwrap();
    if !*initialized {
        android_logger::init_once(Config::default().with_min_level(Level::Debug));
        *initialized = true
    }
}

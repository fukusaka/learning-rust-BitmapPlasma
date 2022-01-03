extern crate jni;
#[macro_use]
extern crate log;

use std::ptr::null_mut;
use std::sync::Mutex;

use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;
use lazy_static::lazy_static;

use crate::android_bitmap::{
    AndroidBitmapInfo, AndroidBitmap_getInfo, AndroidBitmap_lockPixels, AndroidBitmap_unlockPixels,
    ANDROID_BITMAP_FORMAT_RGB_565,
};

mod android_bitmap;
mod frame_stats;
mod logger;
mod plasma;

lazy_static! {
    static ref plasmaObj: plasma::Plasma = plasma::Plasma::new();
    static ref statsMutexObj: Mutex<frame_stats::Stats> = Mutex::new({ frame_stats::Stats::new() });
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern "C" fn Java_com_example_plasma_PlasmaView_renderPlasma(
    env: *mut JNIEnv,
    obj: JObject,
    bitmap: JObject,
    time_ms: jlong,
) {
    let mut info: AndroidBitmapInfo = AndroidBitmapInfo::default();
    let mut pixels: *mut ::std::os::raw::c_void = null_mut();

    logger::use_android_logger();

    let ret = AndroidBitmap_getInfo(env, bitmap, &mut info);
    if ret < 0 {
        error!("AndroidBitmap_getInfo() failed ! error={}", ret);
        return;
    }

    if info.format != ANDROID_BITMAP_FORMAT_RGB_565 {
        error!("Bitmap format is not RGB_565 !");
        return;
    }

    let ret = AndroidBitmap_lockPixels(env, bitmap, &mut pixels);
    if ret < 0 {
        error!("AndroidBitmap_lockPixels() failed ! error={}", ret);
    }

    let mut statsObj = statsMutexObj.lock().unwrap();

    statsObj.stats_start_frame();

    /* Now fill the values with a nice little plasma */
    plasmaObj.fill_plasma(&info, pixels, time_ms);

    AndroidBitmap_unlockPixels(env, bitmap);

    statsObj.stats_end_frame();
}

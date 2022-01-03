extern crate jni;
#[macro_use]
extern crate log;

use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;

mod android_bitmap;

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern "C" fn Java_com_example_plasma_PlasmaView_renderPlasma(
    env: *mut JNIEnv,
    obj: JObject,
    bitmap: JObject,
    time_ms: jlong,
) {
}

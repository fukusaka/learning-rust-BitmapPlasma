extern crate jni;

use jni::objects::JObject;
use jni::JNIEnv;

pub const ANDROID_BITMAP_FORMAT_NONE: AndroidBitmapFormat = 0;
pub const ANDROID_BITMAP_FORMAT_RGBA_8888: AndroidBitmapFormat = 1;
pub const ANDROID_BITMAP_FORMAT_RGB_565: AndroidBitmapFormat = 4;
pub const ANDROID_BITMAP_FORMAT_RGBA_4444: AndroidBitmapFormat = 7;
pub const ANDROID_BITMAP_FORMAT_A_8: AndroidBitmapFormat = 8;
pub const ANDROID_BITMAP_FORMAT_RGBA_F16: AndroidBitmapFormat = 9;
pub type AndroidBitmapFormat = i32;

#[repr(C)]
pub struct AndroidBitmapInfo {
    /** The bitmap width in pixels. */
    pub width: u32,
    /** The bitmap height in pixels. */
    pub height: u32,
    /** The number of byte per row. */
    pub stride: u32,
    /** The bitmap pixel format. See {@link AndroidBitmapFormat} */
    pub format: AndroidBitmapFormat,
    /** Bitfield containing information about the bitmap.
     *
     * <p>Two bits are used to encode alpha. Use {@link ANDROID_BITMAP_FLAGS_ALPHA_MASK}
     * and {@link ANDROID_BITMAP_FLAGS_ALPHA_SHIFT} to retrieve them.</p>
     *
     * <p>One bit is used to encode whether the Bitmap uses the HARDWARE Config. Use
     * {@link ANDROID_BITMAP_FLAGS_IS_HARDWARE} to know.</p>
     *
     * <p>These flags were introduced in API level 30.</p>
     */
    pub flags: u32,
}

impl Default for AndroidBitmapInfo {
    fn default() -> AndroidBitmapInfo {
        AndroidBitmapInfo {
            width: 0,
            height: 0,
            stride: 0,
            format: 0,
            flags: 0,
        }
    }
}

extern "C" {
    /**
     * Given a java bitmap object, fill out the {@link AndroidBitmapInfo} struct for it.
     * If the call fails, the info parameter will be ignored.
     */
    #[allow(non_snake_case)]
    pub fn AndroidBitmap_getInfo(
        env: *mut JNIEnv,
        jbitmap: JObject,
        info: *mut AndroidBitmapInfo,
    ) -> i32;

    /**
     * Given a java bitmap object, attempt to lock the pixel address.
     * Locking will ensure that the memory for the pixels will not move
     * until the unlockPixels call, and ensure that, if the pixels had been
     * previously purged, they will have been restored.
     *
     * If this call succeeds, it must be balanced by a call to
     * AndroidBitmap_unlockPixels, after which time the address of the pixels should
     * no longer be used.
     *
     * If this succeeds, *addrPtr will be set to the pixel address. If the call
     * fails, addrPtr will be ignored.
     */
    #[allow(non_snake_case)]
    pub fn AndroidBitmap_lockPixels(
        env: *mut JNIEnv,
        jbitmap: JObject,
        addrPtr: *mut *mut ::std::os::raw::c_void,
    ) -> i32;

    /**
     * Call this to balance a successful call to AndroidBitmap_lockPixels.
     */
    pub fn AndroidBitmap_unlockPixels(env: *mut JNIEnv, jbitmap: JObject);
}

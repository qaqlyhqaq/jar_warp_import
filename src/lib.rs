use std::ffi::c_void;
use jni::strings::JNIString;
use jni::objects::*;
use jni::JNIEnv;
#[unsafe(no_mangle)]
pub fn test_func(_env: JNIEnv, _class: JClass){
    println!("register_native_methods test_func")
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn JNI_Onload(_env: JNIEnv, _class: jni::objects::JObject){
//     let fn_ptr = test_func;
//     let nmd: jni::NativeMethod = jni::NativeMethod{
//         name: JNIString::from("test_func"),
//         sig: JNIString::from("Ljava/lang/Void;"),
//         fn_ptr: fn_ptr as *mut c_void
//     };
//     JNIEnv::register_native_methods(&_env, _class, &[nmd]).expect("register_native_methods");
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_Onload<'local>(mut _env: JNIEnv<'local>,  _class: JObject<'local>){
    let fn_ptr = test_func;

    let nmd: jni::NativeMethod = jni::NativeMethod{
        name: JNIString::from("test_func"),
        sig: JNIString::from("Ljava/lang/Void;"),
        fn_ptr: fn_ptr as *mut c_void
    };

    let global_ref:GlobalRef = _env.new_global_ref(_class).unwrap();

    JNIEnv::register_native_methods(&mut _env, global_ref, &[nmd]).expect("register_native_methods");
}

use std::ffi::c_void;
use jni::strings::JNIString;
use jni::objects::*;
use jni::JNIEnv;



#[unsafe(no_mangle)]
pub fn Java_org_manta_ray_excel_XlsxParser_testFunc(_env: JNIEnv, _class: JClass){
    println!("register_native_methods test_func")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_Onload<'local>(mut _env: JNIEnv<'local>,  _class: JClass<'local>){
    let fn_ptr = Java_org_manta_ray_excel_XlsxParser_testFunc;

    let nmd: jni::NativeMethod = jni::NativeMethod{
        name: JNIString::from("Java_org_manta_ray_excel_XlsxParser_testFunc"),
        sig: JNIString::from("Ljava/lang/Void;"),
        fn_ptr: fn_ptr as *mut c_void
    };

    println!("native method {}", fn_ptr as usize);
    
    JNIEnv::register_native_methods(&mut _env, _class, &[nmd]).expect("register_native_methods");


}

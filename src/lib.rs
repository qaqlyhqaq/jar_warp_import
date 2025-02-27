use std::ffi::c_void;
use std::io::{Read, Seek, SeekFrom};
use jni::objects::*;
use jni::JNIEnv;
use jni::sys::jsize;

struct JavaInputStreamWrapper<'a> {
    inner: JObject<'a>,
    env: JNIEnv<'a>,
}

// impl<'a> From<( JObject<'a>,&'a mut JNIEnv<'a>)> for JavaInputStreamWrapper<'a>{
//     fn from(value:(JObject<'a>,&'a mut JNIEnv<'a>)) -> Self {
//         JavaInputStreamWrapper{
//             inner: value.0,
//             env: value.1
//         }
//     }
// }

impl <'a> Read for JavaInputStreamWrapper<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len:usize = buf.len();
        if buf_len == 0 {
            return Ok(0);
        }

        let array = self.env.new_byte_array(buf_len as jsize).unwrap();

        let from = JValue::Object(&*array);
        let value = self.env.call_method(&self.inner, "read", "([B)I", &[from]).unwrap();
        let i = value.i().unwrap();
        Ok(i as usize)
    }
}

impl <'a> Seek for JavaInputStreamWrapper<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        todo!()
    }
}


#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_testFunc(_env: JNIEnv, _class: JClass){
    println!("register native lib successfully !");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_nativeParse<'a>(mut _env: JNIEnv<'a>, _class: JClass, jobj: jni::objects::JObject<'a>){
    //Ljava/io/InputStream;

    // let wrapper = JavaInputStreamWrapper::from((jobj,&'a mut _env));
    let wrapper = JavaInputStreamWrapper{
        inner:jobj,
        env: _env,
    };
    println!("call native fun of native successfully !");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_Onload<'local>(mut _env: JNIEnv<'local>,  _class: JClass<'local>){
    // let fn_ptr = Java_org_manta_ray_excel_XlsxParser_testFunc;
    //
    // let nmd: jni::NativeMethod = jni::NativeMethod{
    //     name: JNIString::from("Java_org_manta_ray_excel_XlsxParser_testFunc"),
    //     sig: JNIString::from("Ljava/lang/Void;"),
    //     fn_ptr: fn_ptr as *mut c_void
    // };
    //
    // println!("native method {}", fn_ptr as usize);
    //
    // JNIEnv::register_native_methods(&mut _env, _class, &[nmd]).expect("register_native_methods");
    //

}

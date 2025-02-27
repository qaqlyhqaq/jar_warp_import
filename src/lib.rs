#![feature(slice_as_array)]

use calamine::Reader;
use jni::objects::*;
use jni::sys::jbyte;
use jni::JNIEnv;
use std::io::Cursor;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_testFunc(_env: JNIEnv, _class: JClass){
    println!("register native lib successfully !");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_nativeParse<'a>(mut env: JNIEnv<'a>, _class: JClass<'a>, jByteArrayObject: JByteArray<'a>) -> JObject<'a> {
    let buf_size = env.get_array_length(&jByteArrayObject).unwrap();

    let mut vec1:Vec<jbyte> = Vec::with_capacity(buf_size as usize);

    vec1.resize(buf_size as usize, 0);

    println!("vec1 size:{}", vec1.len());

    let vec1 =  env.convert_byte_array(jByteArrayObject).expect("can't convert byte array");

    let cursor:Cursor<Vec<u8>> = Cursor::new(vec1);
    let mut xlsx = calamine::Xlsx::new(cursor).unwrap();

    let range = xlsx.worksheet_range("1").expect("无法找到相关名称的sheet");

    let listClass = env.find_class("java/util/ArrayList").unwrap();
    //总 list 对象
    let listObject = env.new_object(&listClass, "()V", &[]).unwrap();
    range.rows().for_each(|row| {
        let itemListObject = env.new_object(&listClass, "()V", &[]).unwrap();
        for cell in row {
            let value = env.new_string(cell.to_string()).unwrap();
            // print!("{}", cell);
            let value = JValue::Object(&value);
            env.call_method(&listObject,  "add", "(Ljava/lang/Object;)Z",&[value]).unwrap();
        }
        let itemListObject = JValue::Object(&itemListObject);
        env.call_method(&listObject,  "add", "(Ljava/lang/Object;)Z",&[itemListObject]).unwrap();
    });

    listObject
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

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
    //Ljava/io/InputStream;

    let buf_size = env.get_array_length(&jByteArrayObject).unwrap();

    let mut vec1:Vec<jbyte> = Vec::with_capacity(buf_size as usize);

    vec1.resize(buf_size as usize + 1000, 0);

    println!("vec1 size:{}", vec1.len());

    // _env.get_byte_array_region(jByteArrayObject, buf_size, &mut vec1).expect("can't get byte array");
    let vec1 =  env.convert_byte_array(jByteArrayObject).expect("can't convert byte array");

    let cursor:Cursor<Vec<u8>> = Cursor::new(vec1);
    let mut xlsx = calamine::Xlsx::new(cursor).unwrap();

    let range = xlsx.worksheet_range("1").expect("无法找到相关名称的sheet");


    range.rows().for_each(|row| {
        for cell in row {
            print!("{}", cell);
        }
    });

    //生成list 对象包装 解析结果
    // let listClass = env.find_class("java/lang/String").unwrap();
    let listClass = env.find_class("java/util/ArrayList").unwrap();
    let listObject = env.new_object(&listClass, "()V", &[]).unwrap();
    // let list_add_method = env.get_method_id(&listClass, "add", "(Ljava/lang/Object;)Z").unwrap();

    let string = env.new_string("asdfasdf").unwrap();

    let str_jvalue = JValue::Object(&*string);

    env.call_method(&listObject,  "add", "(Ljava/lang/Object;)Z",&[str_jvalue]).unwrap();


    return listObject;
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

#![feature(slice_as_array)]

use std::io::{Read, Seek, SeekFrom};
use jni::objects::*;
use jni::JNIEnv;
use jni::sys::{jbyte, jsize};
use std::io::Cursor;
use calamine::{Reader, Xlsx};

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_testFunc(_env: JNIEnv, _class: JClass){
    println!("register native lib successfully !");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn Java_org_manta_ray_excel_XlsxParser_nativeParse<'a>(mut _env: JNIEnv<'a>, _class: JClass<'a>, jByteArrayObject: JByteArray<'a>){
    //Ljava/io/InputStream;

    let buf_size = _env.get_array_length(&jByteArrayObject).unwrap();

    let mut vec1:Vec<jbyte> = Vec::with_capacity(buf_size as usize);

    vec1.resize(buf_size as usize + 1000, 0);

    println!("vec1 size:{}", vec1.len());

    // _env.get_byte_array_region(jByteArrayObject, buf_size, &mut vec1).expect("can't get byte array");
    let vec1 =  _env.convert_byte_array(jByteArrayObject).expect("can't convert byte array");

    let cursor:Cursor<Vec<u8>> = Cursor::new(vec1);
    let mut xlsx = calamine::Xlsx::new(cursor).unwrap();

    let range = xlsx.worksheet_range("").unwrap();

    range.rows().for_each(|row| {
        for cell in row {
            print!("{}", cell);
        }
    });


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

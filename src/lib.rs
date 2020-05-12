#[cfg(all(target_arch = "wasm32", target_vendor = "unknown"))]
use js_sys::Array;
#[cfg(all(target_arch = "wasm32", target_vendor = "unknown"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_vendor = "unknown"))]
use wasm_bindgen::JsValue;

use std::fs::File;
use std::io::Read;
use std::io::Seek;

#[cfg(all(target_arch = "wasm32", target_vendor = "unknown"))]
#[cfg_attr(all(target_arch = "wasm32", target_vendor = "unknown"), wasm_bindgen)]
pub fn files(buf: &[u8]) -> Array {
    let mut cursor = std::io::Cursor::new(buf);
    list_files(cursor).into_iter().map(JsValue::from).collect()
}

#[cfg(not(all(target_arch = "wasm32", target_vendor = "unknown")))]
#[cfg_attr(not(all(target_arch = "wasm32", target_vendor = "unknown")), no_mangle)]
pub fn files(path: String) {
    let f = File::open(path).unwrap();
    let files = list_files(f);
    println!("..");
    for file in files {
        println!("{}", file);
    }
}

fn list_files<T: Read + Seek>(buf: T) -> Vec<String> {
    let mut archive = zip::ZipArchive::new(buf).unwrap();

    let mut files = vec![];

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        files.push(outpath.to_str().unwrap().to_string());
    }

    files
}

use std::panic;

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use arrow::{
     ipc::{reader::FileReader, writer::FileWriter}, json::ArrayWriter
};

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = time)]
  fn log_time(s: &str);
  #[wasm_bindgen(js_namespace = console, js_name = timeEnd)]
  fn log_time_end(s: &str);
}


#[wasm_bindgen]
pub fn parse_arrow_to_json(buffer: &Uint8Array) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  log_time("parse_arrow_to_json");

  let buffer = buffer.to_vec();
  let cursor = std::io::Cursor::new(buffer);

  let reader = FileReader::try_new(cursor, None).unwrap();

  let mut writer = ArrayWriter::new(Vec::new());

  for batch in reader {
      let batch = batch.unwrap();
      writer.write(&batch).unwrap();
  }

  writer.finish().unwrap();
  let json_writer = writer.into_inner();
  log_time_end("parse_arrow_to_json");

  let str = String::from_utf8(json_writer).unwrap();
  let js_value = JsValue::from_str(&str);
  js_value

}

#[wasm_bindgen]
pub fn decompress_arrow(buffer: &Uint8Array) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  log_time("decompress_arrow");
  // Convert Uint8Array to Vec<u8>

  let buffer = buffer.to_vec();
  let cursor = std::io::Cursor::new(buffer);

  let reader = FileReader::try_new(cursor, None).unwrap();
  let mut writer = FileWriter::try_new(Vec::new(), &reader.schema().clone()).unwrap();

  for batch in reader {
      let batch = batch.unwrap();
      writer.write(&batch).unwrap();
  }

  writer.finish().unwrap();
  let file = writer.into_inner();
  log_time_end("decompress_arrow");

  // Convert Vec<u8> to Uint8Array
  let js_value = JsValue::from(file.unwrap());
  js_value
}


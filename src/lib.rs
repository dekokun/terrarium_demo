#[macro_use]
extern crate http_guest;

use http_guest::{Request, Response, KVStore};

pub fn user_entrypoint(kvs: &mut KVStore, req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
  let mut name = req.uri().to_string();
  // 最初の "/" 除去
  name.remove(0);
  let kvs_key = &name;
  kvs.append(kvs_key, b"1");
  let count = kvs.get(kvs_key).unwrap().len();
  let body = format!("<!DOCTYPE html>
<html>
<head>
<title></title>
<meta charset='utf-8'>
{}
<input type='button' value='{}に投票' onclick='window.location.reload();' />",
count, name);
  Response::builder()
    .status(200)
    .header("Content-Type", "text/html")
    .body(body.as_bytes().to_owned())
    .unwrap()
}

guest_app_kvs!(user_entrypoint);

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;
use tracing::debug;

mod error;
#[cfg(test)] mod tests;
mod utils;

use std::{net::SocketAddr, vec};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(Deserialize)]
struct PathParams {
  num1: i32,
  num2: i32,
}

/// An endpoint that takes 2 numbers and returns (n1 xor n2).pow(3)
// use the Axum Path extractor to get the path parameters
async fn xor_and_pow(Path(PathParams { num1, num2 }): Path<PathParams>) -> String {
  let xor_result = num1 ^ num2;
  let pow_result = xor_result.pow(3);
  format!("{}", pow_result)
}

// Adapt the handler to work with a variable number of integers
async fn calculate_sled_id(Path(path): Path<String>) -> String {
  debug!("numbers: {:?}", path);
  let numbers = path.split('/').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let xor_result = numbers.into_iter().fold(0, |acc, num| acc ^ num);
  let pow_result = xor_result.pow(3);
  format!("{}", pow_result)
}

#[shuttle_runtime::main]
async fn axum(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(secret_store).unwrap();

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(error_handler))
    // .route("/1/:num1/:num2", get(xor_and_pow))
    .route("/1/*path", get(calculate_sled_id));

  Ok(router.into())
}

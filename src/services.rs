// use serde::Deserialize;
use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
};

use crate::utils;
use crate::DEFAULT_ENCODING;

#[get("/welcome")]
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome. I am Dr. Samuel Hayden, I'm the head of this facility. 
    I think we can work together and resolve this problem in a way that benefits us both.")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/unescape")]
pub async fn unescape(req_body: String) -> impl Responder {

    let unescaped_req_body = utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, &DEFAULT_ENCODING).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/unescape/{encoding}")]
pub async fn unescape_decode(web::Path((encoding,)): web::Path<(String,)>, req_body: String) -> impl Responder {

    let unescaped_req_body = utils::unescape_as_bytes(&req_body).expect("Unable to unescape request's body.");

    let response = utils::attempt_decode(&unescaped_req_body, &encoding).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_base64")]
pub async fn decode_base64(req_body: String) -> impl Responder {

    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, &DEFAULT_ENCODING).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_base64/{encoding}")]
pub async fn decode_base64_encoding(web::Path((encoding,)): web::Path<(String,)>, req_body: String) -> impl Responder {

    let raw_payload = base64::decode(&req_body).expect("Unable to decode base64.");

    let response = utils::attempt_decode(&raw_payload, &encoding).unwrap();

    HttpResponse::Ok().body(response)

}

#[post("/decode_base64_mime")]
pub async fn decode_base64_mime(req_body: String) -> impl Responder {

    let response = utils::decode_mime_subject(&req_body).unwrap();

    HttpResponse::Ok().body(response)

}



// #[derive(Deserialize, Debug)]
// pub struct TestData {
//     payload: String,
//     decode: String,
// }

// #[post("/form_test")]
// pub async fn form_test(request: web::Form<TestData>) -> impl Responder { 
//     format!("{:?}", &request)
// }

// #[post("/json_test")]
// pub async fn json_test(request: web::Json<TestData>) -> impl Responder { 
//     format!("{:?}", &request)
// }
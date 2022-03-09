//    Copyright (C) 2022  Gerald Wu
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU Affero General Public License as published
//    by the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the GNU Affero General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
#![deny(warnings)]

use std::collections::HashMap;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Result, Server, StatusCode};
use lazy_static::{__Deref, lazy_static};
use rand::Rng;

lazy_static! {
    static ref FILES: HashMap<&'static str, Vec<u8>> = {
        HashMap::from([
            ("img/dab.png", include_bytes!("img/dab.png").to_vec()),
            ("img/smug.png", include_bytes!("img/smug.png").to_vec()),
            ("img/aqua.webp", include_bytes!("img/aqua.webp").to_vec()),
        ])
    };
}

fn format_response(status_code: &StatusCode) -> String {
    format!(
        r#"<!DOCTYPE html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>{code}</title>
    </head>
    <body>
        <h1>{code}</h1>
        <img src="/{img}">
    </body>
</html>"#,
        code = status_code,
        img = FILES.keys().collect::<Vec<&&str>>()[rand::thread_rng().gen_range(0..FILES.len())],
    )
}

fn basic_response(status_code: StatusCode) -> Response<Body> {
    let mut response = Response::default();
    *response.status_mut() = status_code;
    response
}

async fn simple_file_send(file: &str) -> Result<Response<Body>> {
    Ok(Response::new(
        FILES.get(file).expect("File server").deref().into(),
    ))
}

async fn status_code_handler(req: Request<Body>) -> Result<Response<Body>> {
    match req.uri().path()[1..].trim() {
        "favicon.ico" => Ok(basic_response(StatusCode::NOT_FOUND)),
        file if FILES.contains_key(&file) => simple_file_send(file).await,
        path => {
            if let Ok(code) = path.parse::<u16>() {
                if let Ok(status_code) = StatusCode::from_u16(code) {
                    let response = Response::builder()
                        .header("Content-Type", "text/html")
                        .status(&status_code)
                        .body(format_response(&status_code).into())
                        .expect("Response builder");
                    Ok(response)
                } else {
                    Ok(basic_response(StatusCode::INTERNAL_SERVER_ERROR))
                }
            } else {
                Ok(Response::default())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = ([0, 0, 0, 0], 3000).into();

    let make_svc =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(status_code_handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on {}", addr);

    server.await?;

    Ok(())
}

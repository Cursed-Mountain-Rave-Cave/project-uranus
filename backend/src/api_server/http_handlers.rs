use crate::game_server::requests::*;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

mod requests;
mod responses;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/play")]
pub async fn play(
    game_request: Json<requests::GameRequest>,
    game_server: Data<actix::Addr<crate::game_server::GameServer>>,
) -> impl Responder {
    log::debug!("game_server: {:?}", game_server);
    log::debug!("player_id: {}", game_request.player_id);

    let result = game_server
        .send(register::Register {
            player_id: game_request.player_id.clone(),
        })
        .await;

    let response = match result {
        Ok(session_id) => responses::Play {
            code: "ok".to_owned(),
            session_id,
        },
        _ => responses::Play {
            code: "error".to_owned(),
            session_id: "".to_owned(),
        },
    };

    HttpResponse::Ok().json(response)
}

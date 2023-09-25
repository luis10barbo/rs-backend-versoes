mod db;

use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use db::adquirir_conexao;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::db::{
    adicionar_versao, adquirir_versoes, modificar_versao, remover_versao, Versao, VersaoId,
};

struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db: Arc::new(Mutex::new(adquirir_conexao().unwrap())),
            }))
            .service(versao_delete)
            .service(versao_edit)
            .service(versao_post)
            .service(versao_get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/versao")]
async fn versao_get(data: web::Data<AppState>) -> impl Responder {
    let versao = adquirir_versoes(&data.db.lock().unwrap());
    if versao.is_err() {
        return HttpResponse::InternalServerError().body(format!("{}", versao.unwrap_err()));
    }
    return HttpResponse::Ok().json(versao.unwrap());
}

#[post("/versao")]
async fn versao_post(data: web::Data<AppState>, info: web::Json<Versao>) -> impl Responder {
    let versao = Versao {
        id: 0,
        versao: info.versao,
        download: info.download.clone(),
    };
    let res = adicionar_versao(&data.db.lock().unwrap(), &versao);
    if res.is_ok() {
        return HttpResponse::Ok().body(format!(
            "Versao {} adicionada com download {}",
            versao.id, versao.download
        ));
    }
    HttpResponse::Ok().body(format!("Falha ao adicionar versao: {}", res.unwrap_err()))
}

#[derive(Deserialize)]
struct Id {
    id: i32,
}

#[delete("/versao")]
async fn versao_delete(data: web::Data<AppState>, info: web::Json<Id>) -> impl Responder {
    let res = remover_versao(&data.db.lock().unwrap(), info.id);
    if res.is_ok() {
        return HttpResponse::Ok().body("Versao removida");
    }
    HttpResponse::InternalServerError().body(res.unwrap_err().to_string())
}

#[post("/versao")]
async fn versao_edit(data: web::Data<AppState>, info: web::Json<VersaoId>) -> impl Responder {
    let versao = VersaoId {
        download: info.download.clone(),
        id: info.id,
        versao: info.versao,
    };
    let res = modificar_versao(&data.db.lock().unwrap(), &versao);
    if res.is_err() {
        return HttpResponse::Ok().body(format!("Versao nao editada: {}", res.unwrap_err()));
    }
    HttpResponse::Ok().body("Versao editada")
}

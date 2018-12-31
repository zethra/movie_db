use actix::prelude::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpResponse, Path,
    State,
};
use futures::future::Future;

use crate::db::{DbExecutor, CreateMovie};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn movies((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {

    state
        .db
        .send(CreateMovie {
            title: name.into_inner()
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

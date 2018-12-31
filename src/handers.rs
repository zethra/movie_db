use actix::prelude::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpResponse, Form,
    State,
};
use futures::future::Future;

use crate::db::{DbExecutor, CreateMovie};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn create_movie((create_movie, state): (Form<CreateMovie>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(create_movie.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

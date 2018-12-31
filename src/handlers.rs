use actix::prelude::*;
use actix_web::{
    AsyncResponder, FutureResponse, HttpResponse, Form, Query,
    State,
};
use futures::future::Future;

use crate::db::{DbExecutor, CreateMovie, DeleteMovie, GetMovie};

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

pub fn delete_movie((delete_movie, state): (Query<DeleteMovie>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(delete_movie.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn get_movie((get_movie, state): (Query<GetMovie>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(get_movie.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(movie) => Ok(HttpResponse::Ok().json(movie)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

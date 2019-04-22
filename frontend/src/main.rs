use failure::Error;
use serde_derive::{Deserialize, Serialize};
use yew::{html, start_app, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub rating: String,
    pub category: String,
    pub format: String,
    pub aspect: String,
    pub actors: String,
    pub drawer: String,
    pub column: String,
}

struct Model {
    link: ComponentLink<Model>,
    data: Option<Vec<Movie>>,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
}

enum Msg {
    FetchMovies,
    FetchMoviesReady(Result<Vec<Movie>, Error>),
    FetchError,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,
            data: None,
            fetch_service: FetchService::new(),
            ft: None,
        };
        model.load_movies("/all_movies.json");
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchMoviesReady(data) => {
                self.data = data.ok();
            }
            Msg::FetchMovies => {
                self.load_movies("/api/all_movies");
            }
            _ => {
                unimplemented!();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Model> {
        if let Some(movies) = &self.data {
            html! {
                <div>
                    <button onclick=|_| Msg::FetchMovies,>{ "Fetch Movies" }</button>
                    <ul>
                        { for movies.iter().map(view_movie) }
                    </ul>
                </div>
            }
        } else {
            html! {
                <div>
                    <button onclick=|_| Msg::FetchMovies,>{ "Fetch Movies" }</button>
                </div>
            }
        }
    }
}

impl Model {
    fn load_movies(&mut self, url: &str) {
        let callback = self.link
            .send_back(move |response: Response<Json<Result<Vec<Movie>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Msg::FetchMoviesReady(data)
                } else {
                    Msg::FetchError
                }
            });
        let request = Request::get(url).body(Nothing).unwrap();
        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}

fn view_movie(movie: &Movie) -> Html<Model> {
    html! {
        <li>{ movie.title.clone() }</li>
    }
}

fn main() {
    start_app::<Model>();
}
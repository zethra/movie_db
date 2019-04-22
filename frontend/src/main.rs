use failure::Error;
use serde_derive::{Deserialize, Serialize};
use yew::{html, start_app, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Movie {
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

enum Scene {
    Loading,
    Main,
}

struct Model {
    link: ComponentLink<Model>,
    data: Option<Vec<Movie>>,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
    scene: Scene,
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
            scene: Scene::Loading,
        };
        model.load_movies("/api/all_movies");
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchMoviesReady(data) => {
                self.data = data.ok();
                self.scene = Scene::Main;
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
        match self.scene {
            Scene::Loading => {
                view_page(html! {
                    <div>{ "Loading" }</div>
                })
            }
            Scene::Main => {
                if let Some(movies) = &self.data {
                    view_page(html! {
                        <section class="list",>
                            { for movies.iter().enumerate().map(view_movie_title) }
                        </section>
                    })
                } else {
                    view_page(html! {
                        <div>
                            { "Error" }
                        </div>
                    })
                }
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

fn view_movie_title((idx, movie): (usize, &Movie)) -> Html<Model> {
    if idx % 2 == 0 {
        html! { <div class="even",> { movie.title.clone() } </div> }
    } else {
        html! { <div class="odd",> { movie.title.clone() } </div> }
    }
}

fn view_page(main: Html<Model>) -> Html<Model> {
    html! {
        <div>
            <header>
                <h1>{ "Movie DB" }</h1>
            </header>
            <main>
                { main }
            </main>
        </div>
    }
}

fn main() {
    start_app::<Model>();
}
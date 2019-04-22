#![recursion_limit="128"]
use failure::Error;
use serde_derive::{Deserialize, Serialize};
use yew::{html, start_app, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const ALL_MOVIES: &str = "/api/all_movies";
const MOVIE: &str = "/api/movie";

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
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
    Main(Option<Vec<Movie>>),
    AddMovie(Movie),
}

struct Model {
    link: ComponentLink<Model>,
    fetch_service: FetchService,
    ft: Option<FetchTask>,
    scene: Scene,
}

enum Msg {
    Main,
    MainReady(Result<Vec<Movie>, Error>),
    FetchError,
    AddMovie,
    AddMovieEditTitle(String),
    AddMovieEditRating(String),
    AddMovieEditCategory(String),
    AddMovieEditFormat(String),
    AddMovieEditAspect(String),
    AddMovieEditActors(String),
    AddMovieEditDrawer(String),
    AddMovieEditColumn(String),
    AddMovieSubmit,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,
            fetch_service: FetchService::new(),
            ft: None,
            scene: Scene::Loading,
        };
        model.load_movies(ALL_MOVIES);
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainReady(data) => {
                self.scene = Scene::Main(data.ok());
            }
            Msg::Main => {
                self.load_movies(ALL_MOVIES);
            }
            Msg::AddMovie => {
                self.scene = Scene::AddMovie(Default::default());
            }
            Msg::AddMovieEditTitle(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.title = data;
                }
            }
            Msg::AddMovieEditRating(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.rating = data;
                }
            }
            Msg::AddMovieEditCategory(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.category = data;
                }
            }
            Msg::AddMovieEditFormat(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.format = data;
                }
            }
            Msg::AddMovieEditAspect(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.aspect = data;
                }
            }
            Msg::AddMovieEditActors(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.actors = data;
                }
            }
            Msg::AddMovieEditDrawer(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.drawer = data;
                }
            }
            Msg::AddMovieEditColumn(data) => {
                if let Scene::AddMovie(movie) = &mut self.scene {
                    movie.column = data;
                }
            }
            Msg::AddMovieSubmit => {
                if let Scene::AddMovie(movie) = &self.scene {
                    let callback = self.link
                        .send_back(move |response: Response<Json<Result<(), Error>>>| {
                            let (meta, _) = response.into_parts();
                            println!("META: {:?}", meta);
                            if meta.status.is_success() {
                                Msg::Main
                            } else {
                                Msg::FetchError
                            }
                        });
                    let request = Request::post(MOVIE)
                        .header("Content-Type", "application/json")
                        .body(Json(&movie))
                        .expect("Failed to construct request");
                    let task = self.fetch_service.fetch(request, callback);
                    self.ft = Some(task);
                }
            }
            Msg::FetchError => {
                println!("Fetch Error");
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Model> {
        match &self.scene {
            Scene::Loading => {
                view_page(html! {
                    <div>{ "Loading" }</div>
                })
            }
            Scene::Main(movies) => {
                if let Some(movies) = &movies {
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
            Scene::AddMovie(movie) => {
                view_page(html! {
                <div>
                    <label>{ "Title" }</label>
                    <input type="text",
                           value=&movie.title,
                           oninput=|e| Msg::AddMovieEditTitle(e.value), />
                    <label>{ "Rating" }</label>
                    <input type="text",
                           value=&movie.rating,
                           oninput=|e| Msg::AddMovieEditRating(e.value), />
                    <label>{ "Category" }</label>
                    <input type="text",
                           value=&movie.category,
                           oninput=|e| Msg::AddMovieEditCategory(e.value), />
                    <label>{ "Format" }</label>
                    <input type="text",
                           value=&movie.format,
                           oninput=|e| Msg::AddMovieEditFormat(e.value), />
                    <label>{ "Aspect" }</label>
                    <input type="text",
                           value=&movie.aspect,
                           oninput=|e| Msg::AddMovieEditAspect(e.value), />
                    <label>{ "Actors" }</label>
                    <input type="text",
                           value=&movie.actors,
                           oninput=|e| Msg::AddMovieEditActors(e.value), />
                    <label>{ "Drawer" }</label>
                    <input type="text",
                           value=&movie.actors,
                           oninput=|e| Msg::AddMovieEditDrawer(e.value), />
                    <label>{ "Column" }</label>
                    <input type="text",
                           value=&movie.actors,
                           oninput=|e| Msg::AddMovieEditColumn(e.value), />
                    <button onclick=|_| Msg::AddMovieSubmit,>{ "Add" }</button>
                </div>
                })
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
                    Msg::MainReady(data)
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
                <h1 onclick=|_| Msg::Main,>{ "Movie DB" }</h1>
                <a onclick=|_| Msg::AddMovie,>{ "Add Movie" }</a>
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
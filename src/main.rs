use yew::prelude::*;
use yew_router::prelude::*;

mod components;

mod hooks;

mod pages;
use pages::home::Home;
use pages::page_not_found::PageNotFound;

mod state;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {}
pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav() }
                <section>
                    <div class="container my-6">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </section>
                { self.view_footer() }
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self) -> Html {
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Example" }</h1>
                </div>
            </nav>
        }
    }

    fn view_footer(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by masamiki" }
                </div>
            </footer>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}

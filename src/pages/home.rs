use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?yew" />
                    </figure>
                </div>
            </div>
        }
    }
}

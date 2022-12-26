use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
        { "Welcome to my website! This site is a work in progress so bear with any inconsistencies. This site is built completely using rust using the code located at https://github.com/jallen02/website" }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}



use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <img class="profile_pic" src="assets/profile_pic.jpeg"/>
            <Title />
            <Footer />
        </div>
    }
}

#[function_component(Title)]
fn title() -> Html {
    html! {
        <div class="heading">
            <h1 class="title">{"Jason Allen's website"}</h1>
            <p class="explainer">
                {"I am a backend engineer trying his hand at web dev."}
            </p>
            <p class="explainer">
                {"Code for this website is available on "}
                <a href="https://github.com/jallen02/website"> {"Github"} </a>
            </p>
        </div>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer>
            <p>
                {"This website is written 100% from scratch in Rust (as well as CSS for styling) and utilizes the Yew framework. "}
                {"This website is a work in progress, and will have visual inconsistencies as I learn and iterate."}
            </p>
        </footer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

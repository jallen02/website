use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Intro(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex items-center">
            <img class="rounded-full w-32 h-32" src="assets/profile_pic.jpeg"/>
            <p class="align-super p-3 font-medium">
                <h1 class="text-3xl"> "Jason Allen" </h1>
                "Backend software dev"
            </p>
        </div>
        <div > 
            "This webpage was built using "
            <a href="https://github.com/leptos-rs/leptos" target="_blank" class="text-blue-600 hover:underline"> "Leptos" </a>
            " and the source is available on "
            <a href="https://github.com/jallen02/website" target="_blank" class="text-blue-600 hover:underline"> "Github" </a>
            "."
        </div>
    }
}

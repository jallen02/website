use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Intro(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex p-1 items-center">
            <img class="rounded-full w-32 h-32" src="assets/profile_pic.jpeg"/>
            <p class="align-super p-3">
                <h1 class="text-3xl"> "Jason Allen" </h1>
                <span> "Backend software dev" </span>
            </p>
        </div>
    }
}

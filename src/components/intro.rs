use leptos::{component, view, IntoView, Scope};

#[component]
pub fn Intro(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="heading">
            <img class="heading-item profile-pic" src="assets/profile_pic.jpeg"/>
            <p class="heading heading-item">
                <h1> "Jason Allen" </h1>
                <span> "Backend software dev." </span>
            </p>
        </div>
    }
}

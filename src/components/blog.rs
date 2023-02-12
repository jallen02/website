use leptos::{component, Scope, IntoView, view};

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    view! { cx, 
        <a href="book/index.html"> "Blog" </a>
    }

}

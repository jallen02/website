use leptos::{component, Scope, IntoView, view};

#[component]
pub fn Blog(cx: Scope) -> impl IntoView {
    view! { cx, 
        <a href="book/index.html" class="text-blue-600 hover:underline"> "Blog" </a>
    }
}

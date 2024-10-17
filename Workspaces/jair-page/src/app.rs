use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Comecando!" }</h1>
            <span class="subtitle">{ " Rust " }<i class="heart" /></span>
        </main>
    }
}

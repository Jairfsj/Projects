use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! {
        <div>
            <h1>{ "Contato" }</h1>
            <p>{ "Entre em contato comigo atraves do email: jair@exemplo" }</p>
        </div>
    }
}

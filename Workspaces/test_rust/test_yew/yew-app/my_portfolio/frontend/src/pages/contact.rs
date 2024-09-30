use yew::prelude::*;
use crate::components::{Header, Footer};

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Contact
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <main>
                    <h1>{"Contato"}</h1>
                    <p>{"Você pode me contatar através do e-mail: exemplo@dominio.com"}</p>
                </main>
                <Footer />
            </>
        }
    }
}

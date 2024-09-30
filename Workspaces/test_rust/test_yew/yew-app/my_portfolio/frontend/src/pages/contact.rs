use yew::prelude::*;
use crate::componentes::{Header, Footer};

pub struct Projects;

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_:Self::Properties, _: ComponentLink<Self>) -> Self {
        Projects
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
           <>
                <Header />
                <main>
                    <h1>{"Projetos Executados"}</h1>
                    <ul>
                        <li>{"Projeto 1: Um sistema de chat em tempo real"}</li>
                        <li>{"Projeto 2: Um site de protfolio"}</li>   
                    </ul>
                </main>
                <Footer />
            </>
        }
    }
}

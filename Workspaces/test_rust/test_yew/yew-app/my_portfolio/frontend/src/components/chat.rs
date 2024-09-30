use yew::prelude::*;

pub struct Chat {
    link: ComponentLink<Self>,
    messages: Vec<String>,
    input_value: String,

}

pub enum Msg {
    SendMessage,
    UpdateInput(String),

}

impl Component for Chat {
    type Messag = Msg;
    type Properties = ();


    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Chat {
            link,
            message: vec![],
            input_value: String::new(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => {
                if !self.input_value.is_empty(){
                    self.messages.push(self.input_value.clones());

                }
                true
            }
            Msg::UpdateInput(value) => {
                self.input_value;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="chat">
                <h2>{"Chat en Tempo Real"}</h2>
                <div class="messages">
                    { for self.messages.iter().map(|msg| html! { <div>{msg}</div>}) }
                </div>
                <input type="text" value=&self.input_value oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value)) />
                <button onclick=self.link.callback(|_| Msg::SendMessage)>{"Enviar"}</button>
            </div>
        }
    }
}

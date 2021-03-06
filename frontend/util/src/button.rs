use yew::prelude::*;

//use util::color::Color;

pub struct Button {
    title: String,
    //color: Color,
    disabled: bool,
    onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    //pub color: Color,
    pub disabled: bool,
    pub onclick: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Button".into(),
            //color: Color::Primary,
            disabled: false,
            onclick: None,
        }
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {
            title: props.title,
            //            color: Color::Primary,
            disabled: props.disabled,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        //self.color = props.color;
        self.disabled = props.disabled;
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {

        //<button class=("btn", &self.color.to_button_class()), disabled=self.disabled, onclick=|_| Msg::Clicked,>{ &self.title }</button>
        html! {
            <button class=("btn", "green"), disabled=self.disabled, onclick=|_| Msg::Clicked,>{ &self.title }</button>
        }
    }
}

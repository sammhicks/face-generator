use yew::prelude::*;

pub struct ColorPicker {
    link: ComponentLink<Self>,
    props: Props,
    initial_value: String,
}

#[derive(Debug)]
pub enum Msg {
    ValueChanged(yew::html::InputData),
    ResetClicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub name: &'static str,
    #[props(required)]
    pub value: *mut String,
    #[props(required)]
    pub value_changed: Callback<()>,
}

impl Component for ColorPicker {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let initial_value = unsafe { (*props.value).clone() };
        Self {
            link,
            props,
            initial_value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueChanged(data) => unsafe { *self.props.value = data.value },
            Msg::ResetClicked => unsafe { *self.props.value = self.initial_value.clone() },
        };
        self.props.value_changed.emit(());
        true
    }

    fn view(&self) -> Html {
        let value = unsafe { (*self.props.value).clone() };

        html! {
            <div class="slider">
                <span class="label">{self.props.name}</span>
                <input type="color" value=value  oninput=self.link.callback(Msg::ValueChanged)/>
                <output>{format!("{}", value)}</output>
                <button onclick=self.link.callback(|_| Msg::ResetClicked)>{"Reset"}</button>
            </div>
        }
    }
}

#[macro_export]
macro_rules! color {
    ( $name:expr, $value:expr, $value_changed:expr) => {
        html!(<ColorPicker name=$name value=&mut $value as *mut String value_changed=$value_changed.clone() />)
    };
}

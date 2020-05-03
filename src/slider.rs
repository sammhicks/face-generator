use yew::{prelude::*, services::ConsoleService};

use crate::value::{SharedValue, Value};

fn set_value(console: &mut ConsoleService, value: &mut Value, data: yew::html::InputData) {
    match data.value.parse() {
        Ok(v) => *value = v,
        Err(err) => console.error(&format!("{:?}", err)),
    }
}

pub struct Slider {
    link: ComponentLink<Self>,
    props: Props,
    initial_value: Value,
    console: ConsoleService,
}

pub enum Msg {
    MinChanged(yew::html::InputData),
    MaxChanged(yew::html::InputData),
    ValueChanged(yew::html::InputData),
    ResetClicked,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub name: &'static str,
    #[props(required)]
    pub min: Value,
    #[props(required)]
    pub max: Value,
    #[props(required)]
    pub value: SharedValue,
    #[props(required)]
    pub value_changed: Callback<()>,
}

impl Component for Slider {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let initial_value = props.value.get();
        Slider {
            link,
            props,
            initial_value,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MinChanged(data) => set_value(&mut self.console, &mut self.props.min, data),
            Msg::MaxChanged(data) => set_value(&mut self.console, &mut self.props.max, data),
            Msg::ValueChanged(data) => {
                self.props.value.set_str(&mut self.console, data);
                self.props.value_changed.emit(());
            }
            Msg::ResetClicked => {
                self.props.value.set(self.initial_value);
                self.props.value_changed.emit(());
            }
        };
        true
    }

    fn view(&self) -> Html {
        let value = self.props.value.get();

        let max_is_large = self.props.max > 5.0;

        let value_step = if max_is_large { 0.5 } else { 0.1 };
        let max_step = if max_is_large { 1.0 } else { 0.5 };

        html! {
            <div class="slider">
                <span class="label">{self.props.name}</span>
                <input type="number" value=self.props.min oninput=self.link.callback(Msg::MinChanged)/>
                <input type="range" min=self.props.min max=self.props.max value=value step=value_step oninput=self.link.callback(Msg::ValueChanged)/>
                <output>{format!("{:05.2}", value)}</output>
                <input type="number" value=self.props.max step=max_step oninput=self.link.callback(Msg::MaxChanged)/>
                <button onclick=self.link.callback(|_| Msg::ResetClicked)>{"Reset"}</button>
            </div>
        }
    }
}

use yew::{prelude::*, services::ConsoleService};

use crate::value::Value;

fn set_value(console: &mut ConsoleService, value: *mut Value, data: yew::html::InputData) {
    match data.value.parse() {
        Ok(v) => unsafe { *value = v },
        Err(err) => console.error(&format!("{:?}", err)),
    }
}

pub struct Slider {
    link: ComponentLink<Self>,
    props: Props,
    initial_value: Value,
    console: ConsoleService,
}

#[derive(Debug)]
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
    pub value: *mut Value,
    #[props(required)]
    pub value_changed: Callback<()>,
}

impl Component for Slider {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let initial_value = unsafe { *props.value };
        Self {
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
                set_value(&mut self.console, self.props.value, data);
                self.props.value_changed.emit(());
            }
            Msg::ResetClicked => {
                unsafe { *self.props.value = self.initial_value };
                self.props.value_changed.emit(());
            }
        };
        true
    }

    fn view(&self) -> Html {
        let value = unsafe { *self.props.value };

        let max_is_large = self.props.max > 5.0;
        let max_is_small = self.props.max < 1.0;

        let value_step = if max_is_large {
            0.5
        } else if max_is_small {
            0.05
        } else {
            0.1
        };
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

#[macro_export]
macro_rules! slider {
    ( $name:expr, $min:expr, $max:expr, $value:expr, $value_changed:ident ) => {
        html!(<Slider name=$name min=$min max=$max value=&mut $value as *mut Value value_changed=$value_changed.clone() />)
    };
}

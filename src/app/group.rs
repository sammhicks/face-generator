use yew::prelude::*;

pub struct Group {
    link: ComponentLink<Self>,
    props: Props,
    show_children: bool,
}

pub enum Msg {
    ShowHide,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub name: &'static str,
    pub children: Children,
}

impl Component for Group {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            show_children: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowHide => {
                self.show_children = !self.show_children;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html!(
            <fieldset class={if self.show_children {""} else {"hide_children"}}>
                <legend class="clickable" onclick=self.link.callback(|_|Msg::ShowHide)>{self.props.name}</legend>
                { self.props.children.render() }
            </fieldset>
        )
    }
}

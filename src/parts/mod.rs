use yew::prelude::*;

use crate::{group::Group, slider::Slider, value::Value};

pub mod eyes;
pub mod face;
pub mod mouth;
pub mod nose;

pub use eyes::*;
pub use face::*;
pub use mouth::*;
pub use nose::*;

pub struct Head {
    pub face: Face,
    pub eyes: Eyes,
    pub nose: Nose,
    pub mouth: Mouth,
}

impl Head {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html! {
            <>
                {self.face.controls(value_changed.clone())}
                {self.eyes.controls(value_changed.clone())}
                {self.nose.controls(value_changed.clone())}
                {self.mouth.controls(value_changed.clone())}
            </>
        }
    }

    pub fn view(&self) -> Html {
        html!(
            <>
                {self.face.view()}
                {self.eyes.view()}
                {self.nose.view()}
                {self.mouth.view(0.5 * self.face.height)}
            </>
        )
    }
}

pub struct ScaledHead {
    pub scale: Value,
    pub head: Head,
}

impl ScaledHead {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <>
                <Group name="Grid">
                    {crate::slider!("Size", 8.0, 32.0, self.scale, value_changed)}
                </Group>
                {self.head.controls(value_changed.clone())}
            </>
        )
    }

    pub fn view(&self) -> Html {
        html!(
            <g transform=format!("scale({})", self.scale)>
                {self.head.view()}
            </g>
        )
    }

    pub fn size(&self, border: Value) -> (Value, Value) {
        (
            self.scale * (2.0 * border + self.head.face.width),
            self.scale * (2.0 * border + self.head.face.height),
        )
    }
}

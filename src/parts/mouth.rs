use yew::prelude::*;

use crate::{color::ColorPicker, group::Group, slider::Slider, value::Value};

pub struct TopLip {
    pub roundness: (Value, Value),
    pub philtrum: (Value, Value),
}

impl TopLip {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Top Lip">
                {crate::slider!("RX", 0.0, 2.0, self.roundness.0)}
                {crate::slider!("RY", 0.0, 2.0, self.roundness.1)}
                <Group name="Philtrum">
                    {crate::slider!("Width", 0.0, 1.0, self.philtrum.0)}
                    {crate::slider!("Position", 0.0, 1.0, self.philtrum.1)}
                </Group>
            </Group>
        )
    }
}

pub struct BottomLip {
    pub roundness: (Value, Value),
}

impl BottomLip {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Bottom Lip">
                {crate::slider!("RX", 0.0, 2.0, self.roundness.0)}
                {crate::slider!("RY", 0.0, 2.0, self.roundness.1)}
            </Group>
        )
    }
}

pub struct Mouth {
    pub color: String,
    pub position: Value,
    pub width: Value,
    pub top_lip: TopLip,
    pub bottom_lip: BottomLip,
    pub smile: (Value, Value),
}

impl Mouth {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Mouth">
                {crate::color!("Color", self.color)}
                {self.top_lip.controls(value_changed.clone())}
                {self.bottom_lip.controls(value_changed.clone())}
                <Group name="Smile">
                    {crate::slider!("Width", 0.0, 2.0, self.smile.0)}
                    {crate::slider!("Height", 0.0, 2.0, self.smile.1)}
                </Group>
            </Group>
        )
    }

    pub fn view(&self, face_ry: Value) -> Html {
        let y = self.position * face_ry;
        let left = -0.5 * self.width;
        let right = 0.5 * self.width;
        let x = 0.0;

        let lip = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} Z",
            left,
            y,
            left + self.top_lip.roundness.0,
            y - self.top_lip.roundness.1,
            x - self.top_lip.philtrum.0,
            y - self.top_lip.philtrum.1,
            x,
            y - self.top_lip.philtrum.1,
            x + self.top_lip.philtrum.0,
            y - self.top_lip.philtrum.1,
            right - self.top_lip.roundness.0,
            y - self.top_lip.roundness.1,
            right,
            y,
            right - self.bottom_lip.roundness.0,
            y + self.bottom_lip.roundness.1,
            left + self.bottom_lip.roundness.0,
            y + self.bottom_lip.roundness.1,
            left,
            y
        );

        let smile = format!(
            "M{} {} C{} {}, {} {}, {} {}",
            left,
            y,
            left + self.smile.0,
            y + self.smile.1,
            right - self.smile.0,
            y + self.smile.1,
            right,
            y
        );

        html!(
            <>
                <path class="line" style=format!("fill: {};", self.color) d=lip />
                <path class="line" d=smile />
            </>
        )
    }
}

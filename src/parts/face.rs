use yew::prelude::*;

use crate::{color::ColorPicker, group::Group, slider::Slider, value::Value};

fn bezier_arch(rx: Value, ry: Value, roundness: (Value, Value), sx: Value, sy: Value) -> String {
    let left = sx * -rx;
    let right = sx * rx;

    let middle_y = sy * ry;

    let roundness_x = sx * roundness.0;
    let roundness_y = sy * roundness.1;

    format!(
        "C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
        left,
        roundness_y,
        -roundness_x,
        middle_y,
        0.0,
        middle_y,
        roundness_x,
        middle_y,
        right,
        roundness_y,
        right,
        0.0,
    )
}

pub struct Forehead {
    pub roundness: (Value, Value),
}

impl Forehead {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Forehead">
                {crate::slider!("RX", 0.0, 7.0, self.roundness.0)}
                {crate::slider!("RY", 0.0, 10.0, self.roundness.1)}
            </Group>
        )
    }
}

pub struct Fringe {
    pub hair_color: String,
    pub thickness: Value,
    pub roundness: (Value, Value),
}

impl Fringe {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Fringe">
                {crate::color!("Color", self.hair_color)}
                {crate::slider!("Thickness", 0.0, 10.0, self.thickness)}
                {crate::slider!("RX", 0.0, 7.0, self.roundness.0)}
                {crate::slider!("RY", 0.0, 10.0, self.roundness.1)}
            </Group>
        )
    }
}

pub struct Chin {
    pub roundness: (Value, Value),
}

impl Chin {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Chin">
                {crate::slider!("RX", 0.0, 7.0, self.roundness.0)}
                {crate::slider!("RY", 0.0, 10.0, self.roundness.1)}
            </Group>
        )
    }
}

pub struct Face {
    pub skin_color: String,
    pub width: Value,
    pub height: Value,
    pub forehead: Forehead,
    pub fringe: Fringe,
    pub chin: Chin,
}

impl Face {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Face">
                {crate::color!("Skin", self.skin_color)}
                {crate::slider!("Width", 0.0, 16.0, self.width)}
                {crate::slider!("Height", 0.0, 32.0, self.height)}
                {self.forehead.controls(value_changed.clone())}
                {self.fringe.controls(value_changed.clone())}
                {self.chin.controls(value_changed.clone())}
            </Group>
        )
    }

    pub fn view(&self) -> Html {
        let face_rx = 0.5 * self.width;
        let face_ry = 0.5 * self.height;

        let face = {
            let top = bezier_arch(
                face_rx,
                face_ry - 0.5 * self.fringe.thickness,
                self.fringe.roundness,
                1.0,
                -1.0,
            );

            let bottom = bezier_arch(face_rx, face_ry, self.chin.roundness, -1.0, 1.0);
            html!(<path id="face" class="line" style=format!("fill: {};", self.skin_color) d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        let hair = {
            let top = bezier_arch(face_rx, face_ry, self.forehead.roundness, 1.0, -1.0);
            let bottom = bezier_arch(
                face_rx,
                face_ry - self.fringe.thickness,
                self.fringe.roundness,
                -1.0,
                -1.0,
            );

            html!(<path id="hair" class="line" style=format!("fill: {};", self.fringe.hair_color) d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        yew::virtual_dom::VList::new_with_children(vec![face, hair]).into()
    }
}

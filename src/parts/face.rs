use yew::prelude::*;

use crate::{
    group::Group,
    slider::Slider,
    value::{SharedPair, SharedValue, Value},
};

fn bezier_arch(rx: Value, ry: Value, roundness: &SharedPair, sx: Value, sy: Value) -> String {
    let left = sx * -rx;
    let right = sx * rx;

    let middle_y = sy * ry;

    let roundness_x = sx * roundness.0.get();
    let roundness_y = sy * roundness.1.get();

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
    pub roundness: SharedPair,
}

impl Forehead {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Forehead">
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

pub struct Fringe {
    pub thickness: SharedValue,
    pub roundness: SharedPair,
}

impl Fringe {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Fringe">
                <Slider name="Thickness" min=0.0 max=10.0 value=self.thickness.clone() value_changed=value_changed.clone() />
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

pub struct Chin {
    pub roundness: SharedPair,
}

impl Chin {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Chin">
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

pub struct Face {
    pub width: SharedValue,
    pub height: SharedValue,
    pub forehead: Forehead,
    pub fringe: Fringe,
    pub chin: Chin,
}

impl Face {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Face">
                <Slider name="Width" min=0.0 max=16.0 value=self.width.clone() value_changed=value_changed.clone() />
                <Slider name="Height" min=0.0 max=32.0 value=self.height.clone() value_changed=value_changed.clone() />
                {self.forehead.controls(value_changed.clone())}
                {self.fringe.controls(value_changed.clone())}
                {self.chin.controls(value_changed.clone())}
            </Group>
        )
    }

    pub fn view(&self) -> Html {
        let face_rx = 0.5 * self.width.get();
        let face_ry = 0.5 * self.height.get();

        let face = {
            let top = bezier_arch(
                face_rx,
                face_ry - 0.5 * self.fringe.thickness.get(),
                &self.fringe.roundness,
                1.0,
                -1.0,
            );

            let bottom = bezier_arch(face_rx, face_ry, &self.chin.roundness, -1.0, 1.0);
            html!(<path id="face" class="line" d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        let hair = {
            let top = bezier_arch(face_rx, face_ry, &self.forehead.roundness, 1.0, -1.0);
            let bottom = bezier_arch(
                face_rx,
                face_ry - self.fringe.thickness.get(),
                &self.fringe.roundness,
                -1.0,
                -1.0,
            );

            html!(<path id="hair" class="line" d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        html!(
            <>
                {face}
                {hair}
            </>
        )
    }
}

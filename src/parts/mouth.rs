use yew::prelude::*;

use crate::{
    group::Group,
    slider::Slider,
    value::{SharedPair, SharedValue, Value},
};

pub struct TopLip {
    pub roundness: SharedPair,
    pub philtrum: SharedPair,
}

impl TopLip {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Top Lip">
                <Slider name="RX" min=0.0 max=2.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=2.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
                <Group name="Philtrum">
                    <Slider name="Width" min=0.0 max=1.0 value=self.philtrum.0.clone() value_changed=value_changed.clone() />
                    <Slider name="Position" min=0.0 max=1.0 value=self.philtrum.1.clone() value_changed=value_changed.clone() />
                </Group>
            </Group>
        )
    }
}

pub struct BottomLip {
    pub roundness: SharedPair,
}

impl BottomLip {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Bottom Lip">
                <Slider name="RX" min=0.0 max=2.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=2.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

pub struct Mouth {
    pub position: SharedValue,
    pub width: SharedValue,
    pub top_lip: TopLip,
    pub bottom_lip: BottomLip,
    pub smile: SharedPair,
}

impl Mouth {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Mouth">
                { self.top_lip.controls(value_changed.clone()) }
                { self.bottom_lip.controls(value_changed.clone()) }
                <Group name="Smile">
                    <Slider name="Width" min=0.0 max=2.0 value=self.smile.0.clone() value_changed=value_changed.clone() />
                    <Slider name="Height" min=0.0 max=2.0 value=self.smile.1.clone() value_changed=value_changed.clone() />
                </Group>
            </Group>
        )
    }

    pub fn view(&self, face_ry: Value) -> Html {
        let y = self.position.get() * face_ry;
        let left = -0.5 * self.width.get();
        let right = 0.5 * self.width.get();
        let x = 0.0;

        let lip = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} Z",
            left,
            y,
            left + self.top_lip.roundness.0.get(),
            y - self.top_lip.roundness.1.get(),
            x - self.top_lip.philtrum.0.get(),
            y - self.top_lip.philtrum.1.get(),
            x,
            y - self.top_lip.philtrum.1.get(),
            x + self.top_lip.philtrum.0.get(),
            y - self.top_lip.philtrum.1.get(),
            right - self.top_lip.roundness.0.get(),
            y - self.top_lip.roundness.1.get(),
            right,
            y,
            right - self.bottom_lip.roundness.0.get(),
            y + self.bottom_lip.roundness.1.get(),
            left + self.bottom_lip.roundness.0.get(),
            y + self.bottom_lip.roundness.1.get(),
            left,
            y
        );

        let smile = format!(
            "M{} {} C{} {}, {} {}, {} {}",
            left,
            y,
            left + self.smile.0.get(),
            y + self.smile.1.get(),
            right - self.smile.0.get(),
            y + self.smile.1.get(),
            right,
            y
        );

        html!(
            <>
                <path id="lips" class="line" d=lip />
                <path class="line" d=smile />
            </>
        )
    }
}

use yew::prelude::*;

use crate::{
    group::Group,
    slider::Slider,
    value::{SharedPair, SharedValue, Value},
};

pub struct Brows {
    pub end_height: SharedPair,
    pub curve: SharedValue,
}

pub struct Eyes {
    pub size: SharedPair,
    pub separation: SharedValue,
    pub brows: Brows,
}

impl Eyes {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Eyes">
                <Slider name="Width" min=0.0 max=6.0 value=self.size.0.clone() value_changed=value_changed.clone() />
                <Slider name="Height" min=0.0 max=3.0 value=self.size.1.clone() value_changed=value_changed.clone() />
                <Slider name="Separation" min=0.0 max=6.0 value=self.separation.clone() value_changed=value_changed.clone() />
                <Group name="Brows">
                    <Slider name="B1" min=0.0 max=2.0 value=self.brows.end_height.0.clone() value_changed=value_changed.clone() />
                    <Slider name="B2" min=0.0 max=2.0 value=self.brows.curve.clone() value_changed=value_changed.clone() />
                    <Slider name="B3" min=0.0 max=2.0 value=self.brows.end_height.1.clone() value_changed=value_changed.clone() />
                </Group>
            </Group>
        )
    }

    fn view_one(&self, sx: Value) -> Html {
        let width = self.size.0.get();
        let separation = 0.5 * self.separation.get();

        let cx = sx * 0.5 * (self.separation.get() + width);

        let left = sx * separation;
        let right = sx * (separation + width);
        let offset = 0.5 * self.size.1.get();
        let iris_radius = 0.75 * offset;
        let pupil_radius = 0.5 * iris_radius;
        let border = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} Z",
            left,
            0.0,
            left + sx * offset,
            -offset,
            right - sx * offset,
            -offset,
            right,
            0.0,
            right - sx * offset,
            offset,
            left + sx * offset,
            offset,
            left,
            0.0
        );

        let brow = format!(
            "M {} {} Q {} {}, {} {}",
            left - sx * 0.2,
            -self.brows.end_height.0.get(),
            0.5 * (left + right),
            -self.brows.curve.get(),
            right + sx * 0.5,
            -self.brows.end_height.1.get(),
        );

        html!(
            <g>
                <path class="eye" d=border />
                <circle class="iris" cx=cx cy=0.0 r=iris_radius />
                <circle class="pupil" cx=cx cy=0.0 r=pupil_radius />
                <path class="line" d=border />
                <path class="line" d=brow />
            </g>
        )
    }

    pub fn view(&self) -> Html {
        html!(
            <>
                {self.view_one(-1.0)}
                {self.view_one(1.0)}
            </>
        )
    }
}

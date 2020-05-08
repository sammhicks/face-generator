use yew::prelude::*;

use crate::{color::ColorPicker, group::Group, slider::Slider, value::Value};

pub struct Brows {
    pub end_height: (Value, Value),
    pub curve: Value,
}

pub struct Eyes {
    pub color: String,
    pub size: (Value, Value),
    pub separation: Value,
    pub brows: Brows,
}

impl Eyes {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Eyes">
                {crate::color!("Color", self.color)}
                {crate::slider!("Width", 0.0, 6.0, self.size.0)}
                {crate::slider!("Height", 0.0, 3.0, self.size.1)}
                {crate::slider!("Separation", 0.0, 6.0, self.separation)}
                <Group name="Brows">
                    {crate::slider!("B1", 0.0, 2.0, self.brows.end_height.0)}
                    {crate::slider!("B2", 0.0, 2.0, self.brows.curve)}
                    {crate::slider!("B3", 0.0, 2.0, self.brows.end_height.1)}
                </Group>
            </Group>
        )
    }

    fn view_one(&self, sx: Value) -> Html {
        let width = self.size.0;
        let separation = 0.5 * self.separation;

        let cx = sx * 0.5 * (self.separation + width);

        let left = sx * separation;
        let right = sx * (separation + width);
        let offset = 0.5 * self.size.1;
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
            -self.brows.end_height.0,
            0.5 * (left + right),
            -self.brows.curve,
            right + sx * 0.5,
            -self.brows.end_height.1,
        );

        html!(
            <g>
                <path class="eye" d=border />
                <circle class="iris" style=format!("stroke: none; fill: {};", self.color) cx=cx cy=0.0 r=iris_radius />
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

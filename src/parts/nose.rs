use yew::prelude::*;

use crate::{group::Group, slider::Slider, value::Value};

pub struct Nose {
    pub width: Value,
    pub height: Value,
    pub bridge_gap: (Value, Value),
    pub bridge_top_curve: (Value, Value),
    pub bridge_side_curve: (Value, Value),
    pub nostril_radius: Value,
    pub tip_curve: Value,
}

impl Nose {
    pub fn controls(&mut self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Nose">
                {crate::slider!("Width", 0.0, 4.0, self.width)}
                {crate::slider!("Height", 0.0, 5.0, self.height)}
                <Group name="Bridge">
                    <Group name="Gap">
                        {crate::slider!("X", 0.0, 2.0, self.bridge_gap.0)}
                        {crate::slider!("Y", 0.0, 2.0, self.bridge_gap.1)}
                    </Group>
                    <Group name="Top Curve">
                        {crate::slider!("X", 0.0, 4.0, self.bridge_top_curve.0)}
                        {crate::slider!("Y", 0.0, 4.0, self.bridge_top_curve.1)}
                    </Group>
                    <Group name="Side Curve">
                        {crate::slider!("X", 0.0, 1.0, self.bridge_side_curve.0)}
                        {crate::slider!("Y", 0.0, 1.0, self.bridge_side_curve.1)}
                    </Group>
                    {crate::slider!("Nostril", 0.0, 0.5, self.nostril_radius)}
                    {crate::slider!("Tip", 0.0, 0.5, self.tip_curve)}
                </Group>
            </Group>
        )
    }

    pub fn view(&self) -> Html {
        let width = 0.5 * self.width;
        let height = self.height;
        let bridge_top_curve = (self.bridge_top_curve.0, self.bridge_top_curve.1);
        let bridge_side_curve = (self.bridge_side_curve.0, self.bridge_side_curve.1);
        let nostril_radius = self.nostril_radius;
        let tip_curve = self.tip_curve;

        let left_curve = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
            -0.5 * self.bridge_gap.0,
            self.bridge_gap.1,
            -self.bridge_gap.0,
            bridge_top_curve.0,
            -width,
            height - bridge_side_curve.0 - bridge_top_curve.1,
            -width,
            height - bridge_side_curve.0,
            -width,
            height - bridge_side_curve.0 + bridge_side_curve.1,
            -width / 3.0 - bridge_side_curve.1,
            height - nostril_radius + bridge_side_curve.1,
            -width / 3.0,
            height - nostril_radius,
        );

        let right_curve = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
            0.5 * self.bridge_gap.0,
            self.bridge_gap.1,
            self.bridge_gap.0,
            bridge_top_curve.0,
            width,
            height - bridge_side_curve.0 - bridge_top_curve.1,
            width,
            height - bridge_side_curve.0,
            width,
            height - bridge_side_curve.0 + bridge_side_curve.1,
            width / 3.0 + bridge_side_curve.1,
            height - nostril_radius + bridge_side_curve.1,
            width / 3.0,
            height - nostril_radius,
        );

        let bottom_curve = {
            let outer_x = 5.0 * width / 6.0;
            let inner_x = 1.0 * width / 3.0;

            let top_y = height - 2.0 * nostril_radius;
            let middle_y = height - 1.0 * nostril_radius;
            let bottom_y = height;

            format!(
                "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
                -outer_x,
                middle_y,
                -outer_x + nostril_radius,
                top_y,
                -inner_x - nostril_radius,
                top_y,
                -inner_x,
                middle_y,
                -inner_x + tip_curve,
                middle_y + tip_curve,
                -tip_curve,
                bottom_y,
                0.0,
                bottom_y,
                tip_curve,
                bottom_y,
                inner_x - tip_curve,
                middle_y + tip_curve,
                inner_x,
                middle_y,
                inner_x + nostril_radius,
                top_y,
                outer_x - nostril_radius,
                top_y,
                outer_x,
                middle_y,
            )
        };

        html!(
            <g>
                <path class="line" d=left_curve/>
                <path class="line" d=right_curve/>
                <path class="line" d=bottom_curve/>
            </g>
        )
    }
}

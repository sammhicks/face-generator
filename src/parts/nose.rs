use yew::prelude::*;

use crate::{
    group::Group,
    slider::Slider,
    value::{SharedPair, SharedValue},
};

pub struct Nose {
    pub width: SharedValue,
    pub height: SharedValue,
    pub bridge_gap: SharedPair,
    pub bridge_top_curve: SharedPair,
    pub bridge_side_curve: SharedPair,
    pub nostril_radius: SharedValue,
    pub tip_curve: SharedValue,
}

impl Nose {
    pub fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Nose">
                <Slider name="Width" min=0.0 max=4.0 value=self.width.clone() value_changed=value_changed.clone() />
                <Slider name="Height" min=0.0 max=5.0 value=self.height.clone() value_changed=value_changed.clone() />
                <Group name="Bridge">
                    <Group name="Gap">
                        <Slider name="X" min=0.0 max=2.0 value=self.bridge_gap.0.clone() value_changed=value_changed.clone() />
                        <Slider name="Y" min=0.0 max=2.0 value=self.bridge_gap.1.clone() value_changed=value_changed.clone() />
                    </Group>
                    <Group name="Top Curve">
                        <Slider name="X" min=0.0 max=4.0 value=self.bridge_top_curve.0.clone() value_changed=value_changed.clone() />
                        <Slider name="Y" min=0.0 max=4.0 value=self.bridge_top_curve.1.clone() value_changed=value_changed.clone() />
                    </Group>
                    <Group name="Side Curve">
                        <Slider name="X" min=0.0 max=1.0 value=self.bridge_side_curve.0.clone() value_changed=value_changed.clone() />
                        <Slider name="Y" min=0.0 max=1.0 value=self.bridge_side_curve.1.clone() value_changed=value_changed.clone() />
                    </Group>
                    <Slider name="Nostril" min=0.0 max=0.5 value=self.nostril_radius.clone() value_changed=value_changed.clone() />
                    <Slider name="Tip" min=0.0 max=0.5 value=self.tip_curve.clone() value_changed=value_changed.clone() />
                </Group>
            </Group>
        )
    }

    pub fn view(&self) -> Html {
        let width = 0.5 * self.width.get();
        let height = self.height.get();

        let bridge_top_curve = (self.bridge_top_curve.0.get(), self.bridge_top_curve.1.get());
        let bridge_side_curve = (
            self.bridge_side_curve.0.get(),
            self.bridge_side_curve.1.get(),
        );
        let nostril_radius = self.nostril_radius.get();
        let tip_curve = self.tip_curve.get();

        let left_curve = format!(
            "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
            -0.5 * self.bridge_gap.0.get(),
            self.bridge_gap.1.get(),
            -self.bridge_gap.0.get(),
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
            0.5 * self.bridge_gap.0.get(),
            self.bridge_gap.1.get(),
            self.bridge_gap.0.get(),
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

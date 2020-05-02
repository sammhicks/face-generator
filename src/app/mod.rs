use yew::prelude::*;

mod group;
mod slider;

use group::Group;
use slider::{SharedPair, SharedValue, Slider, Value};

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

struct Forehead {
    roundness: SharedPair,
}

impl Forehead {
    fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Forehead">
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

struct Fringe {
    thickness: SharedValue,
    roundness: SharedPair,
}

impl Fringe {
    fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Fringe">
                <Slider name="Thickness" min=0.0 max=10.0 value=self.thickness.clone() value_changed=value_changed.clone() />
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

struct Chin {
    roundness: SharedPair,
}

impl Chin {
    fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Chin">
                <Slider name="RX" min=0.0 max=7.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=10.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

struct Face {
    width: SharedValue,
    height: SharedValue,
    forehead: Forehead,
    fringe: Fringe,
    chin: Chin,
}

impl Face {
    fn controls(&self, value_changed: Callback<()>) -> Html {
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
}

struct Eyes {
    size: SharedPair,
    separation: SharedValue,
}

impl Eyes {
    fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Eyes">
                <Slider name="Width" min=0.0 max=6.0 value=self.size.0.clone() value_changed=value_changed.clone() />
                <Slider name="Height" min=0.0 max=3.0 value=self.size.1.clone() value_changed=value_changed.clone() />
                <Slider name="Separation" min=0.0 max=6.0 value=self.separation.clone() value_changed=value_changed.clone() />
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
        html!(
            <g>
                <path class="eye" d=border />
                <circle class="iris" cx=cx cy=0.0 r=iris_radius />
                <circle class="pupil" cx=cx cy=0.0 r=pupil_radius />
                <path class="line" d=border />
            </g>
        )
    }

    fn view(&self) -> Html {
        html!(
            <>
                {self.view_one(-1.0)}
                {self.view_one(1.0)}
            </>
        )
    }
}

struct Nose {
    width: SharedValue,
    height: SharedValue,
    bridge_gap: SharedPair,
    bridge_top_curve: SharedPair,
    bridge_side_curve: SharedPair,
    nostril_radius: SharedValue,
    tip_curve: SharedValue,
}

impl Nose {
    fn controls(&self, value_changed: Callback<()>) -> Html {
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

    fn view(&self) -> Html {
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

struct TopLip {
    roundness: SharedPair,
    philtrum: SharedPair,
}

impl TopLip {
    fn controls(&self, value_changed: Callback<()>) -> Html {
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

struct BottomLip {
    roundness: SharedPair,
}

impl BottomLip {
    fn controls(&self, value_changed: Callback<()>) -> Html {
        html!(
            <Group name="Bottom Lip">
                <Slider name="RX" min=0.0 max=2.0 value=self.roundness.0.clone() value_changed=value_changed.clone() />
                <Slider name="RY" min=0.0 max=2.0 value=self.roundness.1.clone() value_changed=value_changed.clone() />
            </Group>
        )
    }
}

struct Mouth {
    position: SharedValue,
    width: SharedValue,
    top_lip: TopLip,
    bottom_lip: BottomLip,
    smile: SharedPair,
}

impl Mouth {
    fn controls(&self, value_changed: Callback<()>) -> Html {
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
}

pub struct App {
    link: ComponentLink<Self>,
    grid_size: SharedValue,
    face: Face,
    eyes: Eyes,
    nose: Nose,
    mouth: Mouth,
}

pub enum Msg {
    Redraw(()),
}

type Props = ();

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            grid_size: SharedValue::new(16.0),
            face: Face {
                width: SharedValue::new(14.0),
                height: SharedValue::new(20.0),
                forehead: Forehead {
                    roundness: SharedValue::new_pair(5.0, 4.0),
                },
                fringe: Fringe {
                    thickness: SharedValue::new(4.0),
                    roundness: SharedValue::new_pair(4.0, 2.0),
                },
                chin: Chin {
                    roundness: SharedValue::new_pair(3.0, 6.0),
                },
            },
            eyes: Eyes {
                size: SharedValue::new_pair(4.0, 2.0),
                separation: SharedValue::new(2.0),
            },
            nose: Nose {
                width: SharedValue::new(3.0),
                height: SharedValue::new(4.3),
                bridge_gap: SharedValue::new_pair(1.0, 1.0),
                bridge_top_curve: SharedValue::new_pair(3.0, 0.7),
                bridge_side_curve: SharedValue::new_pair(0.8, 0.5),
                nostril_radius: SharedValue::new(0.3),
                tip_curve: SharedValue::new(0.2),
            },
            mouth: Mouth {
                position: SharedValue::new(0.65),
                width: SharedValue::new(5.0),
                top_lip: TopLip {
                    roundness: SharedValue::new_pair(1.5, 1.0),
                    philtrum: SharedValue::new_pair(0.5, 0.5),
                },
                bottom_lip: BottomLip {
                    roundness: SharedValue::new_pair(1.5, 1.5),
                },
                smile: SharedValue::new_pair(1.5, 0.5),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Redraw(()) => true,
        }
    }

    fn view(&self) -> Html {
        let border = 1.0;
        let face_rx = 0.5 * self.face.width.get();
        let face_ry = 0.5 * self.face.height.get();

        let face_centre_x = border + face_rx;
        let face_centre_y = border + face_ry;

        let hair = {
            let top = bezier_arch(face_rx, face_ry, &self.face.forehead.roundness, 1.0, -1.0);
            let bottom = bezier_arch(
                face_rx,
                face_ry - self.face.fringe.thickness.get(),
                &self.face.fringe.roundness,
                -1.0,
                -1.0,
            );

            html!(<path id="hair" class="line" d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        let face = {
            let top = bezier_arch(
                face_rx,
                face_ry - 0.5 * self.face.fringe.thickness.get(),
                &self.face.fringe.roundness,
                1.0,
                -1.0,
            );

            let bottom = bezier_arch(face_rx, face_ry, &self.face.chin.roundness, -1.0, 1.0);
            html!(<path id="face" class="line" d=format!("M {} {} {} {} Z", -face_rx, 0.0, top, bottom) />)
        };

        let mouth = {
            let y = self.mouth.position.get() * face_ry;
            let left = -0.5 * self.mouth.width.get();
            let right = 0.5 * self.mouth.width.get();
            let x = 0.0;

            let lip = format!(
                "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} Z",
                left,
                y,
                left + self.mouth.top_lip.roundness.0.get(),
                y - self.mouth.top_lip.roundness.1.get(),
                x - self.mouth.top_lip.philtrum.0.get(),
                y - self.mouth.top_lip.philtrum.1.get(),
                x,
                y - self.mouth.top_lip.philtrum.1.get(),
                x + self.mouth.top_lip.philtrum.0.get(),
                y - self.mouth.top_lip.philtrum.1.get(),
                right - self.mouth.top_lip.roundness.0.get(),
                y - self.mouth.top_lip.roundness.1.get(),
                right,
                y,
                right - self.mouth.bottom_lip.roundness.0.get(),
                y + self.mouth.bottom_lip.roundness.1.get(),
                left + self.mouth.bottom_lip.roundness.0.get(),
                y + self.mouth.bottom_lip.roundness.1.get(),
                left,
                y
            );

            let smile = format!(
                "M{} {} C{} {}, {} {}, {} {}",
                left,
                y,
                left + self.mouth.smile.0.get(),
                y + self.mouth.smile.1.get(),
                right - self.mouth.smile.0.get(),
                y + self.mouth.smile.1.get(),
                right,
                y
            );

            html!(
                <>
                    <path id="lips" class="line" d=lip />
                    <path class="line" d=smile />
                </>
            )
        };

        let canvas_width = self.grid_size.get() * (self.face.width.get() + 2.0 * border);
        let canvas_height = self.grid_size.get() * (self.face.height.get() + 2.0 * border);

        let redraw = self.link.callback(Msg::Redraw);

        html! {
            <div>
                <fieldset id="controls">
                    <legend>{"Controls"}</legend>
                    <Group name="Grid">
                        <Slider name="Size" min=8.0 max=32.0 value=self.grid_size.clone() value_changed=self.link.callback(Msg::Redraw) />
                    </Group>
                    {self.face.controls(redraw.clone())}
                    {self.eyes.controls(redraw.clone())}
                    {self.nose.controls(redraw.clone())}
                    {self.mouth.controls(redraw.clone())}
                </fieldset>
                <svg width=canvas_width height=canvas_height>
                    <g transform=format!("scale({}) translate({}, {})", self.grid_size.get(), face_centre_x, face_centre_y)>
                        {face}
                        {hair}
                        {self.eyes.view()}
                        {self.nose.view()}
                        {mouth}
                    </g>
                </svg>
            </div>
        }
    }
}

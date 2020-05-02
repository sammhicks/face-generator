use yew::prelude::*;

mod group;
mod slider;

use group::Group;
use slider::{SharedPair, SharedValue, Slider, Value};

fn bezier_arch(
    left: Value,
    start_y: Value,
    width: Value,
    height: Value,
    roundness: &SharedPair,
    sx: Value,
    sy: Value,
) -> String {
    let right = left + sx * width;

    let middle_x = 0.5 * (left + right);
    let middle_y = start_y + sy * height;

    let roundness_x = sx * roundness.0.get();
    let roundness_y = sy * roundness.1.get();

    format!(
        "C{} {}, {} {}, {} {} C{} {}, {} {}, {} {}",
        left,
        start_y + roundness_y,
        middle_x - roundness_x,
        middle_y,
        middle_x,
        middle_y,
        middle_x + roundness_x,
        middle_y,
        right,
        start_y + roundness_y,
        right,
        start_y,
    )
}

fn eye(centre: (Value, Value), size: &SharedPair) -> Html {
    let left = centre.0 - 0.5 * size.0.get();
    let right = centre.0 + 0.5 * size.0.get();
    let offset = 0.5 * size.1.get();

    let iris_radius = 0.75 * offset;

    let pupil_radius = 0.5 * iris_radius;

    let border = format!(
        "M{} {} C{} {}, {} {}, {} {} C{} {}, {} {}, {} {} Z",
        left,
        centre.1,
        left + offset,
        centre.1 - offset,
        right - offset,
        centre.1 - offset,
        right,
        centre.1,
        right - offset,
        centre.1 + offset,
        left + offset,
        centre.1 + offset,
        left,
        centre.1
    );

    html!(
        <g>
            <path class="eye" d=border />
            <circle class="iris" cx=centre.0 cy=centre.1 r=iris_radius />
            <circle class="pupil" cx=centre.0 cy=centre.1 r=pupil_radius />
            <path class="line" d=border />
        </g>
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
        let face_width = self.face.width.get();
        let face_height = self.face.height.get();

        let face_left = border;
        let face_right = face_left + face_width;
        let face_top = border;
        let face_bottom = face_top + face_height;

        let face_centre_x = 0.5 * (face_left + face_right);
        let face_centre_y = 0.5 * (face_top + face_bottom);

        let eye_offset = 0.5 * (self.eyes.separation.get() + self.eyes.size.0.get());

        let hair = {
            let width = self.face.width.get();
            let height = 0.5 * self.face.height.get();

            let top = bezier_arch(
                face_left,
                face_top + height,
                width,
                height,
                &self.face.forehead.roundness,
                1.0,
                -1.0,
            );
            let bottom = bezier_arch(
                face_left + width,
                face_top + height,
                width,
                height - self.face.fringe.thickness.get(),
                &self.face.fringe.roundness,
                -1.0,
                -1.0,
            );

            html!(<path id="hair" class="line" d=format!("M {} {} {} {} Z", face_left, face_top + height, top, bottom) />)
        };

        let face = {
            let width = self.face.width.get();
            let height = 0.5 * self.face.height.get();

            let top = bezier_arch(
                face_left,
                face_top + height,
                width,
                height - self.face.fringe.thickness.get(),
                &self.face.fringe.roundness,
                1.0,
                -1.0,
            );

            let bottom = bezier_arch(
                face_left + width,
                face_top + height,
                self.face.width.get(),
                height,
                &self.face.chin.roundness,
                -1.0,
                1.0,
            );
            html!(<path id="face" class="line" d=format!("M {} {} {} {} Z", face_left, face_top + height, top, bottom) />)
        };

        let mouth = {
            let y = 0.5 * (1.0 + self.mouth.position.get()) * face_height;
            let left = face_centre_x - 0.5 * self.mouth.width.get();
            let right = face_centre_x + 0.5 * self.mouth.width.get();
            let x = 0.5 * (left + right);

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

        let canvas_width = self.grid_size.get() * (face_right + border);
        let canvas_height = self.grid_size.get() * (face_bottom + border);

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
                    {self.mouth.controls(redraw.clone())}
                </fieldset>
                <svg width=canvas_width height=canvas_height>
                    <g transform=format!("scale({})", self.grid_size.get())>
                        {face}
                        {hair}
                        {eye((face_centre_x - eye_offset, face_centre_y), &self.eyes.size)}
                        {eye((face_centre_x + eye_offset, face_centre_y), &self.eyes.size)}
                        {mouth}
                    </g>
                </svg>
            </div>
        }
    }
}

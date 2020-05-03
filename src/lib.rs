#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod group;
mod parts;
mod slider;
mod value;

use group::Group;
use parts::*;
use slider::Slider;
use value::SharedValue;

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
                brows: Brows {
                    end_height: SharedValue::new_pair(0.5, 0.5),
                    curve: SharedValue::new(2.0),
                },
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
                        {self.face.view()}
                        {self.eyes.view()}
                        {self.nose.view()}
                        {self.mouth.view(face_ry)}
                    </g>
                </svg>
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();

    Ok(())
}

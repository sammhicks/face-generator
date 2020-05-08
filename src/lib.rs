#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use std::boxed::Box;
use std::cell::RefCell;

mod color;
mod group;
mod parts;
mod slider;
mod value;

use parts::*;

pub struct App {
    link: ComponentLink<Self>,
    head: Box<RefCell<ScaledHead>>,
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
            head: Box::from(RefCell::new(ScaledHead {
                scale: 16.0,
                head: Head {
                    face: Face {
                        skin_color: String::from("#ffdab9"),
                        width: 14.0,
                        height: 20.0,
                        forehead: Forehead {
                            roundness: (5.0, 4.0),
                        },
                        fringe: Fringe {
                            hair_color: String::from("#a52a2a"),
                            thickness: 4.0,
                            roundness: (4.0, 2.0),
                        },
                        chin: Chin {
                            roundness: (3.0, 6.0),
                        },
                    },
                    eyes: Eyes {
                        color: String::from("#add8e6"),
                        size: (4.0, 2.0),
                        separation: 2.0,
                        brows: Brows {
                            end_height: (0.5, 0.5),
                            curve: 2.0,
                        },
                    },
                    nose: Nose {
                        width: 3.0,
                        height: 4.3,
                        bridge_gap: (1.0, 1.0),
                        bridge_top_curve: (3.0, 0.7),
                        bridge_side_curve: (0.8, 0.5),
                        nostril_radius: 0.3,
                        tip_curve: 0.2,
                    },
                    mouth: Mouth {
                        color: String::from("#ff0000"),
                        position: 0.65,
                        width: 5.0,
                        top_lip: TopLip {
                            roundness: (1.5, 1.0),
                            philtrum: (0.5, 0.5),
                        },
                        bottom_lip: BottomLip {
                            roundness: (1.5, 1.5),
                        },
                        smile: (1.5, 0.5),
                    },
                },
            })),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Redraw(()) => true,
        }
    }

    fn view(&self) -> Html {
        let mut head = self.head.borrow_mut();
        let (canvas_width, canvas_height) = head.size(1.0);

        let face_centre_x = 0.5 * canvas_width;
        let face_centre_y = 0.5 * canvas_height;

        html! {
            <div>
                <fieldset id="controls">
                    <legend>{"Controls"}</legend>
                    {head.controls(self.link.callback(Msg::Redraw))}
                </fieldset>
                <svg width=canvas_width height=canvas_height>
                    <g transform=format!("translate({}, {})", face_centre_x, face_centre_y)>
                        {head.view()}
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

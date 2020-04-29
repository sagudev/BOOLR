#[derive(Clone, Copy, Debug)]
/// All rotation
pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

#[derive(Clone, Copy, Debug)]
/// Sides of component
pub enum Side {
    S0,
    S1,
    S2,
    S3,
}

#[derive(Clone, Copy, Debug)]
/// Pin of component
pub struct Pin {
    value: bool,
    side: Side,
    pos: u32,
    name: Option<&'static str>,
}

impl Pin {
    pub fn new(side: Side, pos: u32) -> Self {
        Self {
            value: false,
            side,
            pos,
            name: None,
        }
    }
    pub fn new_name(side: Side, pos: u32, name: &'static str) -> Self {
        Self {
            value: false,
            side,
            pos,
            name: Some(name),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Value,
    Char,
    Icon,
}

#[derive(Clone, Debug, Default)]
/// Properties of component
pub struct Properties {
    tip: Option<Type>,
    text: Option<&'static str>,
    frequency: Option<f32>,
    duration: Option<i32>,
    delay: Option<i32>,
    data: Option<Vec<usize>>,
    address_width: Option<i32>,
    data_width: Option<i32>,
    description: Option<String>,
}

#[derive(Clone, Debug, Default)]
/// ((width, height), (input, output), prop)
/// ((u32, u32), (Vec<Pin>, Vec<Pin>), Properties)
pub struct ElementDefault {
    width: u32,
    height: u32,
    input: Vec<Pin>,
    output: Vec<Pin>,
    prop: Properties,
}

#[derive(Clone, Copy, Debug)]
/// Elements that are avabile
pub enum Element {
    Input,
    Output,
    NOT,
    AND,
    OR,
    XOR,
    Button,
    Constant,
    Delay,
    Clock,
    Debug,
    Beep,
    Counter,
    LED,
    Display,
    //Custom,
    TimerStart,
    TimerEnd,
    ROM,
}

impl Element {
    /// get default data for element
    /// returns ((width, height), (input, output)
    pub fn get_default_data(self) -> ElementDefault {
        match self {
            Element::Input => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::Output => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::NOT => ElementDefault {
                width: 1,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Char),
                    text: Some("!"),
                    ..Default::default()
                },
            },
            Element::AND => ElementDefault {
                width: 2,
                height: 2,
                input: [Pin::new(Side::S3, 0), Pin::new(Side::S3, 1)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Char),
                    text: Some("&"),
                    ..Default::default()
                },
            },
            Element::OR => ElementDefault {
                width: 2,
                height: 2,
                input: [Pin::new(Side::S3, 0), Pin::new(Side::S3, 1)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Char),
                    text: Some("|"),
                    ..Default::default()
                },
            },
            Element::XOR => ElementDefault {
                width: 2,
                height: 2,
                input: [Pin::new(Side::S3, 0), Pin::new(Side::S3, 1)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Char),
                    text: Some("^"),
                    ..Default::default()
                },
            },
            Element::Button => ElementDefault {
                width: 2,
                height: 2,
                input: [Pin::new(Side::S3, 0), Pin::new(Side::S3, 1)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Icon),
                    text: Some("radio_button_checked"),
                    ..Default::default()
                },
            },
            Element::Constant => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::Delay => ElementDefault {
                // dialog (read the source l2067)
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Icon),
                    text: Some("timer"),
                    ..Default::default()
                },
            },
            Element::Clock => ElementDefault {
                // dialog (read the source l2303)
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Icon),
                    text: Some("access_time"),
                    ..Default::default()
                },
            },
            Element::Debug => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Icon),
                    text: Some("report_problem"),
                    ..Default::default()
                },
            },
            Element::Beep => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Icon),
                    text: Some("report_problem"),
                    frequency: Some(700.0),
                    duration: Some(200),
                    ..Default::default()
                },
            },
            Element::Counter => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::LED => ElementDefault {
                width: 1,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::Display => ElementDefault {
                width: 4,
                height: 5,
                input: [
                    Pin::new_name(Side::S0, 0, "A"),
                    Pin::new_name(Side::S0, 1, "B"),
                    Pin::new_name(Side::S0, 2, "C"),
                    Pin::new_name(Side::S0, 3, "D"),
                    Pin::new_name(Side::S2, 0, "E"),
                    Pin::new_name(Side::S2, 1, "F"),
                    Pin::new_name(Side::S2, 2, "G"),
                    Pin::new_name(Side::S2, 3, "DP"),
                ]
                .to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            /* Element::Custom => ElementDefault {
                width: 3,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            }, */
            Element::TimerStart => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::TimerEnd => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Element::ROM => ElementDefault {
                // dialog (l2951)
                width: 3,
                height: 8,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Char),
                    text: Some("ROM"),
                    ..Default::default()
                },
            },
        }
    }
}

#[derive(Clone, Debug)]
/// Component on screen
pub struct Component {
    // add value usize
    pub name: String,
    pub base: Element,
    pub pos: (i32, i32),
    pub width: u32,
    pub height: u32,
    pub input: Vec<Pin>,
    pub output: Vec<Pin>,
    pub rotation: Rotation,
}

impl Component {
    pub fn new(name: String, base: Element, pos: (i32, i32)) -> Self {
        let default = base.get_default_data();
        Self {
            name,
            base,
            pos,
            width: default.width,
            height: default.height,
            input: default.input,
            output: default.output,
            rotation: Rotation::R0,
        }
    }
    pub fn rotate(&mut self) {
        self.rotation = {
            match self.rotation.clone() {
                Rotation::R0 => Rotation::R1,
                Rotation::R1 => Rotation::R2,
                Rotation::R2 => Rotation::R3,
                Rotation::R3 => Rotation::R0,
            }
        }
    }
    pub fn draw(&self) {}
}

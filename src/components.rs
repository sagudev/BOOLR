use crate::audio::beep;
use wasm_bindgen::prelude::*;
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
    rom: Option<Vec<usize>>,
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

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
/// Elements that are avabile.
/// When you add new element you will see errors (because match will ensure that all possible cases are being handled so no wildcard (_) in element match),
/// where you need to add implementation.
/// rename to Element when https://github.com/rustwasm/wasm-bindgen/issues/1807 is solved
pub enum Elements {
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

impl Elements {
    /// get default data for element
    /// returns ((width, height), (input, output)
    pub fn get_default_data(self) -> ElementDefault {
        match self {
            Elements::Input => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::Output => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::NOT => ElementDefault {
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
            Elements::AND => ElementDefault {
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
            Elements::OR => ElementDefault {
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
            Elements::XOR => ElementDefault {
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
            Elements::Button => ElementDefault {
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
            Elements::Constant => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::Delay => ElementDefault {
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
            Elements::Clock => ElementDefault {
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
            Elements::Debug => ElementDefault {
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
            Elements::Beep => ElementDefault {
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
            Elements::Counter => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::LED => ElementDefault {
                width: 1,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::Display => ElementDefault {
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
            /* Elements::Custom => ElementDefault {
                width: 3,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            }, */
            Elements::TimerStart => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::TimerEnd => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elements::ROM => ElementDefault {
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
            // !do not add wildcard here
        }
    }
    /* pub fn rustfy(component: String) -> Self {
        component.to_lowercase();
    } */
}

#[derive(Clone, Debug)]
/// Component on screen
pub struct Component {
    pub name: String,
    pub base: Elements,
    pub pos: (i32, i32),
    pub width: u32,
    pub height: u32,
    pub input: Vec<Pin>,
    pub output: Vec<Pin>,
    pub rotation: Rotation,
    pub value: usize,
    pub prop: Properties,
}

impl Component {
    pub fn new(name: String, base: Elements, pos: (i32, i32)) -> Self {
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
            value: 0,
            prop: default.prop,
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
    /// How to draw
    pub fn draw(&mut self) {}
    /// Calculate output pin
    pub fn function(&mut self) {
        /*
        let u1: usize = 1;
        let u0: usize = 0;
        assert_eq!(true, u1 != 0);
        assert_eq!(false, u0 != 0);
        let b1: bool = true;
        let b0: bool = false;
        assert_eq!(b1 as usize, u1);
        assert_eq!(b0 as usize, u0);
        */
        match self.base {
            Elements::Input => self.output[0].value = self.value != 0,
            Elements::Output => self.value = self.input[0].value as usize,
            Elements::NOT => self.output[0].value = !self.input[0].value,
            Elements::AND => self.output[0].value = self.input[0].value && self.input[1].value,
            Elements::OR => self.output[0].value = self.input[0].value || self.input[1].value,
            Elements::XOR => self.output[0].value = self.input[0].value ^ self.input[1].value,
            Elements::Button => self.output[0].value = self.value != 0,
            Elements::Constant => self.output[0].value = self.value != 0,
            Elements::Delay => { /* No implementation */ }
            Elements::Clock => self.output[0].value = self.value != 0,
            Elements::Debug => {
                self.input[0].value = self.value != 0;
                /* notifications.push(format!("{}: {}", self.name, self.value));
                boolrConsole.log(format!("{}: {}", self.name, self.value)); */
            }
            Elements::Beep => {
                if self.input[0].value {
                    beep(self.prop.frequency, self.prop.duration);
                }
            }
            Elements::Counter => {
                if self.input[0].value {
                    self.value += 1;
                }
            }
            Elements::LED => self.value = self.input[0].value as usize,
            Elements::Display => { /* No implementation */ }
            /* Elements::Custom => { // l3114
                for input in self.input {

                }
            }, */
            Elements::TimerStart => self.output[0].value = self.value != 0,
            Elements::TimerEnd => self.value = self.input[0].value as usize,
            Elements::ROM => {
                let mut addr = 0;
                for i in 0..self.input.len() {
                    addr |= (self.input[i].value as usize) << i; // is whole rom impl right? l2960
                }
                if self.prop.rom.is_some() {
                    let content = self.prop.rom.as_ref().unwrap()[addr];
                    for i in 0..self.output.len() {
                        self.output[i].value = if (content & (1 << i)) > 0 {
                            true
                        } else {
                            false
                        };
                    }
                }
            }
            // !do not add wildcard here, rather use empty implementation
        }
    }
}

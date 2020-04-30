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
/// Elemento that are avabile.
/// When you add new element you will see errors (because match will ensure that all possible cases are being handled so no wildcard (_) in element match),
/// where you need to add implementation.
/// rename to Element when https://github.com/rustwasm/wasm-bindgen/issues/1807 is solved
pub enum Elemento {
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

impl Elemento {
    /// get default data for element
    /// returns ((width, height), (input, output)
    pub fn get_default_data(self) -> ElementDefault {
        match self {
            Elemento::Input => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::Output => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::NOT => ElementDefault {
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
            Elemento::AND => ElementDefault {
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
            Elemento::OR => ElementDefault {
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
            Elemento::XOR => ElementDefault {
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
            Elemento::Button => ElementDefault {
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
            Elemento::Constant => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::Delay => ElementDefault {
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
            Elemento::Clock => ElementDefault {
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
            Elemento::Debug => ElementDefault {
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
            Elemento::Beep => ElementDefault {
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
            Elemento::Counter => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::LED => ElementDefault {
                width: 1,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::Display => ElementDefault {
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
            /* Elemento::Custom => ElementDefault {
                width: 3,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            }, */
            Elemento::TimerStart => ElementDefault {
                width: 2,
                height: 1,
                input: [].to_vec(),
                output: [Pin::new(Side::S1, 0)].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::TimerEnd => ElementDefault {
                width: 2,
                height: 1,
                input: [Pin::new(Side::S3, 0)].to_vec(),
                output: [].to_vec(),
                prop: Properties {
                    tip: Some(Type::Value),
                    ..Default::default()
                },
            },
            Elemento::ROM => ElementDefault {
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
    pub base: Elemento,
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
    pub fn new(name: String, base: Elemento, pos: (i32, i32)) -> Self {
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
            match self.rotation {
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
            Elemento::Input => self.output[0].value = self.value != 0,
            Elemento::Output => self.value = self.input[0].value as usize,
            Elemento::NOT => self.output[0].value = !self.input[0].value,
            Elemento::AND => self.output[0].value = self.input[0].value && self.input[1].value,
            Elemento::OR => self.output[0].value = self.input[0].value || self.input[1].value,
            Elemento::XOR => self.output[0].value = self.input[0].value ^ self.input[1].value,
            Elemento::Button => self.output[0].value = self.value != 0,
            Elemento::Constant => self.output[0].value = self.value != 0,
            Elemento::Delay => { /* No implementation */ }
            Elemento::Clock => self.output[0].value = self.value != 0,
            Elemento::Debug => {
                self.input[0].value = self.value != 0;
                /* notifications.push(format!("{}: {}", self.name, self.value));
                boolrConsole.log(format!("{}: {}", self.name, self.value)); */
            }
            Elemento::Beep => {
                if self.input[0].value {
                    beep(self.prop.frequency, self.prop.duration);
                }
            }
            Elemento::Counter => {
                if self.input[0].value {
                    self.value += 1;
                }
            }
            Elemento::LED => self.value = self.input[0].value as usize,
            Elemento::Display => { /* No implementation */ }
            /* Elemento::Custom => { // l3114
                for input in self.input {

                }
            }, */
            Elemento::TimerStart => self.output[0].value = self.value != 0,
            Elemento::TimerEnd => self.value = self.input[0].value as usize,
            Elemento::ROM => {
                let mut addr = 0;
                for i in 0..self.input.len() {
                    addr |= (self.input[i].value as usize) << i; // is whole rom impl right? l2960
                }
                if self.prop.rom.is_some() {
                    let content = self.prop.rom.as_ref().unwrap()[addr];
                    for i in 0..self.output.len() {
                        self.output[i].value = (content & (1 << i)) > 0;
                    }
                }
            } // !do not add wildcard here, rather use empty implementation
        }
    }
}

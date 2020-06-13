use crate::audio::beep;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

static mut ID: usize = 0;

/// id only for component and wires
/// max size 18_446_744_073_709_551_615
pub fn generate_id() -> usize {
    unsafe {
        ID += 1;
        ID - 1
    }
}

/// id only for component and wires
/// max size 18_446_744_073_709_551_615
pub fn set_id(id: usize) {
    unsafe {
        ID = id;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
/// position
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
/// All rotation
pub enum Rotation {
    R0,
    R1,
    R2,
    R3,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
/// Sides of component
pub enum Side {
    S0,
    S1,
    S2,
    S3,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
/// color
pub struct Color(u32, u32, u32);

#[derive(Serialize, Deserialize, Clone, Debug)]
// Every wire has id, every component has id so
// hasmap that has id for key is good
// something like this: ID => Component
// but for storaging it will probably be a problem.
pub struct Global {
    pub components: HashMap<usize, Component>,
    pub wires: HashMap<usize, Wire>,
    /// When for example a custom component inside a custom component is opened, the path of custom components is saved in the following array
    pub path: Vec<String>, // not known type
}

/// functions
impl Global {
    /// Adds component to the board
    pub fn add(
        &mut self,
        component: Component,
        pos: Option<Pos>,
        force: Option<bool>,
        undoable: Option<bool>,
    ) -> bool {
        if let Some(pos) = pos {
            let x = pos.x;
            let y = pos.y;
        } else {
            let x = component.pos.x;
            let y = component.pos.y;
        }
        let force = {
            if let Some(force) = force {
                force
            } else {
                false
            }
        };
        let undoable = {
            if let Some(undoable) = undoable {
                undoable
            } else {
                false
            }
        };
        /*         if !findPortByPos(self, x, y) && !findWireByPos(self, x, y) || force {
            self.components.push(component);
            if(undoable && component.constructor == Custom && component.input.length + component.output.length == 0) {
                component.open();
            }
            if(undoable) {
                if(this != redoCaller) redoStack = [];
                undoStack.push(() => {
                    removeComponent(component);
                    redoStack.push(add.bind(redoCaller,...arguments));
                });
            }
            if(socket && sendToSocket) {
                socket.send(JSON.stringify({
                    type: "add",
                    data: stringify([component])
                }));
            }
            return true;
        } */
        false
    }
}

/// find functions
impl Global {
    /// findComponentByPos:
    /// Finds and returns component by position
    /// If no component is found, it returns undefined
    /// x = mouse.grid.x, y = mouse.grid.y
    pub fn find_component_by_pos(&self, x: i64, y: i64) -> Option<Component> {
        for (id, component) in &self.components {
            if x >= component.pos.x
                && x < component.pos.x + component.width as i64
                && y <= component.pos.y
                && y > component.pos.y - component.height as i64
            {
                return Some(component.clone());
            }
        }
        None
    }
    // findPortByPos
    // Finds and returns a port of a component by position
    // If no port is found, it returns undefined
    // x = mouse.grid.x, y = mouse.grid.y
    /* pub fn find_port_by_pos(&self, x: i64, y: i64) -> Option<Component> {
        if self.find_component_by_pos().is_none() { return None };
    for i in 0..=4 {
        const component = findComponentByPos(
            x - Math.round(Math.sin(Math.PI / 2 * i)),
            y - Math.round(Math.cos(Math.PI / 2 * i))
        );

        if(component) {
            const side = i;
            let pos;
            if(side % 2 == 0) {
                pos = x - component.pos.x;
            }  else {
                pos = component.pos.y - y;
            }

            const found = findPortByComponent(component,side,pos);
            if(found) return found;
        }
    }
    } */
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wire {
    pub from: Pin,
    pub to: Pin,
    pub color: Color,
    pub intersections: Vec<(usize, usize)>,
    pub pos: Vec<Pos>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Pin (port) of component
pub struct Pin {
    value: bool,
    side: Side,
    pos: u32,
    name: Option<String>,
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
            name: Some(name.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Type {
    Value,
    Char,
    Icon,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
/// Properties of component
pub struct Properties {
    tip: Option<Type>,
    text: Option<String>,
    frequency: Option<f32>,
    duration: Option<i32>,
    delay: Option<i32>,
    rom: Option<Vec<usize>>,
    address_width: Option<i32>,
    data_width: Option<i32>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
/// Elements that are available.
// When you add new element you will see errors (because match will ensure
//    that all possible cases are being handled so no wildcard (_) in
//    element match), where you need to add implementation.
//
// rename to Element when https://github.com/rustwasm/wasm-bindgen/issues/1807 is solved
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

/* impl std::str::FromStr for Elemento {
    type Err = String;
    fn from_str(s: &str) -> Result<Elemento, String> {
        match s {
            "input" => Ok(Elemento::Input),
            "output" => Ok(Elemento::Output),
            "not" => Ok(Elemento::NOT),
            "and" => Ok(Elemento::AND),
            "or" => Ok(Elemento::OR),
            "xor" => Ok(Elemento::XOR),
            "button" => Ok(Elemento::Button),
            "constant" => Ok(Elemento::Constant),
            "delay" => Ok(Elemento::Delay),
            "clock" => Ok(Elemento::Clock),
            "debug" => Ok(Elemento::Debug),
            "beep" => Ok(Elemento::Beep),
            "counter" => Ok(Elemento::Counter),
            "led" => Ok(Elemento::LED),
            "display" => Ok(Elemento::Display),
            //"custom" => Ok(Elemento::Custom),
            "timerstart" => Ok(Elemento::TimerStart),
            "timerend" => Ok(Elemento::TimerEnd),
            "rom" => Ok(Elemento::ROM),
            _ => Err(format!("Not found elemento: {}", s)),
        }
    }
}

#[wasm_bindgen]
impl Elemento {
    pub fn rustfy(component: String) -> Self {
        component.to_lowercase().parse::<Elemento>().unwrap()
    }
} */

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
                    text: Some("!".to_string()),
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
                    text: Some("&".to_string()),
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
                    text: Some("|".to_string()),
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
                    text: Some("^".to_string()),
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
                    text: Some("radio_button_checked".to_string()),
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
                    text: Some("timer".to_string()),
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
                    text: Some("access_time".to_string()),
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
                    text: Some("report_problem".to_string()),
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
                    text: Some("report_problem".to_string()),
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
                    text: Some("ROM".to_string()),
                    ..Default::default()
                },
            },
            // !do not add wildcard here
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Component on screen
// even pins has id
pub struct Component {
    pub name: String,
    pub base: Elemento,
    pub pos: Pos,
    pub width: u32,
    pub height: u32,
    pub input: Vec<Pin>,
    pub output: Vec<Pin>,
    pub rotation: Rotation,
    pub value: usize,
    pub prop: Properties,
}

impl Component {
    pub fn new(name: String, base: Elemento, pos: Pos) -> Self {
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

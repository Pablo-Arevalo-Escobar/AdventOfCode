use std::collections::HashMap;
use std::io::Read;

// Module implementation
#[derive(PartialEq)]
pub enum EModuleType {
    FlipFlop = 0,
    Conjunction = 1,
    Broadcast = 2,
}

pub trait OutSignal {
    fn out_signal(&mut self, input_id: &str, input_signal: bool) -> bool;

    fn get_type(&self) -> EModuleType;

    fn add_input_pin(&mut self, input: &str);

    fn get_output_list(&mut self) -> &Vec<String>;

    fn is_on(&mut self) -> bool;
}

pub struct FlipFlopModule {
    pub state: bool,
    pub output_pins: Vec<String>,
}

impl OutSignal for FlipFlopModule {
    fn out_signal(&mut self, input_id: &str, input: bool) -> bool {
        if input == false {
            self.state = !self.state;
        }

        return self.state;
    }

    fn get_type(&self) -> EModuleType {
        return EModuleType::FlipFlop;
    }

    fn add_input_pin(&mut self, input: &str) {}

    fn get_output_list(&mut self) -> &Vec<String> {
        return &self.output_pins;
    }

    fn is_on(&mut self) -> bool {
        return self.state;
    }
}

pub struct ConjunctionModule {
    pub input_pins: HashMap<String, bool>,
    pub output_pins: Vec<String>,
}

impl OutSignal for ConjunctionModule {
    fn out_signal(&mut self, input_id: &str, input: bool) -> bool {
        if let Some(map_value) = self.input_pins.get_mut(input_id) {
            *map_value = input;
        }

        let mut out = true;
        for (_id, signal) in self.input_pins.iter() {
            out = out && *signal;
        }
        return !out;
    }

    fn get_type(&self) -> EModuleType {
        return EModuleType::Conjunction;
    }

    fn add_input_pin(&mut self, input: &str) {
        self.input_pins.insert(input.to_string(), false);
    }

    fn get_output_list(&mut self) -> &Vec<String> {
        return &self.output_pins;
    }

    fn is_on(&mut self) -> bool {
        return true;
    }
}

pub struct BroadcastModule {
    pub output_pins: Vec<String>,
}

impl OutSignal for BroadcastModule {
    fn out_signal(&mut self, input_id: &str, input_signal: bool) -> bool {
        return input_signal;
    }

    fn get_type(&self) -> EModuleType {
        return EModuleType::Broadcast;
    }

    fn add_input_pin(&mut self, input: &str) {}

    fn get_output_list(&mut self) -> &Vec<String> {
        return &self.output_pins;
    }
    fn is_on(&mut self) -> bool {
        return true;
    }
}
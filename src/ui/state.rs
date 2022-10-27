use std::thread;
use std::{
    sync::Mutex,
    sync::{mpsc, Arc, Weak},
};

use gl::Gl;

use crate::processor::Processor;

use super::textfield::TextField;

#[derive(Debug, Clone)]
pub enum Component {
    TextField,
    Processor,
}

#[derive(Debug, Clone)]
pub enum OutputEvent {
    Print(String),
    InputPos(u32),
    Newline,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    UserText(String),
}

#[derive(Debug, Clone)]
pub enum Event {
    OutputEvent(OutputEvent),
    InputEvent(InputEvent),
}

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

pub trait EventActor: Send {
    fn set_event_sender(&mut self, event_sender: EventSender);
    fn set_event_receiver(&mut self, event_receiver: Mutex<EventReceiver>);
}

pub struct Events {
    event_sender: EventSender,
    event_receiver: Mutex<mpsc::Receiver<Event>>,
}

impl Events {
    pub fn new() -> Events {
        let (event_sender, event_receiver) = mpsc::channel();
        Events { event_sender, event_receiver: Mutex::new(event_receiver) }
    }

    pub fn update(&self, components: &mut Components) {
        let r = self.event_receiver.lock().unwrap().try_recv();
        if let Ok(event) = r {
            // println!("{:?}", event);

            self.handle_event(&event, components);
        };
    }

    pub fn handle_event(&self, e: &Event, components: &mut Components) {
        match e {
            Event::OutputEvent(_) => {
                for listener in components.on_print_listeners.clone().iter() {
                    listener.send(e.clone()).unwrap();
                }
            }
            Event::InputEvent(_) => {
                for listener in components.on_enter_listeners.clone().iter() {
                    listener.send(e.clone()).unwrap();
                }
            }
        }
    }
    pub fn emit(&self, e: Event) {
        self.event_sender.send(e).unwrap();
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Components {
    text_field: TextField,
    processor: Processor,
    on_print_listeners: Vec<EventSender>,
    on_enter_listeners: Vec<EventSender>,
}

impl Components {
    pub fn init(&mut self) {
        self.processor.init();
    }

    fn get_output(&mut self, component: &Component) -> Box<&mut dyn EventActor> {
        match component {
            Component::TextField => Box::new(&mut self.text_field),
            _ => panic!("No such output component: {:?}", component),
        }
    }

    fn get_input(&mut self, component: &Component) -> Box<&mut dyn EventActor> {
        match component {
            Component::Processor => Box::new(&mut self.processor),
            _ => panic!("No such input component: {:?}", component),
        }
    }
}

pub struct State {
    components: Components,
    events: Events,
}

impl State {
    pub fn new(text_field: TextField, processor: Processor) -> Self {
        State {
            components: Components {
                text_field,
                processor,
                on_print_listeners: Vec::new(),
                on_enter_listeners: Vec::new(),
            },
            events: Events::new(),
        }
    }

    pub fn run(&mut self) {
        self.components.init();
    }

    pub fn update(&mut self, event: &glutin::event::Event<()>) {
        self.components.text_field.handle_event(event);
        self.components.processor.update();
        self.events.update(&mut self.components);
    }

    pub fn render(&mut self, gl: &Gl) {
        self.components.text_field.render(gl);
    }

    pub fn on_print(&mut self, component: Component) {
        let (event_sender, event_receiver) = mpsc::channel();
        self.components.on_print_listeners.push(event_sender);

        self.components.get_output(&component).set_event_sender(self.events.event_sender.clone());
        self.components.get_output(&component).set_event_receiver(Mutex::new(event_receiver));
    }
    pub fn on_enter(&mut self, component: Component) {
        let (event_sender, event_receiver) = mpsc::channel();
        self.components.on_enter_listeners.push(event_sender);

        self.components.get_input(&component).set_event_sender(self.events.event_sender.clone());
        self.components.get_input(&component).set_event_receiver(Mutex::new(event_receiver));
    }
}

use crate::ui::state::{InputEvent, OutputEvent};

pub struct Processor {
    username: String,
    directory: String,
    pub output_event: Vec<OutputEvent>,
}

impl Processor {
    pub fn new() -> Self {
        let username = "user".to_string();
        let directory = "/".to_string();

        Processor { username, directory, output_event: vec![] }
    }

    pub fn init(&mut self) {
        // self.output_event = output_event;

        println!("Running processor");
        self.println("Welcome".to_string());
        self.print_prefix();
    }

    pub fn update(&mut self, event: InputEvent) {
        // println!("{:?}", event);

        match event {
            InputEvent::UserText(s) => {
                println!("User entered: {}", s);
                self.process(&s);
                self.send_event(OutputEvent::Newline);
                self.print_prefix();
            }
        }
    }

    fn process(&mut self, s: &str) {
        let s = s.replace(&self.prefix(), "");
        let mut input = s.split_whitespace();
        if let Some(cmd) = input.next() {
            println!("cmd: {}", cmd);
            self.send_event(OutputEvent::Newline);
            match cmd {
                "echo" => {
                    self.print(input.clone().fold(String::new(), |r, c| r + c + " "));
                }
                "ls" => {
                    self.print("bin    dev    usr".to_string());
                }
                _ => self.print(format!("Command '{cmd}' not found")),
            }
        }
    }

    fn prefix(&self) -> String {
        format!("{}:{}$ ", self.username, self.directory)
    }

    fn print_prefix(&mut self) {
        self.send_event(OutputEvent::Print(self.prefix()));
        self.send_event(OutputEvent::InputPos(self.prefix().len() as u32 + 1));
    }

    fn println(&mut self, s: String) {
        self.print(s);
        self.send_event(OutputEvent::Newline);
    }

    fn print(&mut self, s: String) {
        self.send_event(OutputEvent::Print(s));
    }

    fn send_event(&mut self, e: OutputEvent) {
        // println!("Sending {:?}", e);
        self.output_event.push(e);
        // println!("Sent {:?}", e);
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

// impl EventActor for Processor {
//     fn set_event_sender(&mut self, event_sender: EventSender) {
//         self.event_sender = Some(event_sender);
//     }

//     fn set_event_receiver(&mut self, event_receiver: Mutex<EventReceiver>) {
//         self.event_receiver = Some(event_receiver);
//     }
// }

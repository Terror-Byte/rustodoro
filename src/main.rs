use iced::widget::{button, column, text};
use iced::Alignment;
use iced::{Element, Sandbox, Settings, window};
//use std::time::{Duration, Instant};
use iced::time;

fn main() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: iced::Size::new(200.0, 200.0),
            resizable: (true),
            decorations: (true),
            ..Default::default()
        },
        ..Default::default()
    };
    Timer::run(settings)
}

// Timer prototype
//#[derive(Default)]
struct Timer {
    // start_time: Instant,
    // focus_time: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum TimerMessage {
    Start,
}

impl Sandbox for Timer {
    type Message = TimerMessage;

    fn new() -> Timer {
        // Timer { start_time: Instant::now(), focus_time: Duration::from_secs(300) }
        Timer { }
    }

    fn title(&self) -> String {
        String::from("Pomodoro Timer")
    }

    fn update(&mut self, message: TimerMessage) {
        match message {
            TimerMessage::Start => {
                //print!("Start!");
                
                // Capture current time as the starting time
                // Capture finish time (starting time + duration)
                // On update, compare current time and finish time. If we've equal or over, stop.
                // Does this have a constant update loop?
            }
        }
        
    }

    fn view(&self) -> Element<TimerMessage> {
        column![
            button("Start").on_press(TimerMessage::Start),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

// Counter Example
#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Counter {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Increment => {
                self.value += 1;
            }
            CounterMessage::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<CounterMessage> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // 'Increment' message when pressed
            button("+").on_press(CounterMessage::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // 'Decrement' message when pressed
            button("-").on_press(CounterMessage::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
use iced::theme;
use iced::widget::{button, column, container, row, text};
use iced::Application;
use iced::{keyboard, Subscription};

pub mod subscriptions;
pub mod util;

struct HidIoGui {
    count: u32,
    layer: u8,
    volume: (String, u8, String),
}

#[derive(Debug, Clone, Copy)]
enum HidState {
    Loading,
    Loaded(),
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    LayerChanged(u8),
    Volume(String, u8, String),
}

impl TryFrom<hid_client_stdout::Messages> for Message {
    type Error = ();

    fn try_from(msg: hid_client_stdout::Messages) -> Result<Self, Self::Error> {
        match msg {
            hid_client_stdout::Messages::Volume(c, v, a) => {
                use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command;
                let cmd = match c {
                    Command::Set => "Set".to_string(),
                    Command::Inc => "Inc".to_string(),
                    Command::Dec => "Dec".to_string(),
                    Command::Mute => "Mute".to_string(),
                    Command::UnMute => "UnMute".to_string(),
                    Command::ToggleMute => "ToggleMute".to_string(),
                };
                Ok(Message::Volume(cmd, v.try_into().unwrap(), a.unwrap()))
            }
            hid_client_stdout::Messages::LayerChanged(l) => {
                Ok(Message::LayerChanged(l.try_into().unwrap()))
            }
        }
    }
}

impl iced::Application for HidIoGui {
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                count: 0,
                layer: 0,
                volume: ("".to_string(), 0, "".to_string()),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("HID-IO GUI")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Increment => {
                if self.count != u32::MAX {
                    self.count += 1;
                    println!("{}", self.count);
                }
                iced::Command::none()
            }
            Message::Decrement => {
                if self.count != u32::MIN {
                    self.count -= 1;
                    println!("{}", self.count);
                }
                iced::Command::none()
            }
            Message::LayerChanged(l) => {
                println!("Layer: {}", l);
                self.layer = l;
                iced::Command::none()
            }
            Message::Volume(c, v, a) => {
                println!("Volume: {} {} {}", c, v, a);
                self.volume = (c, v, a);
                iced::Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let kb = keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("k") if modifiers.command() => Some(Message::Increment),
            keyboard::Key::Character("j") if modifiers.command() => Some(Message::Decrement),
            _ => None,
        });
        let hid = subscriptions::hidio::hid_worker();

        iced::Subscription::batch(vec![kb, hid])
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let counter = row![
            button("+")
                .on_press(Message::Increment)
                .padding(20)
                .style(theme::Button::Text),
            text(self.count.to_string()).horizontal_alignment(iced::alignment::Horizontal::Center),
            button("-")
                .on_press(Message::Decrement)
                .padding(20)
                .style(theme::Button::Destructive),
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center);

        let info = row![
            text("Command: "),
            text(self.volume.0.clone()),
            text("Volume: "),
            text(self.volume.1.to_string()),
            text("Application: "),
            text(self.volume.2.clone())
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center);

        let col = column!(counter, info).padding(20);

        let cont = container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y();

        cont.into()
    }
}

fn main() -> iced::Result {
    HidIoGui::run(iced::Settings::default())
}

use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command as VolumeCommand;
use iced::theme;
use iced::widget::{button, column, container, row, text};
use iced::Application;
use iced::{keyboard, Subscription};

pub mod subscriptions;
pub mod util;

struct HidIoGui {
    count: u32,
    layer: u8,
    volume: (Option<VolumeCommand>, u8, Option<String>),
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    Hid(hid_client_stdout::Messages),
}

// impl TryFrom<hid_client_stdout::Messages> for Message {
//     type Error = ();
//
//     fn try_from(msg: hid_client_stdout::Messages) -> Result<Self, Self::Error> {
//         match msg {
//             hid_client_stdout::Messages::Volume(c, v, a) => {
//                 use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command;
//                 let cmd = match c {
//                     Command::Set => "Set".to_string(),
//                     Command::Inc => "Inc".to_string(),
//                     Command::Dec => "Dec".to_string(),
//                     Command::Mute => "Mute".to_string(),
//                     Command::UnMute => "UnMute".to_string(),
//                     Command::ToggleMute => "ToggleMute".to_string(),
//                 };
//                 Ok(Message::Volume(cmd, v.try_into().unwrap(), a.unwrap()))
//             }
//             hid_client_stdout::Messages::LayerChanged(l) => {
//                 Ok(Message::LayerChanged(l.try_into().unwrap()))
//             }
//         }
//     }
// }

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
                volume: (None, 0, None),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("HID-IO GUI")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        println!("Update: {:?}", message);
        match message {
            Message::Increment => {
                if self.count != u32::MAX {
                    self.count += 1;
                    println!("Inc: {}", self.count);
                }
            }
            Message::Decrement => {
                if self.count != u32::MIN {
                    self.count -= 1;
                    println!("Inc: {}", self.count);
                }
            }
            Message::Hid(msg) => {
                use hid_client_stdout::Messages;
                match msg {
                    Messages::LayerChanged(l) => {
                        println!("Layer: {}", l);
                        self.layer = l.try_into().unwrap();
                    }
                    Messages::Volume(c, v, a) => {
                        println!("Volume: {:?} {} {:?}", c, v, a);
                        let app = match a {
                            Some(app) => Some(app.clone()),
                            None => None,
                        };
                        self.volume = (Some(c), v.try_into().unwrap(), app);
                    }
                }
            }
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let kb = keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("k") if modifiers.command() => Some(Message::Increment),
            keyboard::Key::Character("j") if modifiers.command() => Some(Message::Decrement),
            _ => None,
        });
        let hid = subscriptions::hidio::hid_worker().map(Message::Hid);

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

        let mut cmd_str = "None".to_string();
        if self.volume.0.is_some() {
            cmd_str = hid_client_stdout::util::str_from_command(self.volume.0.clone().unwrap())
        }

        let volume = row![
            text("Command: "),
            text(cmd_str),
            text("Volume: "),
            text(self.volume.1.to_string()),
            text("Application: "),
            text(match self.volume.2.clone() {
                Some(app) => app,
                None => "None".to_string(),
            })
        ]
        .spacing(10)
        .padding(20)
        .align_items(iced::Alignment::Center);

        let layer = row![text("Layer: "), text(self.layer),]
            .spacing(10)
            .padding(20)
            .align_items(iced::Alignment::Center);

        let col = column!(counter, volume, layer).padding(20);

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

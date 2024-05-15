use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command as VolumeCommand;
use iced::widget::{button, column, container, row, text};
use iced::{keyboard, Subscription};

pub mod subscriptions;
pub mod util;

#[derive(Default)]
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
    NAN,
}

impl HidIoGui {
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
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
            _ => {}
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
            use keyboard::Key;

            match key.as_ref() {
                Key::Character("k") => Some(Message::Increment),
                Key::Character("j") => Some(Message::Decrement),
                _ => None,
            }
        }

        let hid = subscriptions::hidio::hid_worker();

        iced::Subscription::batch(vec![hid, keyboard::on_key_press(handle_hotkey)])
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let counter = row![
            button("+")
                .on_press(Message::Increment)
                .padding(20)
                .style(button::text),
            text(self.count.to_string()).horizontal_alignment(iced::alignment::Horizontal::Center),
            button("-")
                .style(button::danger)
                .on_press(Message::Decrement)
                .padding(20)
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

        let layer = row![
            text("Layer: "),
            text(self.layer).horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(10)
        .padding(20)
        .align_items(iced::Alignment::Center);

        let col = column!(counter, volume, layer).padding(20);

        let cont = container(col)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill);

        cont.into()
    }
}

fn main() -> iced::Result {
    iced::program("HidIoGui", HidIoGui::update, HidIoGui::view)
        .subscription(HidIoGui::subscription)
        // .load(HidIoGui::load)
        .theme(HidIoGui::theme)
        .run()
}

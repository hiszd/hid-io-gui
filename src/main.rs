use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command as VolumeCommand;
use iced::widget::{button, column, container, row, text};
use iced::{keyboard, Subscription};

pub mod subscriptions;
pub mod util;

#[derive(Default, Clone)]
struct Volume {
    command: String,
    amount: String,
    app: String,
}

#[derive(Default, Clone)]
struct Strings {
    layer: String,
    volume: Volume,
}

#[derive(Default, Clone)]
struct HidIoGui {
    layer: u8,
    volume: (Option<VolumeCommand>, u8, Option<String>),
    strings: Strings,
}

#[derive(Debug, Clone)]
pub enum Message {
    Hid(hid_client_stdout::Messages),
    NAN,
}

impl HidIoGui {
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Hid(msg) => {
                use hid_client_stdout::Messages;
                match msg {
                    Messages::LayerChanged(l) => {
                        // println!("Layer: {}", l);
                        let l = l.try_into().unwrap();
                        self.layer = l;
                        self.strings.layer = l.to_string();
                    }
                    Messages::Volume(c, v, a) => {
                        // println!("Volume: {:?} {} {:?}", c, v, a);
                        let app = match a {
                            Some(app) => app.clone(),
                            None => "None".to_string(),
                        };
                        self.strings.volume = Volume {
                            command: util::pad_string(
                                hid_client_stdout::util::format_command(c),
                                10,
                                util::Direction::Center,
                            ),
                            amount: util::pad_string(v.to_string(), 3, util::Direction::Center),
                            app: util::pad_string(app, 7, util::Direction::Center),
                        };
                    }
                }
            }
            _ => {}
        }
        iced::Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // fn handle_hotkey(key: keyboard::Key, _modifiers: keyboard::Modifiers) -> Option<Message> {
        //     use keyboard::Key;
        //
        //     match key.as_ref() {
        //         Key::Character("q") => Some(Message::Increment),
        //         _ => None,
        //     }
        // }

        // let hid = subscriptions::hidio::hid_worker();

        // iced::Subscription::batch(vec![hid, keyboard::on_key_press(handle_hotkey)])
        subscriptions::hidio::hid_worker()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let strings = self.strings.clone();

        let volume = row![
            text("Command: "),
            text(strings.volume.command),
            text("Volume: "),
            text(strings.volume.amount),
            text("Application: "),
            text(strings.volume.app)
        ]
        .spacing(10)
        .padding(20)
        .align_items(iced::Alignment::Center);

        let layer = row![
            text("Layer: "),
            text(strings.layer).horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(10)
        .padding(20)
        .align_items(iced::Alignment::Center);

        let col = column!(volume, layer).padding(20);

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
        .default_font(iced::Font::with_name("FiraCode Nerd Font Mono"))
        .antialiasing(true)
        .theme(HidIoGui::theme)
        .run()
}

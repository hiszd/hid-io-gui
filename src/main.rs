use iced::theme;
use iced::widget::{button, container, row, text};
use iced::Application;
use iced::{keyboard, Subscription};

pub mod util;

struct HidIoGui {
    count: u32,
}

#[derive(Debug, Clone, Copy)]
enum HidState {
    Loading,
    Loaded(),
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl iced::Application for HidIoGui {
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { count: 0 }, iced::Command::none())
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
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let kb = keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("k") if modifiers.command() => Some(Message::Increment),
            keyboard::Key::Character("j") if modifiers.command() => Some(Message::Decrement),
            _ => None,
        });
        let hid = iced::subscription::unfold("hid", HidState::Loading, |t| {
            let strm = util::exec_stream("journalctl", vec!["-fu", "hid-io-ergoone"]);
            strm
        });
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

        let cont = container(counter)
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

use hid_client_stdout::Node;
use hid_io_client::keyboard_capnp::keyboard::signal::volume::Command as VolumeCommand;
use iced::widget::{button, column, combo_box, container, row, text};
use iced::{keyboard, Command, Subscription};

pub mod hidio;
pub mod subscriptions;
pub mod util;

#[derive(Clone)]
struct Volume {
    command: String,
    amount: String,
    app: String,
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            command: String::from("          "),
            amount: String::from("   "),
            app: String::from("       "),
        }
    }
}

#[derive(Clone)]
struct Strings {
    layer: String,
    volume: Volume,
}

impl Default for Strings {
    fn default() -> Self {
        Self {
            layer: String::from(" "),
            volume: Volume::default(),
        }
    }
}

struct HidIoGui {
    strings: Strings,
    nodes: Vec<Node>,
    combo_options: combo_box::State<String>,
    selected_node: Option<Node>,
}

impl Default for HidIoGui {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Hid(hid_client_stdout::Messages),
    NodesFound(Option<Vec<Node>>),
    NodeSelect(String),
    RefreshNodes,
    NAN,
}

impl HidIoGui {
    fn load() -> Command<Message> {
        Command::perform(hidio::fetch::fetch_nodes(), Message::NodesFound)
    }
    fn new() -> Self {
        Self {
            strings: Strings::default(),
            nodes: Vec::new(),
            combo_options: combo_box::State::new(Vec::new()),
            selected_node: None,
        }
    }

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
                            amount: util::pad_string(v.to_string(), 3, util::Direction::Right),
                            app: util::pad_string(app, 7, util::Direction::Center),
                        };
                    }
                }
            }
            Message::NodesFound(nodes) => {
                if let Some(nodes) = nodes {
                    self.nodes = nodes.clone();
                    self.combo_options =
                        combo_box::State::new(self.nodes.iter().map(|n| n.name.clone()).collect());
                    if nodes.len() == 1 {
                        self.selected_node = Some(nodes[0].clone());
                    }
                }
            }
            Message::NodeSelect(node) => {
                let sel_node = Some(self.nodes.iter().find(|n| n.name == node).unwrap().clone());
                println!("Selected: {:?}", sel_node);
                self.selected_node = sel_node;
            }
            Message::RefreshNodes => {
                return Command::perform(hidio::fetch::fetch_nodes(), Message::NodesFound);
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
        if self.selected_node.is_some() {
            subscriptions::hidio::hid_worker(Some(
                self.selected_node.as_ref().unwrap().serial.clone(),
            ))
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let strings = self.strings.clone();

        let node_select = combo_box(
            &self.combo_options,
            "Choose Node",
            match &self.selected_node {
                Some(node) => Some(&node.name),
                None => None,
            },
            Message::NodeSelect,
        )
        .width(250);

        let nodes = row![
            text("Devices: "),
            node_select,
            button("Refresh").on_press(Message::RefreshNodes)
        ]
        .align_items(iced::Alignment::Center)
        .spacing(10);

        let volume = row![
            text("Command: "),
            text(format!("\"{}\"", strings.volume.command)),
            text("Volume: "),
            text(format!("\"{}\"", strings.volume.amount)),
            text("Application: "),
            text(format!("\"{}\"", strings.volume.app)),
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

        let col = column!(nodes, volume, layer).padding(20);

        let cont = container(col)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill);

        cont.into()
    }
}

fn main() -> iced::Result {
    iced::program("HidIoGui", HidIoGui::update, HidIoGui::view)
        .subscription(HidIoGui::subscription)
        .default_font(iced::Font::with_name("FiraCode Nerd Font Mono"))
        .load(HidIoGui::load)
        .antialiasing(true)
        .theme(HidIoGui::theme)
        .run()
}

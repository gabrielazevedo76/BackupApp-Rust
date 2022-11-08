use iced::executor;
use iced::widget::{button, text_input};
use iced::{
    window, Application, Button, Column, Command, Container, Element, Length, Row, Settings, Text,
    TextInput,
};

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> iced::Result {
    Backup::run(Settings {
        window: window::Settings {
            size: (500, 500),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Backup {
    origin_path_input: text_input::State,
    origin_path_value: String,
    target_path_input: text_input::State,
    target_path_value: String,
    action_btn: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    OriginPathChanged(String),
    TargetPathChanged(String),
    ButtonPressed,
}

impl Application for Backup {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Backup - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OriginPathChanged(value) => {
                self.origin_path_value = value;
            }
            Message::TargetPathChanged(value) => {
                self.target_path_value = value;
            }
            Message::ButtonPressed => {
                //let res = get_dir_files(self.origin_path_value.to_string());
                copy_to_another_dir(&self.origin_path_value, &self.target_path_value);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let origin_path_input: Row<Message> = Row::new()
            .max_width(400)
            .align_items(iced::Alignment::Center)
            .push(Text::new("Origin Path: ").size(20))
            .push(
                TextInput::new(
                    &mut self.origin_path_input,
                    "",
                    &self.origin_path_value,
                    Message::OriginPathChanged,
                )
                .padding(10),
            );

        let target_path_input: Row<Message> = Row::new()
            .max_width(400)
            .align_items(iced::Alignment::Center)
            .push(Text::new("Target Path: ").size(20))
            .push(
                TextInput::new(
                    &mut self.target_path_input,
                    "",
                    &self.target_path_value,
                    Message::TargetPathChanged,
                )
                .padding(10),
            );

        let action_btn =
            Button::new(&mut self.action_btn, Text::new("Backup")).on_press(Message::ButtonPressed);

        let content: Column<Message> = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .push(origin_path_input)
            .push(target_path_input)
            .push(action_btn);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn get_dir_files(origin_path: &String) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let path = Path::new(&origin_path);

    for file in fs::read_dir(path).expect("Unable to list") {
        let file = file.expect("unable to get entry");

        files.push(file.path());
    }

    files
}

fn copy_to_another_dir(origin_path: &String, target_path: &String) {
    let files = get_dir_files(&origin_path);

    for file in files {
        let file_name = file.file_name();

        let new_dir = format!("{}{}", target_path, file_name.unwrap().to_string_lossy());

        let copy_file_result = fs::copy(file, new_dir);

        match copy_file_result {
            Err(error) => panic!("Problem to copy file: {:?}", error),
            Ok(file) => file,
        };
    }
}

use iced::widget::{button, center, column, text};
use iced::{system as iced_system, Element, Task};

use system::SysInfo;

pub fn main() -> iced::Result {
    iced::application("System Information - Iced", Example::update, Example::view)
        .run_with(Example::new)
}

#[derive(Default)]
#[allow(clippy::large_enum_variant)]
enum Example {
    #[default]
    Loading,
    Loaded {
        information: iced_system::Information,
    },
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
enum Message {
    InformationReceived(iced_system::Information),
    Refresh,
}

impl Example {
    fn new() -> (Self, Task<Message>) {
        (
            Self::Loading,
            iced_system::fetch_information().map(Message::InformationReceived),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Refresh => {
                let (state, refresh) = Self::new();

                *self = state;

                refresh
            }
            Message::InformationReceived(information) => {
                *self = Self::Loaded { information };

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        use bytesize::ByteSize;

        let content: Element<_> = match self {
            Example::Loading => text("Loading...").size(40).into(),
            Example::Loaded { information } => {
                let sys_info = SysInfo::new();
                let system_name = text!("System name: {}", sys_info.system_name);
                let system_version = text!("System version: {}", sys_info.system_version);
                let system_kernel = text!("System kernel: {}", sys_info.kernel);

                let hostname = text!("Hostname: {}", sys_info.hostname);

                let cpu_brand = text!("Processor brand: {}", information.cpu_brand);

                let cpu_cores = text!(
                    "Processor cores: {}",
                    information
                        .cpu_cores
                        .map_or("unknown".to_string(), |cores| cores.to_string())
                );

                let memory_readable = ByteSize::b(information.memory_total).to_string();

                let memory_total = text!(
                    "Memory (total): {} bytes ({memory_readable})",
                    information.memory_total,
                );

                let memory_text = if let Some(memory_used) = information.memory_used {
                    let memory_readable = ByteSize::b(memory_used).to_string();

                    format!("{memory_used} bytes ({memory_readable})")
                } else {
                    String::from("None")
                };

                let memory_used = text!("Memory (used): {memory_text}");

                let graphics_adapter = text!("Graphics adapter: {}", information.graphics_adapter);

                let graphics_backend = text!("Graphics backend: {}", information.graphics_backend);

                column![
                    system_name.size(30),
                    system_kernel.size(30),
                    system_version.size(30),
                    hostname.size(30),
                    cpu_brand.size(30),
                    cpu_cores.size(30),
                    memory_total.size(30),
                    memory_used.size(30),
                    graphics_adapter.size(30),
                    graphics_backend.size(30),
                    button("Refresh").on_press(Message::Refresh)
                ]
                .spacing(2)
                .into()
            }
        };

        center(content).into()
    }
}

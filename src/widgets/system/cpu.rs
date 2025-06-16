use super::super::{Widget, WidgetMessage};
use crate::theme::Theme;
use iced::{Element, widget::*};
use sysinfo::{CpuExt, System, SystemExt};

pub struct CpuMonitor {
    id: String,
    position: (f32, f32),
    size: (f32, f32),
    pinned: bool,
    sys: System,
}

impl CpuMonitor {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            size: (200.0, 100.0),
            pinned: false,
            sys: System::new_all(),
        }
    }

    fn update_usage(&mut self) {
        self.sys.refresh_cpu();
    }
}

impl Widget for CpuMonitor {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn view(&self, theme: &Theme) -> Element<WidgetMessage> {
        let mut column = Column::new().spacing(5);

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            column = column.push(Text::new(format!("CPU {}: {:.1}%", i, cpu.cpu_usage())));
        }

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    // ... rest of implementation
}

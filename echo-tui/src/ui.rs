use crate::app::App;
use ratatui::layout::Alignment;
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render(_app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(Text::raw(
            r"_______   ______  __    __    ______
|   ____| /      ||  |  |  |  /  __  \
|  |__   |  ,----'|  |__|  | |  |  |  |
|   __|  |  |     |   __   | |  |  |  |
|  |____ |  `----.|  |  |  | |  `--'  |
|_______| \______||__|  |__|  \______/

 Welcome to Echo, your neat little command line music player!

 This project is in very early development, expect bugs and shitty performance. I use this project to learn Rust and stuff along the way.
 If you have suggestions, want to provide feedback or run into a bug (which is very likely), please open an issue on GitHub at https://github.com/jaskonius/echo. ")).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ).alignment(Alignment::Center).wrap(Wrap::default()), frame.size(),
    );
}

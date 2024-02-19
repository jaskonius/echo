use ratatui::widgets::{ListItem, ListState};

pub struct StatefulList<'a> {
    pub state: ListState,
    pub items: Vec<ListItem<'a>>,
}

impl<'a> StatefulList<'a> {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            items: items.into_iter().map(ListItem::new).collect(),
        }
    }

    pub fn prev(&mut self) {
        let next = match self.state.selected() {
            None => 0,
            Some(current) => {
                if current == 0 {
                    self.items.len() - 1
                } else {
                    current - 1
                }
            }
        };
        self.state.select(Some(next));
    }

    pub fn next(&mut self) {
        let next = match self.state.selected() {
            None => 0,
            Some(current) => {
                if current >= self.items.len() - 1 {
                    0
                } else {
                    current + 1
                }
            }
        };
        self.state.select(Some(next));
    }
}

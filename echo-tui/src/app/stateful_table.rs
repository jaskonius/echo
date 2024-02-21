use ratatui::widgets::{Cell, Row, TableState};

#[derive(Clone)]
pub struct RowData {
    items: Vec<String>,
}

impl RowData {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn truncate(mut self, max_len: u16) -> Self {
        let max_len = max_len as usize;
        let ellipsis_bytes: Vec<u8> = vec![0xE2, 0x80, 0xA6]; // U+2026 -> horizontal ellipsis
        let ellipsis = std::str::from_utf8(&ellipsis_bytes).expect("could not decode ellipsis");

        self.items.iter_mut().for_each(|i| {
            // how to trim end?
            if i.len() > max_len {
                i.truncate(max_len);
                i.push_str(ellipsis);
            }
        });

        self
    }

    pub fn to_row(&self) -> Row {
        Row::new(
            self.items
                .iter()
                .map(|i| Cell::from(i.clone()))
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Clone)]
pub struct StatefulTable {
    pub state: TableState,
    pub items: Vec<RowData>,
}

impl StatefulTable {
    pub fn new(items: Vec<Vec<String>>) -> Self {
        Self {
            state: TableState::default().with_selected(Some(0)),
            items: items.iter().map(|r| RowData::new(r.clone())).collect(),
        }
    }

    pub fn truncate(mut self, max_len: u16) -> Self {
        self.items.iter_mut().for_each(|i| {
            <RowData as Clone>::clone(i).truncate(max_len); // idfk what happens but rustc told me to do it o.O
        });

        self
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

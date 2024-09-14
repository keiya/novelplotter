use std::sync::mpsc;
use std::sync::Arc;
use crate::event::Event;

pub struct Character {
    pub name: String,
    pub events: Vec<Arc<Event>>, // RcからArcへ変更
}

impl Character {
    /// 新しいCharacterを作成します。
    pub fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            events: Vec::new(),
        }
    }

    /// メッセージを送信します。
    pub fn speak(&self, message: &str, tx: mpsc::Sender<String>) {
        let output = format!("{}: {}", self.name, message);
        tx.send(output).unwrap();
    }

    /// 新しいEventを追加します。
    pub fn add_event(&mut self, description: &str) -> Arc<Event> {
        let event = Event::new(description);
        self.events.push(Arc::clone(&event));
        event
    }

    /// Eventに原因を追加します。
    pub fn add_cause(&self, event: &Arc<Event>, cause: &Arc<Event>) {
        Event::add_cause(event, cause);
    }

    /// キャラクターのイベントの因果連鎖を出力します。
    pub fn print_causal_chains(&self) {
        for event in &self.events {
            event.print_causal_chain();
        }
    }
}

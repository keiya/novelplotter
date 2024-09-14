use std::sync::{Arc, Mutex, Weak};

pub struct Event {
    pub description: String,
    pub causes: Mutex<Vec<Weak<Event>>>, // Weak<Event> に修正
}

impl Event {
    /// 新しいEventを作成し、Arcでラップして返します。
    pub fn new(description: &str) -> Arc<Self> {
        Arc::new(Event {
            description: description.to_string(),
            causes: Mutex::new(Vec::new()),
        })
    }

    /// Causeを追加します。Weak参照を使用して循環参照を防ぎます。
    pub fn add_cause(this: &Arc<Self>, cause: &Arc<Event>) {
        let mut causes = this.causes.lock().unwrap();
        causes.push(Arc::downgrade(cause));
    }

    /// 因果連鎖を出力します。インスタンスメソッドとして定義。
    pub fn print_causal_chain(&self) {
        let causes = self.causes.lock().unwrap();
        for weak_cause in causes.iter() {
            if let Some(cause) = weak_cause.upgrade() {
                println!("'{}' is caused by '{}'", self.description, cause.description);
                cause.print_causal_chain();
            }
        }
    }
}

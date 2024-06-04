#[derive(Debug)]
pub struct Input {
    pub value: Option<char>,
    pub next: Vec<Option<Input>>,
}

impl Input {
    pub fn push_trailing(&mut self, new: char, idx: usize) -> Result<(), String> {
        let new_value: Input = Input {
            value: Some(new),
            next: Vec::new(),
        };
        if (self.next.len() > idx) {
            self.next[idx].as_mut().unwrap().next.push(Some(new_value));
        } else {
            self.next.push(Some(new_value));
        }

        return Ok(());
    }
}

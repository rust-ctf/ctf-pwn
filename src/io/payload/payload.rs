use crate::io::payload::builder::{Initial, PayloadBuilder};
use crate::io::payload::write::PayloadWrite;

pub struct Payload {
    steps: Vec<PayloadStep>,
}

pub(crate) enum PayloadStep {
    Data(Vec<u8>),
    Send(),
    ReadUntil(Vec<u8>),
    ReadUntilRegex(String),
    ReadRegex(String),
    ReadLine(),
    ReadLineCrlf(),
    Print(),
}

impl Payload {
    pub fn new() -> Payload {
        Payload { steps: Vec::new() }
    }

    pub fn builder() -> PayloadBuilder<Initial> {
        PayloadBuilder::new(Payload::new())
    }

    pub fn send(&mut self) {
        self.steps.push(PayloadStep::Send());
    }

    pub fn recv_until<T: AsRef<[u8]>>(&mut self, data: T) {
        self.steps
            .push(PayloadStep::ReadUntil(data.as_ref().to_vec()));
    }

    pub fn recv_until_regex(&mut self, pattern: &str) {
        self.steps
            .push(PayloadStep::ReadUntilRegex(pattern.to_string()));
    }

    pub fn recv_regex(&mut self, pattern: &str) {
        self.steps.push(PayloadStep::ReadRegex(pattern.to_string()));
    }

    pub fn recv_line(&mut self) {
        self.steps.push(PayloadStep::ReadLine());
    }

    pub fn recv_line_crlf(&mut self) {
        self.steps.push(PayloadStep::ReadLineCrlf());
    }

    pub fn print(&mut self) {
        self.steps.push(PayloadStep::Print());
    }

    pub(crate) fn steps(&self) -> &[PayloadStep] {
        &self.steps
    }
}

impl PayloadWrite for Payload {
    fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        match self.steps.last_mut() {
            Some(PayloadStep::Data(p)) => {
                p.extend_from_slice(data.as_ref());
            }
            None | Some(_) => {
                self.steps.push(PayloadStep::Data(data.as_ref().to_vec()));
            }
        };
    }
}

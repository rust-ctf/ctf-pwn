use crate::io::Payload;
use crate::io::payload::write::PayloadWrite;

pub struct InteractivePayload {
    steps: Vec<PayloadStep>,
}

enum PayloadStep
{
    Payload(Payload),
    ReadExact(usize),
    ReadUntil(Vec<u8>),
    ReadTimeout(),
    ReadLine(),
    InteractiveShell(),
    InteractiveAnsi()
}

impl InteractivePayload
{
    pub fn new() -> Self
    {
        Self { steps: Vec::new() }
    }
}

impl PayloadWrite for InteractivePayload
{
    fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        match self.steps.last_mut()
        {
            Some(PayloadStep::Payload(p)) => {
                p.push(data);
            }
            None | Some(_) => {
                let mut payload = Payload::new();
                payload.push(data);
                self.steps.push(PayloadStep::Payload(payload));
            }
        }
    }
}

impl InteractivePayload
{
    pub fn recv_exact(&mut self, size: usize)
    {
        self.steps.push(PayloadStep::ReadExact(size))
    }

    pub fn recv_until<T:AsRef<[u8]>>(&mut self, data: T)
    {
        self.steps.push(PayloadStep::ReadUntil(data.as_ref().to_vec()))
    }

    pub fn recv_timeout(&mut self)
    {
        self.steps.push(PayloadStep::ReadTimeout())
    }

    pub fn recv_line(&mut self)
    {
        self.steps.push(PayloadStep::ReadLine())
    }

    pub fn interactive_shell(&mut self)
    {
        self.steps.push(PayloadStep::InteractiveShell())
    }

    pub fn interactive_ansi(&mut self)
    {
        self.steps.push(PayloadStep::InteractiveAnsi())
    }

    pub fn steps(&self) -> &[PayloadStep]
    {
        &self.steps
    }
}

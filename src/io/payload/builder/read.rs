use crate::io::payload::builder::PayloadBuilder;
use crate::io::payload::payloads::{Ascii, Bytes, Chain, ReadPayload, ReadPayloadType, Utf8};
use crate::io::*;

impl<T: Readable, A> PayloadBuilder<T, A> {
    fn build_payload<E>(
        self,
        read_type: ReadPayloadType,
    ) -> PayloadBuilder<Chain<T, ReadPayload<E>>, A>
    where
        ReadPayload<E>: PayloadAction,
    {
        PayloadBuilder::from(Chain::new(self.payload, ReadPayload::new(read_type)))
    }

    pub fn recvn(self, len: usize) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A>
    {
        self.build_payload(ReadPayloadType::Recvn(len))
    }
    pub fn recvn_utf8(self, len: usize) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A>
    {
        self.build_payload(ReadPayloadType::Recvn(len))
    }
    pub fn recvn_ascii(self, len: usize) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A>
    {
        self.build_payload(ReadPayloadType::Recvn(len))
    }

    pub fn recv_line(self) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A> {
        self.build_payload(ReadPayloadType::RecvLine())
    }
    pub fn recv_line_utf8(self) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A> {
        self.build_payload(ReadPayloadType::RecvLine())
    }
    pub fn recv_line_ascii(self) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A> {
        self.build_payload(ReadPayloadType::RecvLine())
    }

    pub fn recv_line_crlf(self) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A> {
        self.build_payload(ReadPayloadType::RecvLineCrlf())
    }
    pub fn recv_line_crlf_utf8(self) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A> {
        self.build_payload(ReadPayloadType::RecvLineCrlf())
    }
    pub fn recv_line_crlf_ascii(self) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A> {
        self.build_payload(ReadPayloadType::RecvLineCrlf())
    }

    pub fn recv_until<D: AsRef<[u8]>>(
        self,
        data: D,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A> {
        self.build_payload(ReadPayloadType::RecvUntil(data.as_ref().to_vec(), drop))
    }
    pub fn recv_until_utf8<D: AsRef<[u8]>>(
        self,
        data: D,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A> {
        self.build_payload(ReadPayloadType::RecvUntil(data.as_ref().to_vec(), drop))
    }
    pub fn recv_until_ascii<D: AsRef<[u8]>>(
        self,
        data: D,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A> {
        self.build_payload(ReadPayloadType::RecvUntil(data.as_ref().to_vec(), drop))
    }

    pub fn recv_until_regex(
        self,
        pattern: &str,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A> {
        self.build_payload(ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop))
    }
    pub fn recv_until_regex_utf8(
        self,
        pattern: &str,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A> {
        self.build_payload(ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop))
    }
    pub fn recv_until_regex_ascii(
        self,
        pattern: &str,
        drop: bool,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A> {
        self.build_payload(ReadPayloadType::RecvUntilRegex(pattern.to_string(), drop))
    }

    pub fn recv_regex(self, pattern: &str) -> PayloadBuilder<Chain<T, ReadPayload<Bytes>>, A> {
        self.build_payload(ReadPayloadType::RecvRegex(pattern.to_string()))
    }
    pub fn recv_regex_utf8(self, pattern: &str) -> PayloadBuilder<Chain<T, ReadPayload<Utf8>>, A> {
        self.build_payload(ReadPayloadType::RecvRegex(pattern.to_string()))
    }
    pub fn recv_regex_ascii(
        self,
        pattern: &str,
    ) -> PayloadBuilder<Chain<T, ReadPayload<Ascii>>, A> {
        self.build_payload(ReadPayloadType::RecvRegex(pattern.to_string()))
    }
}

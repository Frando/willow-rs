use std::future::Future;

use crate::encoding::error::DecodeError;
use ufotofu::local_nb::{BulkConsumer, BulkProducer};

/// A type that can be encoded to a bytestring, ensuring that any value of `Self` maps to exactly one bytestring.
///
/// [Definition](https://willowprotocol.org/specs/encodings/index.html#encodings_what)
pub trait Encodable {
    /// A function from the set `Self` to the set of bytestrings.
    ///
    /// [Definition](https://willowprotocol.org/specs/encodings/index.html#encode_s)
    fn encode<Consumer>(
        &self,
        consumer: &mut Consumer,
    ) -> impl Future<Output = Result<(), Consumer::Error>>
    where
        Consumer: BulkConsumer<Item = u8>;
}

/// A type that can be decoded from a bytestring, ensuring that every valid encoding maps to exactly one member of `Self`.
///
/// [Definition](https://willowprotocol.org/specs/encodings/index.html#encodings_what)
pub trait Decodable {
    /// A function from the set of bytestrings to the set of `T`.
    ///
    /// [Definition](https://willowprotocol.org/specs/encodings/index.html#decode_s)
    fn decode<Producer>(
        producer: &mut Producer,
    ) -> impl Future<Output = Result<Self, DecodeError<Producer::Error>>>
    where
        Producer: BulkProducer<Item = u8>,
        Self: Sized;
}

/// A type that can be used to encode `T` to a bytestring *encoded relative to `R`*.
/// This can be used to create more compact encodings from which `T` can be derived by anyone with `R`.
pub trait RelativeEncodable<R> {
    /// A function from the set `Self` to the set of bytestrings *encoded relative to `reference`*.
    fn relative_encode<Consumer>(
        &self,
        reference: &R,
        consumer: &mut Consumer,
    ) -> impl Future<Output = Result<(), Consumer::Error>>
    where
        Consumer: BulkConsumer<Item = u8>;
}

/// A type that can be used to decode `T` from a bytestring *encoded relative to `Self`*.
/// This can be used to decode a compact encoding frow which `T` can be derived by anyone with `R`.
pub trait RelativeDecodable<R> {
    /// A function from the set of bytestrings *encoded relative to `Self`* to the set of `T` in relation to `Self`.
    fn relative_decode<Producer>(
        reference: &R,
        producer: &mut Producer,
    ) -> impl Future<Output = Result<Self, DecodeError<Producer::Error>>>
    where
        Producer: BulkProducer<Item = u8>,
        Self: Sized;
}

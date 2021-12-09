pub mod proto;
pub(crate) use proto;

use std::borrow::Cow;
use binrw::{BinRead, BinWrite};

/// Call<T: BinrwMessage + Sized>
///
/// This struct informs the recipient about the number of incoming
/// messages, followed by a set of bytes which reports its size
/// then the data pertaining to that message.
#[derive(Clone, PartialEq, Default)]
pub(crate) struct Call<'a, T: BinrwMessage + Sized> (
    pub /* msg_count: */ u16,
    pub /* messages:  */ Cow<&'a [T]>,
);

#[derive(Clone, PartialEq, Default)]
pub(crate) enum BinrwMessage<T: Clone + Debug + PartialEq + Sized> {
    Connect,
    Exit,
    Highlight(T),
    Message(T),
    Update(T),
);

impl From<T: BinRead + Binwrite + Sized> for BinrwMessage {
    todo!();
}

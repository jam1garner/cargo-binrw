#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2021_idioms), allow(dead_code, unused_variables))
))]
//! [`binrw-tracing`]
//! 
//! 

/// The `EventCode` enum is a serializable representation for `Vec<u8>` hex messages.
#[rustfmt::skip]
#[derive(PartialEq, Clone, Debug, Hash)]
#[repr(u8)]
pub enum EventCode {
    /// `_Save`
    /// Tells the tracing server to dump its deserialized `EventCache` to a file
    _Save = 0x00,
    
    /// `Push`
    /// Indicates the bytes should be appended to the collection of events
    Push = 0x01,

    /// `Get`
    /// Retrieve the event at some arbitrary position
    Get = 0x02,

    /// `GetRange`
    /// Retrieve the events from some start index until some end position (inclusive range)
    GetRange = 0x03,

    /// `_Lock`
    /// Momentarily block any incoming requests with:
    /// - `_Delete`, 
    /// - `_Drain`, 
    /// - `Get`,
    /// - `GetRange`, and 
    /// - `_Lock` event codes from being served until `_Unlock` is received
    _Lock = 0x04,

    /// `_Unlock`
    /// Releases the `_Lock` to read and mutate EventCache entries
    _Unlock = 0x05,

    /// `_Delete`
    /// Drops an event at some arbitrary position from the cache
    _Delete = 0x06,

    /// `_Drain`
    /// Drop all events from the cache
    _Drain = 0x07,

    /// `_Nil`
    /// Does nothing
    _Nil = 0x08,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

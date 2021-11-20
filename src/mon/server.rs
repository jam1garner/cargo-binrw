// No clue if this is gonna be kept or not
use cfg_if::cfg_if;

use std::rc::{Rc, Weak}; // Used in RelayServer
use std::cell::RefCell;  // ditto

/// RelayServer
///
/// It bridges communication.
/// The `RelayServer` is here because we want a way to talk
/// between binrw and some editor that knows how to listen.
/// 
/// But at the same time, it needs to be told (listen for something else to tell it) to
/// send important updates to the editor (or whatever else is communicating over the protocol).
/// 
/// In effect, the purpose of the RelayServer is two-fold: it relies upon something lower-level
/// to say "Hey! Something happened in the user's binrw code. Tell their editor to do stuff with
/// it." Additionally, it needs to interact with a black-box (i.e. a state machine) that knows what
/// to do with the given updates, right?
pub(crate) struct RelayServer<T: BinrwStateMachine + Sized, U: Tracer + Sized> {
    machine: Weak<RefCell<T>>,
    tracer: Rc<RefCell<U>>
}

/// BinrwStateMachine
///
/// The brains of the operation.
/// Anything that implements `BinrwStateMachine` holds all the power to decide what gets shared,
/// what gets ignored, and what warrants causing a crash.
///
/// In the future, I think this will be the source of many, many bugs -- and will likely need to
/// be replaced with an entire crate -- but I'm not good enough for that right now.
///
///
/// Implementors of this trait should probably satisfy a couple of contracts and mechanisms -- namely:
/// (1) appropriate setup + startup behavior
/// (2) appropriate teardown + shutdown/panic responses
/// (3) a dynamic dispatch mechanism that watches for predictable, well-defined behaviors, i.e. a
/// file changed, a compilation filed, the dynamic lib checksum changed, (whatever).
/// (4) a unified function that `broadcast`'s to everything that cares (internally)
/// (5) an extension, within (4) that sends a parcel of data to high-level, user-facing sh*t -- and
/// returns a `BinResult<()>` back to the caller of said "send"/"dispatch"/etc.
/// 
/// Maybe this looks like a giant match block? I don't know yet.
trait BinrwStateMachine {
}

/// Tracer
///
/// Does the low-level watching.
/// Maybe it watches the `target` or `debug` directory? Who knows? Not me (yet).
struct Tracer {
}

///
impl RelayServer {
    pub fn spawn(host: Into<::std::net::IpAddr>, port: u16) -> Result<RelayServer, &'static str> {
        // Start a `RelayServer`.
    }
}

//! Subclients
//!
//! This module houses all the sub-clients that are used by the main client.
//! You can usually attain an instance of a sub-client by calling the relevant
//! function on the main client. See `PrivyClient` for more information.

include!(concat!(env!("OUT_DIR"), "/subclients.rs"));

mod key_quorums;
mod policies;
mod wallets;

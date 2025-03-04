/// The Extension message contains a single gNMI extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extension {
    #[prost(oneof = "extension::Ext", tags = "1, 2, 3")]
    pub ext: ::core::option::Option<extension::Ext>,
}
/// Nested message and enum types in `Extension`.
pub mod extension {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// A registered extension.
        #[prost(message, tag = "1")]
        RegisteredExt(super::RegisteredExtension),
        /// Well known extensions.
        ///
        /// Master arbitration extension.
        #[prost(message, tag = "2")]
        MasterArbitration(super::MasterArbitration),
        /// History extension.
        #[prost(message, tag = "3")]
        History(super::History),
    }
}
/// The RegisteredExtension message defines an extension which is defined outside
/// of this file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredExtension {
    /// The unique ID assigned to this extension.
    #[prost(enumeration = "ExtensionId", tag = "1")]
    pub id: i32,
    /// The binary-marshalled protobuf extension payload.
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
/// MasterArbitration is used to select the master among multiple gNMI clients
/// with the same Roles. The client with the largest election_id is honored as
/// the master.
/// The document about gNMI master arbitration can be found at
/// <https://github.com/openconfig/reference/blob/master/rpc/gnmi/gnmi-master-arbitration.md>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterArbitration {
    #[prost(message, optional, tag = "1")]
    pub role: ::core::option::Option<Role>,
    #[prost(message, optional, tag = "2")]
    pub election_id: ::core::option::Option<Uint128>,
}
/// Representation of unsigned 128-bit integer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint128 {
    #[prost(uint64, tag = "1")]
    pub high: u64,
    #[prost(uint64, tag = "2")]
    pub low: u64,
}
/// There can be one master for each role. The role is identified by its id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// More fields can be added if needed, for example, to specify what paths the
    /// role can read/write.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// The History extension allows clients to request historical data. Its
/// spec can be found at
/// <https://github.com/openconfig/reference/blob/master/rpc/gnmi/gnmi-history.md>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct History {
    #[prost(oneof = "history::Request", tags = "1, 2")]
    pub request: ::core::option::Option<history::Request>,
}
/// Nested message and enum types in `History`.
pub mod history {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Nanoseconds since the epoch
        #[prost(int64, tag = "1")]
        SnapshotTime(i64),
        #[prost(message, tag = "2")]
        Range(super::TimeRange),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeRange {
    /// Nanoseconds since the epoch
    #[prost(int64, tag = "1")]
    pub start: i64,
    /// Nanoseconds since the epoch
    #[prost(int64, tag = "2")]
    pub end: i64,
}
/// RegisteredExtension is an enumeration acting as a registry for extensions
/// defined by external sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExtensionId {
    /// New extensions are to be defined within this enumeration - their definition
    /// MUST link to a reference describing their implementation.
    EidUnset = 0,
    /// An experimental extension that may be used during prototyping of a new
    /// extension.
    EidExperimental = 999,
}
impl ExtensionId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExtensionId::EidUnset => "EID_UNSET",
            ExtensionId::EidExperimental => "EID_EXPERIMENTAL",
        }
    }
}

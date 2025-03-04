/// Notification is a re-usable message that is used to encode data from the
/// target to the client. A Notification carries two types of changes to the data
/// tree:
///   - Deleted values (delete) - a set of paths that have been removed from the
///     data tree.
///   - Updated values (update) - a set of path-value pairs indicating the path
///     whose value has changed in the data tree.
/// Reference: gNMI Specification Section 2.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// Timestamp in nanoseconds since Epoch.
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// Prefix used for paths in the message.
    #[prost(message, optional, tag = "2")]
    pub prefix: ::core::option::Option<Path>,
    /// Data elements that have changed values.
    #[prost(message, repeated, tag = "4")]
    pub update: ::prost::alloc::vec::Vec<Update>,
    /// Data elements that have been deleted.
    #[prost(message, repeated, tag = "5")]
    pub delete: ::prost::alloc::vec::Vec<Path>,
    /// This notification contains a set of paths that are always updated together
    /// referenced by a globally unique prefix.
    #[prost(bool, tag = "6")]
    pub atomic: bool,
}
/// Update is a re-usable message that is used to store a particular Path,
/// Value pair.
/// Reference: gNMI Specification Section 2.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    /// The path (key) for the update.
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<Path>,
    /// The value (value) for the update.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
    /// The explicitly typed update value.
    #[prost(message, optional, tag = "3")]
    pub val: ::core::option::Option<TypedValue>,
    /// Number of coalesced duplicates.
    #[prost(uint32, tag = "4")]
    pub duplicates: u32,
}
/// TypedValue is used to encode a value being sent between the client and
/// target (originated by either entity).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// One of the fields within the val oneof is populated with the value
    /// of the update. The type of the value being included in the Update
    /// determines which field should be populated. In the case that the
    /// encoding is a particular form of the base protobuf type, a specific
    /// field is used to store the value (e.g., json_val).
    #[prost(
        oneof = "typed_value::Value",
        tags = "1, 2, 3, 4, 5, 6, 14, 7, 8, 9, 10, 11, 12, 13"
    )]
    pub value: ::core::option::Option<typed_value::Value>,
}
/// Nested message and enum types in `TypedValue`.
pub mod typed_value {
    /// One of the fields within the val oneof is populated with the value
    /// of the update. The type of the value being included in the Update
    /// determines which field should be populated. In the case that the
    /// encoding is a particular form of the base protobuf type, a specific
    /// field is used to store the value (e.g., json_val).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String value.
        #[prost(string, tag = "1")]
        StringVal(::prost::alloc::string::String),
        /// Integer value.
        #[prost(int64, tag = "2")]
        IntVal(i64),
        /// Unsigned integer value.
        #[prost(uint64, tag = "3")]
        UintVal(u64),
        /// Bool value.
        #[prost(bool, tag = "4")]
        BoolVal(bool),
        /// Arbitrary byte sequence value.
        #[prost(bytes, tag = "5")]
        BytesVal(::prost::alloc::vec::Vec<u8>),
        /// Deprecated - use double_val.
        #[prost(float, tag = "6")]
        FloatVal(f32),
        /// Floating point value.
        #[prost(double, tag = "14")]
        DoubleVal(f64),
        /// Deprecated - use double_val.
        #[prost(message, tag = "7")]
        DecimalVal(super::Decimal64),
        /// Mixed type scalar array value.
        #[prost(message, tag = "8")]
        LeaflistVal(super::ScalarArray),
        /// protobuf.Any encoded bytes.
        #[prost(message, tag = "9")]
        AnyVal(::prost_types::Any),
        /// JSON-encoded text.
        #[prost(bytes, tag = "10")]
        JsonVal(::prost::alloc::vec::Vec<u8>),
        /// JSON-encoded text per RFC7951.
        #[prost(bytes, tag = "11")]
        JsonIetfVal(::prost::alloc::vec::Vec<u8>),
        /// Arbitrary ASCII text.
        #[prost(string, tag = "12")]
        AsciiVal(::prost::alloc::string::String),
        /// Protobuf binary encoded bytes. The message type is not included.
        /// See the specification at
        /// github.com/openconfig/reference/blob/master/rpc/gnmi/protobuf-vals.md
        /// for a complete specification. \[Experimental\]
        #[prost(bytes, tag = "13")]
        ProtoBytes(::prost::alloc::vec::Vec<u8>),
    }
}
/// Path encodes a data tree path as a series of repeated strings, with
/// each element of the path representing a data tree node name and the
/// associated attributes.
/// Reference: gNMI Specification Section 2.2.2.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    /// Elements of the path are no longer encoded as a string, but rather within
    /// the elem field as a PathElem message.
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub element: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Label to disambiguate path.
    #[prost(string, tag = "2")]
    pub origin: ::prost::alloc::string::String,
    /// Elements of the path.
    #[prost(message, repeated, tag = "3")]
    pub elem: ::prost::alloc::vec::Vec<PathElem>,
    /// The name of the target
    #[prost(string, tag = "4")]
    pub target: ::prost::alloc::string::String,
}
/// PathElem encodes an element of a gNMI path, along with any attributes (keys)
/// that may be associated with it.
/// Reference: gNMI Specification Section 2.2.2.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathElem {
    /// The name of the element in the path.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Map of key (attribute) name to value.
    #[prost(map = "string, string", tag = "2")]
    pub key: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Value encodes a data tree node's value - along with the way in which
/// the value is encoded. This message is deprecated by gNMI 0.3.0.
/// Reference: gNMI Specification Section 2.2.3.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Value of the variable being transmitted.
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// Encoding used for the value field.
    #[prost(enumeration = "Encoding", tag = "2")]
    pub r#type: i32,
}
/// Error message previously utilised to return errors to the client. Deprecated
/// in favour of using the google.golang.org/genproto/googleapis/rpc/status
/// message in the RPC response.
/// Reference: gNMI Specification Section 2.5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// Canonical gRPC error code.
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// Human readable error.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Optional additional information.
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
/// Decimal64 is used to encode a fixed precision decimal number. The value
/// is expressed as a set of digits with the precision specifying the
/// number of digits following the decimal point in the digit set.
/// This message is deprecated in favor of encoding all floating point types
/// as double precision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decimal64 {
    /// Set of digits.
    #[prost(int64, tag = "1")]
    pub digits: i64,
    /// Number of digits following the decimal point.
    #[prost(uint32, tag = "2")]
    pub precision: u32,
}
/// ScalarArray is used to encode a mixed-type array of values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarArray {
    /// The set of elements within the array. Each TypedValue message should
    /// specify only elements that have a field identifier of 1-7 (i.e., the
    /// values are scalar values).
    #[prost(message, repeated, tag = "1")]
    pub element: ::prost::alloc::vec::Vec<TypedValue>,
}
/// SubscribeRequest is the message sent by the client to the target when
/// initiating a subscription to a set of paths within the data tree. The
/// request field must be populated and the initial message must specify a
/// SubscriptionList to initiate a subscription.
/// Reference: gNMI Specification Section 3.5.1.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    /// Extension messages associated with the SubscribeRequest. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "5")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
    #[prost(oneof = "subscribe_request::Request", tags = "1, 3")]
    pub request: ::core::option::Option<subscribe_request::Request>,
}
/// Nested message and enum types in `SubscribeRequest`.
pub mod subscribe_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Specify the paths within a subscription.
        #[prost(message, tag = "1")]
        Subscribe(super::SubscriptionList),
        /// Trigger a polled update.
        #[prost(message, tag = "3")]
        Poll(super::Poll),
    }
}
/// Poll is sent within a SubscribeRequest to trigger the device to
/// send telemetry updates for the paths that are associated with the
/// subscription.
/// Reference: gNMI Specification Section Section 3.5.1.4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Poll {}
/// SubscribeResponse is the message used by the target within a Subscribe RPC.
/// The target includes a Notification message which is used to transmit values
/// of the path(s) that are associated with the subscription. The same message
/// is to indicate that the target has sent all data values once (is
/// synchronized).
/// Reference: gNMI Specification Section 3.5.1.4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {
    /// Extension messages associated with the SubscribeResponse. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "5")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
    #[prost(oneof = "subscribe_response::Response", tags = "1, 3, 4")]
    pub response: ::core::option::Option<subscribe_response::Response>,
}
/// Nested message and enum types in `SubscribeResponse`.
pub mod subscribe_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Changed or sampled value for a path.
        #[prost(message, tag = "1")]
        Update(super::Notification),
        /// Indicate target has sent all values associated with the subscription
        /// at least once.
        #[prost(bool, tag = "3")]
        SyncResponse(bool),
        /// Deprecated in favour of google.golang.org/genproto/googleapis/rpc/status
        #[prost(message, tag = "4")]
        Error(super::Error),
    }
}
/// SubscriptionList is used within a Subscribe message to specify the list of
/// paths that the client wishes to subscribe to. The message consists of a
/// list of (possibly prefixed) paths, and options that relate to the
/// subscription.
/// Reference: gNMI Specification Section 3.5.1.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionList {
    /// Prefix used for paths.
    #[prost(message, optional, tag = "1")]
    pub prefix: ::core::option::Option<Path>,
    /// Set of subscriptions to create.
    #[prost(message, repeated, tag = "2")]
    pub subscription: ::prost::alloc::vec::Vec<Subscription>,
    /// DSCP marking to be used.
    #[prost(message, optional, tag = "4")]
    pub qos: ::core::option::Option<QosMarking>,
    #[prost(enumeration = "subscription_list::Mode", tag = "5")]
    pub mode: i32,
    /// Whether elements of the schema that are marked as eligible for aggregation
    /// should be aggregated or not.
    #[prost(bool, tag = "6")]
    pub allow_aggregation: bool,
    /// The set of schemas that define the elements of the data tree that should
    /// be sent by the target.
    #[prost(message, repeated, tag = "7")]
    pub use_models: ::prost::alloc::vec::Vec<ModelData>,
    /// The encoding that the target should use within the Notifications generated
    /// corresponding to the SubscriptionList.
    #[prost(enumeration = "Encoding", tag = "8")]
    pub encoding: i32,
    /// An optional field to specify that only updates to current state should be
    /// sent to a client. If set, the initial state is not sent to the client but
    /// rather only the sync message followed by any subsequent updates to the
    /// current state. For ONCE and POLL modes, this causes the server to send only
    /// the sync message (Sec. 3.5.2.3).
    #[prost(bool, tag = "9")]
    pub updates_only: bool,
}
/// Nested message and enum types in `SubscriptionList`.
pub mod subscription_list {
    /// Mode of the subscription.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Mode {
        /// Values streamed by the target (Sec. 3.5.1.5.2).
        Stream = 0,
        /// Values sent once-off by the target (Sec. 3.5.1.5.1).
        Once = 1,
        /// Values sent in response to a poll request (Sec. 3.5.1.5.3).
        Poll = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Stream => "STREAM",
                Mode::Once => "ONCE",
                Mode::Poll => "POLL",
            }
        }
    }
}
/// Subscription is a single request within a SubscriptionList. The path
/// specified is interpreted (along with the prefix) as the elements of the data
/// tree that the client is subscribing to. The mode determines how the target
/// should trigger updates to be sent.
/// Reference: gNMI Specification Section 3.5.1.3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// The data tree path.
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<Path>,
    /// Subscription mode to be used.
    #[prost(enumeration = "SubscriptionMode", tag = "2")]
    pub mode: i32,
    /// ns between samples in SAMPLE mode.
    #[prost(uint64, tag = "3")]
    pub sample_interval: u64,
    /// Indicates whether values that have not changed should be sent in a SAMPLE
    /// subscription.
    #[prost(bool, tag = "4")]
    pub suppress_redundant: bool,
    /// Specifies the maximum allowable silent period in nanoseconds when
    /// suppress_redundant is in use. The target should send a value at least once
    /// in the period specified.
    #[prost(uint64, tag = "5")]
    pub heartbeat_interval: u64,
}
/// QOSMarking specifies the DSCP value to be set on transmitted telemetry
/// updates from the target.
/// Reference: gNMI Specification Section 3.5.1.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QosMarking {
    #[prost(uint32, tag = "1")]
    pub marking: u32,
}
/// SetRequest is sent from a client to the target to update values in the data
/// tree. Paths are either deleted by the client, or modified by means of being
/// updated, or replaced. Where a replace is used, unspecified values are
/// considered to be replaced, whereas when update is used the changes are
/// considered to be incremental. The set of changes that are specified within
/// a single SetRequest are considered to be a transaction.
/// Reference: gNMI Specification Section 3.4.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRequest {
    /// Prefix used for paths in the message.
    #[prost(message, optional, tag = "1")]
    pub prefix: ::core::option::Option<Path>,
    /// Paths to be deleted from the data tree.
    #[prost(message, repeated, tag = "2")]
    pub delete: ::prost::alloc::vec::Vec<Path>,
    /// Updates specifying elements to be replaced.
    #[prost(message, repeated, tag = "3")]
    pub replace: ::prost::alloc::vec::Vec<Update>,
    /// Updates specifying elements to updated.
    #[prost(message, repeated, tag = "4")]
    pub update: ::prost::alloc::vec::Vec<Update>,
    /// Extension messages associated with the SetRequest. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "5")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// SetResponse is the response to a SetRequest, sent from the target to the
/// client. It reports the result of the modifications to the data tree that were
/// specified by the client. Errors for this RPC should be reported using the
/// <https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>
/// message in the RPC return. The gnmi.Error message can be used to add additional
/// details where required.
/// Reference: gNMI Specification Section 3.4.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetResponse {
    /// Prefix used for paths.
    #[prost(message, optional, tag = "1")]
    pub prefix: ::core::option::Option<Path>,
    /// A set of responses specifying the result of the operations specified in
    /// the SetRequest.
    #[prost(message, repeated, tag = "2")]
    pub response: ::prost::alloc::vec::Vec<UpdateResult>,
    /// The overall status of the transaction.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub message: ::core::option::Option<Error>,
    /// Timestamp of transaction (ns since epoch).
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    /// Extension messages associated with the SetResponse. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "5")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// UpdateResult is used within the SetResponse message to communicate the
/// result of an operation specified within a SetRequest message.
/// Reference: gNMI Specification Section 3.4.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResult {
    /// Deprecated timestamp for the UpdateResult, this field has been
    /// replaced by the timestamp within the SetResponse message, since
    /// all mutations effected by a set should be applied as a single
    /// transaction.
    #[deprecated]
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// Path associated with the update.
    #[prost(message, optional, tag = "2")]
    pub path: ::core::option::Option<Path>,
    /// Status of the update operation.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub message: ::core::option::Option<Error>,
    /// Update operation type.
    #[prost(enumeration = "update_result::Operation", tag = "4")]
    pub op: i32,
}
/// Nested message and enum types in `UpdateResult`.
pub mod update_result {
    /// The operation that was associated with the Path specified.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Operation {
        Invalid = 0,
        /// The result relates to a delete of Path.
        Delete = 1,
        /// The result relates to a replace of Path.
        Replace = 2,
        /// The result relates to an update of Path.
        Update = 3,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Invalid => "INVALID",
                Operation::Delete => "DELETE",
                Operation::Replace => "REPLACE",
                Operation::Update => "UPDATE",
            }
        }
    }
}
/// GetRequest is sent when a client initiates a Get RPC. It is used to specify
/// the set of data elements for which the target should return a snapshot of
/// data. The use_models field specifies the set of schema modules that are to
/// be used by the target - where use_models is not specified then the target
/// must use all schema models that it has.
/// Reference: gNMI Specification Section 3.3.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    /// Prefix used for paths.
    #[prost(message, optional, tag = "1")]
    pub prefix: ::core::option::Option<Path>,
    /// Paths requested by the client.
    #[prost(message, repeated, tag = "2")]
    pub path: ::prost::alloc::vec::Vec<Path>,
    /// The type of data being requested.
    #[prost(enumeration = "get_request::DataType", tag = "3")]
    pub r#type: i32,
    /// Encoding to be used.
    #[prost(enumeration = "Encoding", tag = "5")]
    pub encoding: i32,
    /// The schema models to be used.
    #[prost(message, repeated, tag = "6")]
    pub use_models: ::prost::alloc::vec::Vec<ModelData>,
    /// Extension messages associated with the GetRequest. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "7")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// Nested message and enum types in `GetRequest`.
pub mod get_request {
    /// Type of elements within the data tree.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DataType {
        /// All data elements.
        All = 0,
        /// Config (rw) only elements.
        Config = 1,
        /// State (ro) only elements.
        State = 2,
        /// Data elements marked in the schema as operational. This refers to data
        /// elements whose value relates to the state of processes or interactions
        /// running on the device.
        Operational = 3,
    }
    impl DataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataType::All => "ALL",
                DataType::Config => "CONFIG",
                DataType::State => "STATE",
                DataType::Operational => "OPERATIONAL",
            }
        }
    }
}
/// GetResponse is used by the target to respond to a GetRequest from a client.
/// The set of Notifications corresponds to the data values that are requested
/// by the client in the GetRequest.
/// Reference: gNMI Specification Section 3.3.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// Data values.
    #[prost(message, repeated, tag = "1")]
    pub notification: ::prost::alloc::vec::Vec<Notification>,
    /// Errors that occurred in the Get.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
    /// Extension messages associated with the GetResponse. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "3")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// CapabilityRequest is sent by the client in the Capabilities RPC to request
/// that the target reports its capabilities.
/// Reference: gNMI Specification Section 3.2.1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRequest {
    /// Extension messages associated with the CapabilityRequest. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "1")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// CapabilityResponse is used by the target to report its capabilities to the
/// client within the Capabilities RPC.
/// Reference: gNMI Specification Section 3.2.2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityResponse {
    /// Supported schema models.
    #[prost(message, repeated, tag = "1")]
    pub supported_models: ::prost::alloc::vec::Vec<ModelData>,
    /// Supported encodings.
    #[prost(enumeration = "Encoding", repeated, tag = "2")]
    pub supported_encodings: ::prost::alloc::vec::Vec<i32>,
    /// Supported gNMI version.
    #[prost(string, tag = "3")]
    pub g_nmi_version: ::prost::alloc::string::String,
    /// Extension messages associated with the CapabilityResponse. See the
    /// gNMI extension specification for further definition.
    #[prost(message, repeated, tag = "4")]
    pub extension: ::prost::alloc::vec::Vec<super::gnmi_ext::Extension>,
}
/// ModelData is used to describe a set of schema modules. It can be used in a
/// CapabilityResponse where a target reports the set of modules that it
/// supports, and within the SubscribeRequest and GetRequest messages to specify
/// the set of models from which data tree elements should be reported.
/// Reference: gNMI Specification Section 3.2.3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelData {
    /// Name of the model.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Organization publishing the model.
    #[prost(string, tag = "2")]
    pub organization: ::prost::alloc::string::String,
    /// Semantic version of the model.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// Encoding defines the value encoding formats that are supported by the gNMI
/// protocol. These encodings are used by both the client (when sending Set
/// messages to modify the state of the target) and the target when serializing
/// data to be returned to the client (in both Subscribe and Get RPCs).
/// Reference: gNMI Specification Section 2.3
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Encoding {
    /// JSON encoded text.
    Json = 0,
    /// Arbitrarily encoded bytes.
    Bytes = 1,
    /// Encoded according to scalar values of TypedValue.
    Proto = 2,
    /// ASCII text of an out-of-band agreed format.
    Ascii = 3,
    /// JSON encoded text as per RFC7951.
    JsonIetf = 4,
}
impl Encoding {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Encoding::Json => "JSON",
            Encoding::Bytes => "BYTES",
            Encoding::Proto => "PROTO",
            Encoding::Ascii => "ASCII",
            Encoding::JsonIetf => "JSON_IETF",
        }
    }
}
/// SubscriptionMode is the mode of the subscription, specifying how the
/// target must return values in a subscription.
/// Reference: gNMI Specification Section 3.5.1.3
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionMode {
    /// The target selects the relevant mode for each element.
    TargetDefined = 0,
    /// The target sends an update on element value change.
    OnChange = 1,
    /// The target samples values according to the interval.
    Sample = 2,
}
impl SubscriptionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionMode::TargetDefined => "TARGET_DEFINED",
            SubscriptionMode::OnChange => "ON_CHANGE",
            SubscriptionMode::Sample => "SAMPLE",
        }
    }
}
/// Generated client implementations.
pub mod g_nmi_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GNmiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GNmiClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GNmiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GNmiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GNmiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Capabilities allows the client to retrieve the set of capabilities that
        /// is supported by the target. This allows the target to validate the
        /// service version that is implemented and retrieve the set of models that
        /// the target supports. The models can then be specified in subsequent RPCs
        /// to restrict the set of data that is utilized.
        /// Reference: gNMI Specification Section 3.2
        pub async fn capabilities(
            &mut self,
            request: impl tonic::IntoRequest<super::CapabilityRequest>,
        ) -> Result<tonic::Response<super::CapabilityResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gnmi.gNMI/Capabilities");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieve a snapshot of data from the target. A Get RPC requests that the
        /// target snapshots a subset of the data tree as specified by the paths
        /// included in the message and serializes this to be returned to the
        /// client using the specified encoding.
        /// Reference: gNMI Specification Section 3.3
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gnmi.gNMI/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set allows the client to modify the state of data on the target. The
        /// paths to modified along with the new values that the client wishes
        /// to set the value to.
        /// Reference: gNMI Specification Section 3.4
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gnmi.gNMI/Set");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod g_nmi_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use r2d2_redis::RedisConnectionManager;
    use redis::{Client, RedisError};
    // use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
    // use redis_async_pool::deadpool::managed::Pool;
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GNmiServer.
    #[async_trait]
    pub trait GNmi: Send + Sync + 'static {
        /// Capabilities allows the client to retrieve the set of capabilities that
        /// is supported by the target. This allows the target to validate the
        /// service version that is implemented and retrieve the set of models that
        /// the target supports. The models can then be specified in subsequent RPCs
        /// to restrict the set of data that is utilized.
        /// Reference: gNMI Specification Section 3.2
        async fn capabilities(
            &self,
            request: tonic::Request<super::CapabilityRequest>,
        ) -> Result<tonic::Response<super::CapabilityResponse>, tonic::Status>;
        /// Retrieve a snapshot of data from the target. A Get RPC requests that the
        /// target snapshots a subset of the data tree as specified by the paths
        /// included in the message and serializes this to be returned to the
        /// client using the specified encoding.
        /// Reference: gNMI Specification Section 3.3
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        /// Set allows the client to modify the state of data on the target. The
        /// paths to modified along with the new values that the client wishes
        /// to set the value to.
        /// Reference: gNMI Specification Section 3.4
        async fn set(
            &self,
            request: tonic::Request<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetResponse>, tonic::Status>;
        fn new() -> Self;
    }
    #[derive(Debug)]
    pub struct GNmiServer<T: GNmi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GNmi> GNmiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GNmiServer<T>
    where
        T: GNmi,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/gnmi.gNMI/Capabilities" => {
                    #[allow(non_camel_case_types)]
                    struct CapabilitiesSvc<T: GNmi>(pub Arc<T>);
                    impl<T: GNmi> tonic::server::UnaryService<super::CapabilityRequest>
                    for CapabilitiesSvc<T> {
                        type Response = super::CapabilityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CapabilityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).capabilities(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CapabilitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gnmi.gNMI/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: GNmi>(pub Arc<T>);
                    impl<T: GNmi> tonic::server::UnaryService<super::GetRequest>
                    for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gnmi.gNMI/Set" => {
                    #[allow(non_camel_case_types)]
                    struct SetSvc<T: GNmi>(pub Arc<T>);
                    impl<T: GNmi> tonic::server::UnaryService<super::SetRequest>
                    for SetSvc<T> {
                        type Response = super::SetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GNmi> Clone for GNmiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GNmi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GNmi> tonic::server::NamedService for GNmiServer<T> {
        const NAME: &'static str = "gnmi.gNMI";
    }
}

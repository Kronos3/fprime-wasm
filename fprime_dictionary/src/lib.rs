use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub deployment_name: String,
    pub project_version: String,
    pub library_versions: Vec<String>,
    pub dictionary_spec_version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum IntegerKind {
    F32,
    F64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FloatKind {
    F32,
    F64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TypeName {
    Integer { name: IntegerKind },
    Float { name: FloatKind },
    Bool,
    String { size: Option<u32> },
    QualifiedIdentifier { name: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumConstant {
    pub name: String,
    pub value: i128,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructMember {
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub index: u32,
    pub size: Option<u32>,
    pub format: Option<String>,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    pub qualified_name: String,
    pub size: u32,
    pub element_type: TypeName,
    pub format: Option<String>,
    pub default: serde_json::Value,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumType {
    pub qualified_name: String,
    pub representation_type: TypeName,
    pub enumerated_constants: Vec<EnumConstant>,
    pub default: serde_json::Value,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructType {
    pub qualified_name: String,
    pub members: HashMap<String, StructMember>,
    pub default: Option<serde_json::Value>,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AliasType {
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub underlying_type: TypeName,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TypeDefinition {
    Array(ArrayType),
    Enum(EnumType),
    Struct(StructType),
    Alias(AliasType),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constant {
    pub annotation: Option<String>,
    pub qualified_name: String,
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CommandKind {
    Async,
    Guarded,
    Sync,
    Set,
    Save,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum QueueFull {
    Assert,
    Block,
    Drop,
    Hook,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormalParam {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: TypeName,
    #[serde(rename = "ref")]
    pub is_ref: bool,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    #[serde(rename = "commandKind")]
    pub kind: CommandKind,
    pub opcode: i128,
    pub formal_params: Vec<FormalParam>,
    pub priority: Option<i128>,
    pub queue_full_behavior: Option<QueueFull>,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TelemetryUpdate {
    Always,
    #[serde(rename = "on change")]
    OnChange,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryChannelLimit {
    pub red: Option<serde_json::Value>,
    pub orange: Option<serde_json::Value>,
    pub yellow: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryChannelLimits {
    pub low: Vec<TelemetryChannelLimit>,
    pub high: Vec<TelemetryChannelLimit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryChannel {
    pub name: String,
    pub id: i128,
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub telemetry_update: TelemetryUpdate,
    pub format: Option<String>,
    pub limits: Option<TelemetryChannelLimits>,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct Dictionary {
    pub metadata: Metadata,
    pub type_definitions: Vec<TypeDefinition>,
    pub constants: Vec<Constant>,
    pub commands: Vec<Command>,
    pub telemetry_channels: Vec<TelemetryChannel>,
    // TODO(tumbar) Fully spec the dictionary loader
}

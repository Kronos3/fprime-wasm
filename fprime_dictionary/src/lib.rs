use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
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
    pub value: i64,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructMemberRaw {
    #[serde(rename = "type")]
    pub type_name: TypeName,
    pub index: u32,
    pub size: Option<u32>,
    pub format: Option<String>,

    pub annotation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructMember {
    pub name: String,
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

fn deserialize_struct_members<'de, D>(deserializer: D) -> Result<Vec<StructMember>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let initial_map: HashMap<String, StructMemberRaw> = HashMap::deserialize(deserializer)?;
    let mut initial: Vec<(String, StructMemberRaw)> = initial_map.into_iter().collect();
    initial.sort_by(|a, b| a.1.index.cmp(&b.1.index));

    initial
        .into_iter()
        .enumerate()
        .map(|(index, (name, member))| {
            if index != member.index as usize {
                Err(D::Error::custom(format!(
                    "Missing struct member with index {}",
                    index
                )))
            } else {
                Ok(StructMember {
                    name,
                    type_name: member.type_name,
                    index: member.index,
                    size: member.size,
                    format: member.format,
                    annotation: member.annotation,
                })
            }
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructType {
    pub qualified_name: String,
    #[serde(deserialize_with = "deserialize_struct_members")]
    pub members: Vec<StructMember>,
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
    pub opcode: u64,
    pub formal_params: Vec<FormalParam>,
    pub priority: Option<i64>,
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
    pub low: Option<TelemetryChannelLimit>,
    pub high: Option<TelemetryChannelLimit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryChannel {
    pub name: String,
    pub id: u64,
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

pub fn parse(json_file: &Path) -> Dictionary {
    match serde_json::from_str(&fs::read_to_string(json_file).expect("failed to read json file")) {
        Ok(d) => d,
        Err(err) => panic!(
            "{}:{}:{} {}",
            json_file.to_str().unwrap(),
            err.line(),
            err.column(),
            err.to_string()
        ),
    }
}

#[cfg(test)]
mod test {
    mod test;
}

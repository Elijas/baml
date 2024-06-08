use anyhow::Context;
use anyhow::Result;
use baml_types::FieldType;
use baml_types::TypeValue;
use internal_baml_jinja::types as jt;
use internal_baml_jinja::types::{OutputFormatContent, RenderOptions};
use serde::Deserialize;
use std::collections::HashMap;

pub enum OutputFormatMode {
    JsonSchema,
    TsInterface,
}

// can you model a list directly in pydantic?

// a dict is modelled as "additionalProperties" wtf?
//   - humans don't understand this, why would an LLM?

// TODO:
// - maps, unions, tuples
mod json_schema {

    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Schema {
        #[serde(rename = "$defs")]
        defs: HashMap<String, TypeDef>,

        #[serde(default)]
        properties: HashMap<String, TypeSpecWithMeta>,

        #[serde(default)]
        required: Vec<String>,

        r#type: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct TypeSpecWithMeta {
        /// Pydantic includes this by default.
        #[serde(rename = "title")]
        _title: Option<String>,

        #[serde(flatten)]
        type_spec: TypeSpec,
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    pub enum TypeSpec {
        #[serde(rename = "string")]
        Ref(TypeRef),
        Inline(TypeDef),
        Union(UnionRef),
    }

    #[derive(Debug, Deserialize)]
    pub struct UnionRef {
        #[serde(rename = "anyOf")]
        any_of: Vec<TypeSpecWithMeta>,
    }

    #[derive(Debug, Deserialize)]
    pub struct TypeRef {
        #[serde(rename = "$ref")]
        r#ref: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type")]
    pub enum TypeDef {
        #[serde(rename = "string")]
        StringOrEnum(StringOrEnumDef),

        #[serde(rename = "object")]
        Class(ClassDef),

        #[serde(rename = "array")]
        Array(Box<ArrayDef>),

        #[serde(rename = "integer")]
        Int,

        #[serde(rename = "number")]
        Float,

        #[serde(rename = "boolean")]
        Bool,

        #[serde(rename = "null")]
        Null,
    }

    #[derive(Debug, Deserialize)]
    pub struct StringOrEnumDef {
        r#enum: Option<Vec<String>>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ClassDef {
        #[serde(default)]
        properties: HashMap<String, TypeSpecWithMeta>,

        #[serde(default)]
        required: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct ArrayDef {
        items: TypeSpecWithMeta,
    }

    impl Into<OutputFormatContent> for &Schema {
        fn into(self) -> OutputFormatContent {
            let mut enums = vec![];
            let mut classes = vec![];

            for (name, type_def) in self.defs.iter() {
                match type_def {
                    TypeDef::StringOrEnum(string_or_enum_def) => {
                        if let Some(enum_values) = &string_or_enum_def.r#enum {
                            enums.push(jt::Enum {
                                name: jt::Name::new(name.clone()),
                                values: enum_values
                                    .iter()
                                    .map(|v| (jt::Name::new(v.clone()), None))
                                    .collect(),
                            });
                        }
                    }
                    TypeDef::Class(class_def) => {
                        classes.push(jt::Class {
                            name: jt::Name::new(name.clone()),
                            fields: class_def
                                .properties
                                .iter()
                                .map(|(field_name, field_type)| {
                                    (jt::Name::new(field_name.clone()), field_type.into(), None)
                                })
                                .collect(),
                        });
                    }
                    _ => {}
                }
            }
            todo!()
        }
    }

    impl Into<FieldType> for &TypeSpecWithMeta {
        fn into(self) -> FieldType {
            match &self.type_spec {
                TypeSpec::Inline(type_def) => match type_def {
                    TypeDef::StringOrEnum(StringOrEnumDef { r#enum: None }) => {
                        FieldType::Primitive(TypeValue::String)
                    }
                    TypeDef::StringOrEnum(StringOrEnumDef { r#enum: Some(_) }) => {
                        // todo
                        FieldType::Enum("".to_string())
                    }
                    TypeDef::Int => FieldType::Primitive(TypeValue::Int),
                    TypeDef::Float => FieldType::Primitive(TypeValue::Float),
                    TypeDef::Bool => FieldType::Primitive(TypeValue::Bool),
                    TypeDef::Null => FieldType::Primitive(TypeValue::Null),
                    TypeDef::Array(array_def) => {
                        FieldType::List(Box::new((&array_def.items).into()))
                    }
                    TypeDef::Class(class_def) => FieldType::Class("".to_string()),
                },
                TypeSpec::Ref(TypeRef { r#ref }) => todo!(),
                TypeSpec::Union(UnionRef { any_of }) => {
                    FieldType::Union(any_of.iter().map(|t| t.into()).collect())
                }
            }
        }
    }
}

pub fn create_output_format(
    from_schema: OutputFormatContent,
    mode: OutputFormatMode,
) -> Result<String> {
    let rendered = from_schema
        .render(RenderOptions::default())
        .context("Failed to render output format")?;
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_output_format() -> Result<()> {
        let model_json_schema = serde_json::json!({
          "$defs": {
            "Role": {
              "enum": [
                "admin",
                "user",
                "guest"
              ],
              "title": "Role",
              "type": "string"
            },
            "__main____Address": {
              "properties": {
                "street": {
                  "title": "Street",
                  "type": "string"
                },
                "city": {
                  "title": "City",
                  "type": "string"
                },
                "postal_code": {
                  "title": "Postal Code",
                  "type": "string"
                }
              },
              "required": [
                "street",
                "city",
                "postal_code"
              ],
              "title": "Address",
              "type": "object"
            },
            "other_demo__Address": {
              "properties": {
                "street": {
                  "title": "Street",
                  "type": "string"
                },
                "city": {
                  "title": "City",
                  "type": "string"
                },
                "postal_code": {
                  "title": "Postal Code",
                  "type": "string"
                }
              },
              "required": [
                "street",
                "city",
                "postal_code"
              ],
              "title": "Address",
              "type": "object"
            },
            "zebra__Address": {
              "properties": {
                "street": {
                  "title": "Street",
                  "type": "string"
                },
                "city": {
                  "title": "City",
                  "type": "string"
                },
                "postal_code": {
                  "title": "Postal Code",
                  "type": "string"
                },
                "continent": {
                  "title": "Continent",
                  "type": "string"
                }
              },
              "required": [
                "street",
                "city",
                "postal_code",
                "continent"
              ],
              "title": "Address",
              "type": "object"
            }
          },
          "properties": {
            "name": {
              "title": "Name",
              "type": "string"
            },
            "age": {
              "title": "Age",
              "type": "integer"
            },
            "roles": {
              "items": {
                "$ref": "#/$defs/Role"
              },
              "title": "Roles",
              "type": "array"
            },
            "primary_address": {
              "$ref": "#/$defs/__main____Address"
            },
            "secondary_addresses": {
              "items": {
                "$ref": "#/$defs/other_demo__Address"
              },
              "title": "Secondary Addresses",
              "type": "array"
            },
            "zebra_addresses": {
              "items": {
                "$ref": "#/$defs/zebra__Address"
              },
              "title": "Zebra Addresses",
              "type": "array"
            },
            "tertiary_address": {
              "anyOf": [
                {
                  "$ref": "#/$defs/other_demo__Address"
                },
                {
                  "items": {
                    "$ref": "#/$defs/other_demo__Address"
                  },
                  "type": "array"
                }
              ],
              "title": "Tertiary Addresses"
            },
            "gpa": {
              "title": "Gpa",
              "type": "number"
            },
            "alive": {
              "title": "Alive",
              "type": "boolean"
            },
            "nope": {
              "title": "Nope",
              "type": "null"
            },
            //"tricky": {
            //  "additionalProperties": {
            //    "type": "string"
            //  },
            //  "title": "Tricky",
            //  "type": "object"
            //}
          },
          "required": [
            "name",
            "age",
            "roles",
            "primary_address",
            "secondary_addresses",
            "zebra_addresses"
          ],
          "title": "User",
          "type": "object"
        });

        let schema = json_schema::Schema::deserialize(&model_json_schema)?;
        println!("{:#?}", schema);

        Ok(())
    }
}

use proc_macro::TokenStream;
use quote::{ quote, format_ident };
use reqwest::blocking::Client;
use openai_utils::{ BASE_URL, authorization };
use serde::Deserialize;
use convert_case::{ Case, Casing };

#[derive(Deserialize)]
struct Models {
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    id: String,
}

#[proc_macro]
pub fn generate_model_id_enum(_input: TokenStream) -> TokenStream {
    let client = Client::new();

    let response: Models = authorization!(client.get(format!("{BASE_URL}/models")))
        .send().unwrap().json().unwrap();

    let mut model_id_idents = Vec::new();
    let mut model_ids = Vec::new();
    let mut model_indexes = Vec::new();
    let mut index: u32 = 0;

    for model in response.data {
        if model.id.contains(':')
           || model.id.contains('.')
           || model.id.contains("deprecated")
        {
            continue;
        }

        model_id_idents.push(format_ident!("{}", model.id.to_case(Case::Pascal)));
        model_ids.push(model.id);
        model_indexes.push(index);

        index += 1;
    }

    quote! {
        use serde::{ Serialize, de };

        #[derive(Debug, PartialEq, Default)]
        pub enum ModelID {
            #[default]
            #(#model_id_idents),*,
            Custom(String),
        }

        impl Serialize for ModelID {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match *self {
                    #( ModelID::#model_id_idents => serializer.serialize_unit_variant("ModelID", #model_indexes, #model_ids) ),*,
                    ModelID::Custom(ref string) => serializer.serialize_str(string),
                }
            }
        }

        impl<'de> Deserialize<'de> for ModelID {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct ModelIDVisitor;

                impl<'de> de::Visitor<'de> for ModelIDVisitor {
                    type Value = ModelID;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "one of {}", "".to_owned() + #( " `" + #model_ids + "`" )+*)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            #( #model_ids => Ok(ModelID::#model_id_idents) ),*,
                            _ => Ok(ModelID::Custom(v.to_string())),
                        }
                    }
                }

                deserializer.deserialize_identifier(ModelIDVisitor)
            }
        }
    }.into()
}

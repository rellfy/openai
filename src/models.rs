//! List and describe the various models available in the API.
//! You can refer to the [Models](https://beta.openai.com/docs/models)
//! documentation to understand what models are available and the differences between them.

use serde::{ Deserialize, Serialize, de };
use reqwest::{ Client, header::AUTHORIZATION, blocking };
use crate::{ BASE_URL, get_token };

#[derive(Deserialize)]
pub struct Model {
    pub id: ModelID,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
}

impl Model {
    pub async fn new(id: ModelID) -> Result<Model, reqwest::Error> {
        let client = Client::builder().build()?;
        let token = get_token();

        client.get(format!("{BASE_URL}/models/{id}"))
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send().await?.json().await
    }
}

impl TryFrom<ModelID> for Model {
    type Error = reqwest::Error;

    fn try_from(value: ModelID) -> Result<Self, Self::Error> {
        let client = blocking::Client::builder().build()?;
        let token = get_token();

        client.get(format!("{BASE_URL}/models/{value}"))
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()?.json()
    }
}

#[derive(Deserialize)]
pub struct ModelPermission {
    pub id: String,
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

// TODO: Find a way to easily generate this enum
#[derive(Debug, PartialEq)]
pub enum ModelID {
    Babbage,
    Ada,
    Davinci,
    TextEmbeddingAda002,
    BabbageCodeSearchCode,
    TextSimilarityBabbage001,
    TextDavinci002,
    TextDavinci001,
    CurieInstructBeta,
    BabbageCodeSearchText,
    BabbageSimilarity,
    CurieSearchQuery,
    CodeSearchBabbageText001,
    CodeCushman001,
    CodeSearchBabbageCode001,
    TextAda001,
    CodeDavinci002,
    TextSimilarityAda001,
    TextDavinciInsert002,
    TextDavinci003,
    AdaCodeSearchCode,
    AdaSimilarity,
    CodeSearchAdaText001,
    TextSearchAdaQuery001,
    TextCurie001,
    TextDavinciEdit001,
    DavinciSearchDocument,
    AdaCodeSearchText,
    TextSearchAdaDoc001,
    CodeDavinciEdit001,
    DavinciInstructBeta,
    TextBabbage001,
    TextSimilarityCurie001,
    CodeSearchAdaCode001,
    AdaSearchQuery,
    TextSearchDavinciQuery001,
    CurieSimilarity,
    DavinciSearchQuery,
    TextDavinciInsert001,
    BabbageSearchDocument,
    AdaSearchDocument,
    Curie,
    TextSearchBabbageDoc001,
    TextSearchCurieDoc001,
    TextSearchCurieQuery001,
    BabbageSearchQuery,
    TextSearchDavinciDoc001,
    TextSearchBabbageQuery001,
    CurieSearchDocument,
    TextSimilarityDavinci001,
    AudioTranscribe001,
    DavinciSimilarity,
    Cushman2020_05_03,
    Ada2020_05_03,
    Babbage2020_05_03,
    Curie2020_05_03,
    Davinci2020_05_03,
    IfDavinciV2,
    IfCurieV2,
    IfDavinci3_0_0,
    DavinciIf3_0_0,
    DavinciInstructBeta2_0_0,
    // TODO: Find a way to make this &str
    Custom(String),
}

impl Serialize for ModelID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            ModelID::Babbage => serializer.serialize_unit_variant("ModelID", 0, "babbage"),
            ModelID::Ada => serializer.serialize_unit_variant("ModelID", 1, "ada"),
            ModelID::Davinci => serializer.serialize_unit_variant("ModelID", 2, "davinci"),
            ModelID::TextEmbeddingAda002 => serializer.serialize_unit_variant("ModelID", 3, "text-embedding-ada-002"),
            ModelID::BabbageCodeSearchCode => serializer.serialize_unit_variant("ModelID", 4, "babbage-code-search-code"),
            ModelID::TextSimilarityBabbage001 => serializer.serialize_unit_variant("ModelID", 5, "text-similarity-babbage-001"),
            ModelID::TextDavinci002 => serializer.serialize_unit_variant("ModelID", 6, "text-davinci-002"),
            ModelID::TextDavinci001 => serializer.serialize_unit_variant("ModelID", 7, "text-davinci-001"),
            ModelID::CurieInstructBeta => serializer.serialize_unit_variant("ModelID", 8, "curie-instruct-beta"),
            ModelID::BabbageCodeSearchText => serializer.serialize_unit_variant("ModelID", 9, "babbage-code-search-text"),
            ModelID::BabbageSimilarity => serializer.serialize_unit_variant("ModelID", 10, "babbage-similarity"),
            ModelID::CurieSearchQuery => serializer.serialize_unit_variant("ModelID", 11, "curie-search-query"),
            ModelID::CodeSearchBabbageText001 => serializer.serialize_unit_variant("ModelID", 12, "code-search-babbage-text-001"),
            ModelID::CodeCushman001 => serializer.serialize_unit_variant("ModelID", 13, "code-cushman-001"),
            ModelID::CodeSearchBabbageCode001 => serializer.serialize_unit_variant("ModelID", 14, "code-search-babbage-code-001"),
            ModelID::TextAda001 => serializer.serialize_unit_variant("ModelID", 15, "text-ada-001"),
            ModelID::CodeDavinci002 => serializer.serialize_unit_variant("ModelID", 16, "code-davinci-002"),
            ModelID::TextSimilarityAda001 => serializer.serialize_unit_variant("ModelID", 17, "text-similarity-ada-001"),
            ModelID::TextDavinciInsert002 => serializer.serialize_unit_variant("ModelID", 18, "text-davinci-insert-002"),
            ModelID::TextDavinci003 => serializer.serialize_unit_variant("ModelID", 19, "text-davinci-003"),
            ModelID::AdaCodeSearchCode => serializer.serialize_unit_variant("ModelID", 20, "ada-code-search-code"),
            ModelID::AdaSimilarity => serializer.serialize_unit_variant("ModelID", 21, "ada-similarity"),
            ModelID::CodeSearchAdaText001 => serializer.serialize_unit_variant("ModelID", 22, "code-search-ada-text-001"),
            ModelID::TextSearchAdaQuery001 => serializer.serialize_unit_variant("ModelID", 23, "text-search-ada-query-001"),
            ModelID::TextCurie001 => serializer.serialize_unit_variant("ModelID", 24, "text-curie-001"),
            ModelID::TextDavinciEdit001 => serializer.serialize_unit_variant("ModelID", 25, "text-davinci-edit-001"),
            ModelID::DavinciSearchDocument => serializer.serialize_unit_variant("ModelID", 26, "davinci-search-document"),
            ModelID::AdaCodeSearchText => serializer.serialize_unit_variant("ModelID", 27, "ada-code-search-text"),
            ModelID::TextSearchAdaDoc001 => serializer.serialize_unit_variant("ModelID", 28, "text-search-ada-doc-001"),
            ModelID::CodeDavinciEdit001 => serializer.serialize_unit_variant("ModelID", 29, "code-davinci-edit-001"),
            ModelID::DavinciInstructBeta => serializer.serialize_unit_variant("ModelID", 30, "davinci-instruct-beta"),
            ModelID::TextBabbage001 => serializer.serialize_unit_variant("ModelID", 31, "text-babbage-001"),
            ModelID::TextSimilarityCurie001 => serializer.serialize_unit_variant("ModelID", 32, "text-similarity-curie-001"),
            ModelID::CodeSearchAdaCode001 => serializer.serialize_unit_variant("ModelID", 33, "code-search-ada-code-001"),
            ModelID::AdaSearchQuery => serializer.serialize_unit_variant("ModelID", 34, "ada-search-query"),
            ModelID::TextSearchDavinciQuery001 => serializer.serialize_unit_variant("ModelID", 35, "text-search-davinci-query-001"),
            ModelID::CurieSimilarity => serializer.serialize_unit_variant("ModelID", 36, "curie-similarity"),
            ModelID::DavinciSearchQuery => serializer.serialize_unit_variant("ModelID", 37, "davinci-search-query"),
            ModelID::TextDavinciInsert001 => serializer.serialize_unit_variant("ModelID", 38, "text-davinci-insert-001"),
            ModelID::BabbageSearchDocument => serializer.serialize_unit_variant("ModelID", 39, "babbage-search-document"),
            ModelID::AdaSearchDocument => serializer.serialize_unit_variant("ModelID", 40, "ada-search-document"),
            ModelID::Curie => serializer.serialize_unit_variant("ModelID", 41, "curie"),
            ModelID::TextSearchBabbageDoc001 => serializer.serialize_unit_variant("ModelID", 42, "text-search-babbage-doc-001"),
            ModelID::TextSearchCurieDoc001 => serializer.serialize_unit_variant("ModelID", 43, "text-search-curie-doc-001"),
            ModelID::TextSearchCurieQuery001 => serializer.serialize_unit_variant("ModelID", 44, "text-search-curie-query-001"),
            ModelID::BabbageSearchQuery => serializer.serialize_unit_variant("ModelID", 45, "babbage-search-query"),
            ModelID::TextSearchDavinciDoc001 => serializer.serialize_unit_variant("ModelID", 46, "text-search-davinci-doc-001"),
            ModelID::TextSearchBabbageQuery001 => serializer.serialize_unit_variant("ModelID", 47, "text-search-babbage-query-001"),
            ModelID::CurieSearchDocument => serializer.serialize_unit_variant("ModelID", 47, "curie-search-document"),
            ModelID::TextSimilarityDavinci001 => serializer.serialize_unit_variant("ModelID", 48, "text-similarity-davinci-001"),
            ModelID::AudioTranscribe001 => serializer.serialize_unit_variant("ModelID", 49, "audio-transcribe-001"),
            ModelID::DavinciSimilarity => serializer.serialize_unit_variant("ModelID", 50, "davinci-similarity"),
            ModelID::Cushman2020_05_03 => serializer.serialize_unit_variant("ModelID", 51, "cushman:2020-05-03"),
            ModelID::Ada2020_05_03 => serializer.serialize_unit_variant("ModelID", 52, "ada:2020-05-03"),
            ModelID::Babbage2020_05_03 => serializer.serialize_unit_variant("ModelID", 53, "babbage:2020-05-03"),
            ModelID::Curie2020_05_03 => serializer.serialize_unit_variant("ModelID", 54, "curie:2020-05-03"),
            ModelID::Davinci2020_05_03 => serializer.serialize_unit_variant("ModelID", 55, "davinci:2020-05-03"),
            ModelID::IfDavinciV2 => serializer.serialize_unit_variant("ModelID", 56, "if-davinci-v2"),
            ModelID::IfCurieV2 => serializer.serialize_unit_variant("ModelID", 57, "if-curie-v2"),
            ModelID::IfDavinci3_0_0 => serializer.serialize_unit_variant("ModelID", 58, "if-davinci:3.0.0"),
            ModelID::DavinciIf3_0_0 => serializer.serialize_unit_variant("ModelID", 59, "davinci-if:3.0.0"),
            ModelID::DavinciInstructBeta2_0_0 => serializer.serialize_unit_variant("ModelID", 60, "davinci-instruct-beta:2.0.0"),
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
                formatter.write_str("`text-davinci-003`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "babbage" => Ok(ModelID::Babbage),
                    "ada" => Ok(ModelID::Ada),
                    "davinci" => Ok(ModelID::Davinci),
                    "text-embedding-ada-002" => Ok(ModelID::TextEmbeddingAda002),
                    "babbage-code-search-code" => Ok(ModelID::BabbageCodeSearchCode),
                    "text-similarity-babbage-001" => Ok(ModelID::TextSimilarityBabbage001),
                    "text-davinci-002" => Ok(ModelID::TextDavinci002),
                    "text-davinci-001" => Ok(ModelID::TextDavinci001),
                    "curie-instruct-beta" => Ok(ModelID::CurieInstructBeta),
                    "babbage-code-search-text" => Ok(ModelID::BabbageCodeSearchText),
                    "babbage-similarity" => Ok(ModelID::BabbageSimilarity),
                    "curie-search-query" => Ok(ModelID::CurieSearchQuery),
                    "code-search-babbage-text-001" => Ok(ModelID::CodeSearchBabbageText001),
                    "code-cushman-001" => Ok(ModelID::CodeCushman001),
                    "code-search-babbage-code-001" => Ok(ModelID::CodeSearchBabbageCode001),
                    "text-ada-001" => Ok(ModelID::TextAda001),
                    "code-davinci-002" => Ok(ModelID::CodeDavinci002),
                    "text-similarity-ada-001" => Ok(ModelID::TextSimilarityAda001),
                    "text-davinci-insert-002" => Ok(ModelID::TextDavinciInsert002),
                    "text-davinci-003" => Ok(ModelID::TextDavinci003),
                    "ada-code-search-code" => Ok(ModelID::AdaCodeSearchCode),
                    "ada-similarity" => Ok(ModelID::AdaSimilarity),
                    "code-search-ada-text-001" => Ok(ModelID::CodeSearchAdaText001),
                    "text-search-ada-query-001" => Ok(ModelID::TextSearchAdaQuery001),
                    "text-curie-001" => Ok(ModelID::TextCurie001),
                    "text-davinci-edit-001" => Ok(ModelID::TextDavinciEdit001),
                    "davinci-search-document" => Ok(ModelID::DavinciSearchDocument),
                    "ada-code-search-text" => Ok(ModelID::AdaCodeSearchText),
                    "text-search-ada-doc-001" => Ok(ModelID::TextSearchAdaDoc001),
                    "code-davinci-edit-001" => Ok(ModelID::CodeDavinciEdit001),
                    "davinci-instruct-beta" => Ok(ModelID::DavinciInstructBeta),
                    "text-babbage-001" => Ok(ModelID::TextBabbage001),
                    "text-similarity-curie-001" => Ok(ModelID::TextSimilarityCurie001),
                    "code-search-ada-code-001" => Ok(ModelID::CodeSearchAdaCode001),
                    "ada-search-query" => Ok(ModelID::AdaSearchQuery),
                    "text-search-davinci-query-001" => Ok(ModelID::TextSearchDavinciQuery001),
                    "curie-similarity" => Ok(ModelID::CurieSimilarity),
                    "davinci-search-query" => Ok(ModelID::DavinciSearchQuery),
                    "text-davinci-insert-001" => Ok(ModelID::TextDavinciInsert001),
                    "babbage-search-document" => Ok(ModelID::BabbageSearchDocument),
                    "ada-search-document" => Ok(ModelID::AdaSearchDocument),
                    "curie" => Ok(ModelID::Curie),
                    "text-search-babbage-doc-001" => Ok(ModelID::TextSearchBabbageDoc001),
                    "text-search-curie-doc-001" => Ok(ModelID::TextSearchCurieDoc001),
                    "text-search-curie-query-001" => Ok(ModelID::TextSearchCurieQuery001),
                    "babbage-search-query" => Ok(ModelID::BabbageSearchQuery),
                    "text-search-davinci-doc-001" => Ok(ModelID::TextSearchDavinciDoc001),
                    "text-search-babbage-query-001" => Ok(ModelID::TextSearchBabbageQuery001),
                    "curie-search-document" => Ok(ModelID::CurieSearchDocument),
                    "text-similarity-davinci-001" => Ok(ModelID::TextSimilarityDavinci001),
                    "audio-transcribe-001" => Ok(ModelID::AudioTranscribe001),
                    "davinci-similarity" => Ok(ModelID::DavinciSimilarity),
                    "cushman:2020-05-03" => Ok(ModelID::Cushman2020_05_03),
                    "ada:2020-05-03" => Ok(ModelID::Ada2020_05_03),
                    "babbage:2020-05-03" => Ok(ModelID::Babbage2020_05_03),
                    "curie:2020-05-03" => Ok(ModelID::Curie2020_05_03),
                    "davinci:2020-05-03" => Ok(ModelID::Davinci2020_05_03),
                    "if-davinci-v2" => Ok(ModelID::IfDavinciV2),
                    "if-curie-v2" => Ok(ModelID::IfCurieV2),
                    "if-davinci:3.0.0" => Ok(ModelID::IfDavinci3_0_0),
                    "davinci-if:3.0.0" => Ok(ModelID::DavinciIf3_0_0),
                    "davinci-instruct-beta:2.0.0" => Ok(ModelID::DavinciInstructBeta2_0_0),
                    _ => Ok(ModelID::Custom(v.to_string())),
                }
            }
        }

        deserializer.deserialize_identifier(ModelIDVisitor)
    }
}

impl std::fmt::Display for ModelID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let ModelID::Custom(id) = self {
            write!(f, "{id}")
        } else {
            let serialized = serde_json::to_string(self).unwrap();

            write!(f, "{}", &serialized[1..serialized.len() - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn model_id_serializes_as_expected() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::ser::to_string(&ModelID::TextDavinci003)?,
            "\"text-davinci-003\"",
        );

        assert_eq!(
            serde_json::ser::to_string(&ModelID::Custom("custom".to_string()))?,
            "\"custom\"",
        );

        Ok(())
    }

    #[test]
    fn model_id_deserializes_as_expected() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::de::from_str::<ModelID>("\"text-davinci-003\"")?,
            ModelID::TextDavinci003,
        );

        assert_eq!(
            serde_json::de::from_str::<ModelID>("\"custom\"")?,
            ModelID::Custom("custom".to_string()),
        );

        Ok(())
    }

    #[test]
    fn can_get_model() {
        dotenv().expect("should load .env file");

        let model = Model::try_from(ModelID::TextDavinci003)
            .expect("should return model");

        assert_eq!(
                model.permission.first()
                .expect("should have at least one permission object").created,
            1673644124,
        )
    }

    #[test]
    fn can_get_custom_model() {
        dotenv().expect("should load .env file");

        let model = Model::try_from(
            ModelID::Custom("davinci:ft-personal-2022-12-12-04-49-51".to_string())
        )
            .expect("should return model");

        assert_eq!(
            model.permission.first()
                .expect("should have at least one permission object").created,
            1670820592,
        )
    }
}

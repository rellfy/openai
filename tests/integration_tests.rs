use openai::{ embeddings::Embedding, models::ModelID };
use dotenv::dotenv;

#[tokio::test]
async fn embeddings_module_matches_mockup() {
    dotenv().expect("should load .env file");

    let embedding = Embedding::new(
        ModelID::TextEmbeddingAda002,
        "The food was delicious and the waiter...",
        None,
    ).await.expect("should create an embedding").vec;

    assert_eq!(
        embedding.first()
            .expect("should have at least one float"),
        &0.0023064255,
    );
}

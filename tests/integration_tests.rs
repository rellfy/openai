//! This is a mockup of what I want the library to look like.
//! These tests probably won't succeed, but when they do, that means the library is near-perfect

use openai::{
    embeddings::Embedding,
    models::text::EmbeddingModel
};

#[tokio::test]
async fn embeddings_module_matches_mockup() {
    let embedding = Embedding::new(
        EmbeddingModel.Ada2,
        "The food was delicious and the waiter...",
    ).await.expect("should create an embedding");

    assert_eq!(embedding.first(), 0.0023064255);
}

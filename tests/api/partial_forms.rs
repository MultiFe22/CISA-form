use crate::helpers::spawn_app;

#[tokio::test]
async fn post_partial_form_returns_a_200_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    // Construct the body for the partial_form endpoint.
    // Ensure to encode the data correctly for x-www-form-urlencoded.
    let body = "field_json=%7B%22schema_type%22%3A%20%22third_schema%22%2C%20%22attestation_confirmation%22%3A%20false%2C%20%22third_party_assessor_confirmation%22%3A%20false%7D&section=3&complete=false";

    // Act
    let response = app.post_partial_form(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn post_partial_form_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    // Construct the body for the partial_form endpoint.
    // Ensure to encode the data correctly for x-www-form-urlencoded.
    let body = "field_json=%7B%22schema_type%22%3A%20%22third_schema%22%2C%20%22attestation_confirmation%22%3A%20false%2C%20%22third_party_assessor_confirmation%22%3A%20false%7D&section=3";

    // Act
    let response = app.post_partial_form(body.into()).await;

    // Assert
    assert_eq!(400, response.status().as_u16());
}
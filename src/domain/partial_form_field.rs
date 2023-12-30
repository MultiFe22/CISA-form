use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct TypeOfAttestation {
    company_wide: bool,
    individual_product: bool,
    multiple_products_or_specific_product_versions: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    product_name: Option<String>,
    version_number: Option<String>,
    release_publish_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirstSchema {
    new_attestation: bool,
    attestation_following_extension_or_waiver: bool,
    revised_attestation: bool,
    type_of_attestation: TypeOfAttestation,
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContactInformation {
    name: Option<String>,
    title: Option<String>,
    address: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondSchema {
    software_producer_information: ContactInformation,
    primary_contact_for_document: ContactInformation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThirdSchema {
    attestation_confirmation: bool,
    third_party_assessor_confirmation: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "schema_type", rename_all = "snake_case")]
pub enum PartialFormField {
    FirstSchema(FirstSchema),
    SecondSchema(SecondSchema),
    ThirdSchema(ThirdSchema),
}

pub fn parse_json(input: &str) -> Result<PartialFormField> {
    serde_json::from_str(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_first_schema_correctly() {
        let json = r#"{ 
            "schema_type": "first_schema",
            "new_attestation": false,
            "attestation_following_extension_or_waiver": false,
            "revised_attestation": false,
            "type_of_attestation": {
                "company_wide": false,
                "individual_product": false,
                "multiple_products_or_specific_product_versions": false
            },
            "products": [
                {
                    "product_name": null,
                    "version_number": null,
                    "release_publish_date": null
                }
            ]
        }"#;
        let parsed = parse_json(json).unwrap();
        if let PartialFormField::FirstSchema(schema) = parsed {
            assert_eq!(schema.new_attestation, false);
            // Further assertions can be added here for the rest of the fields
        } else {
            panic!("Parsed incorrect schema");
        }
    }

    #[test]
    fn it_parses_first_schema_correctly_with_normal_string() {
        let json = "{ \
        \"schema_type\": \"first_schema\", \
        \"new_attestation\": false, \
        \"attestation_following_extension_or_waiver\": false, \
        \"revised_attestation\": false, \
        \"type_of_attestation\": { \
            \"company_wide\": false, \
            \"individual_product\": false, \
            \"multiple_products_or_specific_product_versions\": false \
        }, \
        \"products\": [ \
            { \
                \"product_name\": null, \
                \"version_number\": null, \
                \"release_publish_date\": null \
            } \
        ] \
    }";
        let parsed = parse_json(json).unwrap();
        if let PartialFormField::FirstSchema(schema) = parsed {
            assert_eq!(schema.new_attestation, false);
            // Further assertions can be added here for the rest of the fields
        } else {
            panic!("Parsed incorrect schema");
        }
    }

    #[test]
    fn it_errors_on_invalid_schema_with_normal_string() {
        let json = "{ \
        \"type\": \"invalid_schema\", \
        \"field_a\": \"Hello\", \
        \"field_b\": 123 \
    }";
        let parsed = parse_json(json);
        assert!(parsed.is_err(), "Parsing should fail for invalid schema");
    }

    // Similarly, add tests for SecondSchema and ThirdSchema
}

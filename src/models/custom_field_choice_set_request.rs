/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldChoiceSetRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldChoiceSetRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// * `IATA` - IATA (Airport codes) * `ISO_3166` - ISO 3166 (Country codes) * `UN_LOCODE` - UN/LOCODE (Location codes)
    #[serde(rename = "base_choices", skip_serializing_if = "Option::is_none")]
    pub base_choices: Option<BaseChoices>,
    #[serde(rename = "extra_choices", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extra_choices: Option<Option<Vec<Vec<String>>>>,
    /// Choices are automatically ordered alphabetically
    #[serde(rename = "order_alphabetically", skip_serializing_if = "Option::is_none")]
    pub order_alphabetically: Option<bool>,
}

impl CustomFieldChoiceSetRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(name: String) -> CustomFieldChoiceSetRequest {
        CustomFieldChoiceSetRequest {
            name,
            description: None,
            base_choices: None,
            extra_choices: None,
            order_alphabetically: None,
        }
    }
}

/// * `IATA` - IATA (Airport codes) * `ISO_3166` - ISO 3166 (Country codes) * `UN_LOCODE` - UN/LOCODE (Location codes)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BaseChoices {
    #[serde(rename = "IATA")]
    Iata,
    #[serde(rename = "ISO_3166")]
    Iso3166,
    #[serde(rename = "UN_LOCODE")]
    UnLocode,
}

impl Default for BaseChoices {
    fn default() -> BaseChoices {
        Self::Iata
    }
}

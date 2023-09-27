/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CustomLink : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomLink {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "content_types")]
    pub content_types: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Jinja2 template code for link text
    #[serde(rename = "link_text")]
    pub link_text: String,
    /// Jinja2 template code for link URL
    #[serde(rename = "link_url")]
    pub link_url: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Links with the same group will appear as a dropdown menu
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The class of the first link in a group will be used for the dropdown button  * `outline-dark` - Default * `blue` - Blue * `indigo` - Indigo * `purple` - Purple * `pink` - Pink * `red` - Red * `orange` - Orange * `yellow` - Yellow * `green` - Green * `teal` - Teal * `cyan` - Cyan * `gray` - Gray * `black` - Black * `white` - White * `ghost-dark` - Link
    #[serde(rename = "button_class", skip_serializing_if = "Option::is_none")]
    pub button_class: Option<ButtonClass>,
    /// Force link to open in a new window
    #[serde(rename = "new_window", skip_serializing_if = "Option::is_none")]
    pub new_window: Option<bool>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl CustomLink {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(id: i32, url: String, display: String, content_types: Vec<String>, name: String, link_text: String, link_url: String, created: Option<String>, last_updated: Option<String>) -> CustomLink {
        CustomLink {
            id,
            url,
            display,
            content_types,
            name,
            enabled: None,
            link_text,
            link_url,
            weight: None,
            group_name: None,
            button_class: None,
            new_window: None,
            created,
            last_updated,
        }
    }
}

/// The class of the first link in a group will be used for the dropdown button  * `outline-dark` - Default * `blue` - Blue * `indigo` - Indigo * `purple` - Purple * `pink` - Pink * `red` - Red * `orange` - Orange * `yellow` - Yellow * `green` - Green * `teal` - Teal * `cyan` - Cyan * `gray` - Gray * `black` - Black * `white` - White * `ghost-dark` - Link
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ButtonClass {
    #[serde(rename = "outline-dark")]
    OutlineDark,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "indigo")]
    Indigo,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "pink")]
    Pink,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "teal")]
    Teal,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "ghost-dark")]
    GhostDark,
}

impl Default for ButtonClass {
    fn default() -> ButtonClass {
        Self::OutlineDark
    }
}


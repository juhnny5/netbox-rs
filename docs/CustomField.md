# CustomField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**content_types** | **Vec<String>** |  | 
**r#type** | [**crate::models::Type7**](Type_7.md) |  | 
**object_type** | Option<**String**> |  | [optional]
**data_type** | Option<**String**> |  | [optional][readonly]
**name** | **String** | Internal field name | 
**label** | Option<**String**> | Name of the field as displayed to users (if not provided, the field's name will be used) | [optional]
**group_name** | Option<**String**> | Custom fields within the same group will be displayed together | [optional]
**description** | Option<**String**> |  | [optional]
**required** | Option<**bool**> | If true, this field is required when creating new objects or editing an existing object. | [optional]
**search_weight** | Option<**i32**> | Weighting for search. Lower values are considered more important. Fields with a search weight of zero will be ignored. | [optional]
**filter_logic** | Option<[**crate::models::FilterLogic**](Filter_logic.md)> |  | [optional]
**ui_visibility** | Option<[**crate::models::UiVisibility**](Ui_visibility.md)> |  | [optional]
**default** | Option<[**serde_json::Value**](.md)> | Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. \"Foo\"). | [optional]
**weight** | Option<**i32**> | Fields with higher weights appear lower in a form. | [optional]
**validation_minimum** | Option<**i32**> | Minimum allowed value (for numeric fields) | [optional]
**validation_maximum** | Option<**i32**> | Maximum allowed value (for numeric fields) | [optional]
**validation_regex** | Option<**String**> | Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters. | [optional]
**choices** | Option<**Vec<String>**> | Comma-separated list of available choices (for selection fields) | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



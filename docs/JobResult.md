# JobResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**status** | Option<[**crate::models::Status7**](Status_7.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**scheduled** | Option<**String**> |  | [optional]
**interval** | Option<**i32**> | Recurrence interval (in minutes) | [optional]
**started** | Option<**String**> |  | [optional]
**completed** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**obj_type** | Option<**String**> |  | [optional][readonly]
**user** | Option<[**crate::models::NestedUser**](NestedUser.md)> |  | [optional]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**job_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



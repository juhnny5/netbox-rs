# DeviceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**manufacturer** | [**crate::models::NestedManufacturer**](NestedManufacturer.md) |  | 
**model** | **String** |  | 
**slug** | **String** |  | 
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**f32**> |  | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces | [optional]
**subdevice_role** | Option<[**crate::models::SubdeviceRole**](Subdevice_role.md)> |  | [optional]
**airflow** | Option<[**crate::models::Airflow**](Airflow.md)> |  | [optional]
**weight** | Option<**f32**> |  | [optional]
**weight_unit** | Option<[**crate::models::WeightUnit**](Weight_unit.md)> |  | [optional]
**front_image** | Option<**String**> |  | [optional][readonly]
**rear_image** | Option<**String**> |  | [optional][readonly]
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



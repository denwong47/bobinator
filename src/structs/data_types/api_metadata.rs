/// A "metadata" object returned by an API endpoint.
/// ```ignore
/// {
///     "canBeDeleted": false,
///     "isRequired": false,
///     "isFilterable": true,
///     "isDynamic": false,
///     "siteFilter": {
///         "type": "all-sites"
///     },
///     "isCalculated": false,
///     "description": null,
///     "type": "employee",
///     "categoryName": "work",
///     "mandatory": false,
///     "isTableColumn": true,
///     "path": "/work/reportsTo",
///     "isReadOnly": false,
///     "operators": [
///         "employee_id",
///         "indirect_reportees",
///         "exists",
///         "not_exists",
///         "referred_to_by",
///         "not_referred_to_by"
///     ],
///     "name": "Reports to",
///     "isHistorical": true,
///     "isSearchable": true,
///     "isDisplayable": true,
///     "immutableAttributes": {
///         "siteFilter": true,
///         "isHistorical": true,
///         "mandatory": false,
///         "isRequired": false
///     }
/// }
/// ```
pub struct Metadata {}

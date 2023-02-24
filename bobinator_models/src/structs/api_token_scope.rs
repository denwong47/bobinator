use enum_index::*;

/// API token scope item.
#[derive(Clone, Debug, EnumIndex, PartialEq)]
#[index_type(String)]
pub enum APITokenScope {
    #[index("employee_read")]
    FullEmployeeRead,

    #[index("timeoff")]
    Timeoff,

    #[index("document_view")]
    DocumentView,

    #[index("document_manipulate")]
    DocumentUpload,

    #[index("document_download")]
    DocumentDownload,

    #[index("employee_update")]
    EmployeeUpdate,

    #[index("employee_fields_read")]
    EmployeeFieldsRead,

    #[index("employee_fields_write")]
    EmployeeFieldsWrite,

    #[index("payroll_report_read")]
    PayrollReportRead,

    #[index("onboarding_wizards_read")]
    OnboardingWizardsRead,

    #[index("tasks_company_read")]
    OpenTasksRead,

    #[index("tasks_write")]
    ModifyTasks,
}

use bobinator::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BASE_TOKEN: APIToken = APIToken {
        token: String::from("****************************************"),
        name: String::from("user-token"),
        scopes: vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead
        ],
        employee_id: String::from("0000000000000000000"),
        id: 12345
    };
}

/// Generate a new token, cloned from [`BASE_TOKEN`], allowing mutation.
pub fn new_token() -> APIToken {
    BASE_TOKEN.clone()
}

mod test_add_scope {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $token:expr,
            $operation:ident,
            $operator:expr,
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $name() {
                let mut token = $token;

                token.$operation($operator);

                assert_eq!(token.scopes, $expected)
            }
        };
    }

    test_factory!(
        extend_one_new,
        new_token(),
        extend_scopes,
        vec![APITokenScope::DocumentView],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
            APITokenScope::DocumentView
        ]
    );

    test_factory!(
        extend_one_existing,
        new_token(),
        extend_scopes,
        vec![APITokenScope::Timeoff],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
        ]
    );

    test_factory!(
        extend_empty,
        new_token(),
        extend_scopes,
        vec![],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
        ]
    );

    test_factory!(
        extend_multiple_new,
        new_token(),
        extend_scopes,
        vec![
            APITokenScope::EmployeeUpdate,
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::PayrollReportRead,
        ],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
            APITokenScope::EmployeeUpdate,
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::PayrollReportRead,
        ]
    );

    test_factory!(
        extend_mixture,
        new_token(),
        extend_scopes,
        vec![
            APITokenScope::EmployeeUpdate,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::PayrollReportRead,
            APITokenScope::FullEmployeeRead,
        ],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
            APITokenScope::EmployeeUpdate,
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::PayrollReportRead,
        ]
    );

    test_factory!(
        drop_one_existing,
        new_token(),
        drop_scopes,
        vec![APITokenScope::Timeoff,],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::EmployeeFieldsRead,
        ]
    );

    test_factory!(
        drop_one_new,
        new_token(),
        drop_scopes,
        vec![APITokenScope::OpenTasksRead,],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
        ]
    );

    test_factory!(
        drop_multiple_existing,
        new_token(),
        drop_scopes,
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
        ],
        vec![]
    );

    test_factory!(
        drop_multiple_new,
        new_token(),
        drop_scopes,
        vec![
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::PayrollReportRead,
            APITokenScope::OnboardingWizardsRead,
        ],
        vec![
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::EmployeeFieldsRead,
        ]
    );

    test_factory!(
        drop_mixture,
        new_token(),
        drop_scopes,
        vec![
            APITokenScope::EmployeeFieldsWrite,
            APITokenScope::FullEmployeeRead,
            APITokenScope::Timeoff,
            APITokenScope::PayrollReportRead,
        ],
        vec![APITokenScope::EmployeeFieldsRead,]
    );
}

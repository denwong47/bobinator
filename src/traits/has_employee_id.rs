use async_trait::async_trait;
use reqwest::Client;

use bobinator_models::structs::BobinatorError;

use crate::{bob, Employee};

/// A trait indicating the struct has an employee ID.
pub trait HasEmployeeId: Sync {
    /// Return the primary associated date of the instance.
    fn employee_id<'a>(&'a self) -> &'a str;
}

impl HasEmployeeId for String {
    fn employee_id<'a>(&'a self) -> &'a str {
        self.as_str()
    }
}

/// A trait indicating the struct carries information to query for an
/// [`Employee`] object.
#[async_trait]
pub trait CanEnquireEmployee: HasEmployeeId {
    /// Enquire the server about the employee.
    async fn enquire_employee(&self, conn: &Client) -> Result<Employee, BobinatorError>;
}

#[async_trait]
impl<T> CanEnquireEmployee for T
where
    T: HasEmployeeId,
{
    /// Enquire the server about the employee.
    async fn enquire_employee(&self, conn: &Client) -> Result<Employee, BobinatorError> {
        bob::cookies::employee::enquire(conn, self).await
    }
}

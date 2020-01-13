use crate::models::UserRef;

#[derive(Copy, Clone)]
pub struct Permission {
    pub user: UserRef,
    pub r#type: PermissionType,
}

#[derive(Copy, Clone)]
pub enum PermissionType {
    Read,
    Write,
    Moderate,
    Admin,
    Owner
}

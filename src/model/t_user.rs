use rbatis::core::Error;
use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::rbatis::Rbatis;

use crate::utils::g::RB_SESSION;

#[crud_table]
#[derive(Default, Clone, Debug)]
pub struct TUser {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub pass: Option<String>,
}

pub async fn query_t_user_by_name(name: &String) -> Result<Option<TUser>, rbatis::core::Error> {
    let w = RB_SESSION
        .as_ref()
        .read()
        .await
        .as_ref()
        .unwrap()
        .new_wrapper()
        .eq("name", name);
    RB_SESSION
        .as_ref()
        .read()
        .await
        .as_ref()
        .unwrap()
        .fetch_by_wrapper(w)
        .await
}

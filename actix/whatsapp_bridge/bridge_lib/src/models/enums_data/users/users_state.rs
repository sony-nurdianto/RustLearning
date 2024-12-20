use crate::models::structs_data::databases::users::Users;

pub enum UsersState {
    Exists(Users),
    NotExist,
}

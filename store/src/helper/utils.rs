use crate::structure::userstr::User;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn get_user(input_email: &String, conn: &mut PgConnection) -> bool {
    use crate::schema::user::dsl::*;

    let result = user
        .filter(email.eq(input_email))
        .select(User::as_select())
        .load(conn)
        .expect("Error loading user");

    if result.len() > 0 {
        return true;
    }
    false
}

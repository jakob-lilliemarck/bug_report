use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::{DatabaseErrorKind, Error},
};

#[macro_use]
extern crate diesel;

mod schema;

#[derive(Debug, Queryable)]
pub struct ScriptTagRead {
    name: String,
    tag_id: i32,
    is_output: bool,
    count: Option<i64>,
}

fn main() {
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(
            "postgres://sbox:dev@localhost/sbox",
        ))
        .expect("");
    let conn = pool.get().and_then(|mut c| {
        let (a, b, c) = diesel::alias!(
            schema::script_tag_assoc as a,
            schema::script_tag_assoc as b,
            schema::tags as c
        );
        let query = a
            .select((
                schema::tags::dsl::name,
                a.field(schema::script_tag_assoc::dsl::tag_id),
                a.field(schema::script_tag_assoc::dsl::is_output),
                b.filter(
                    b.field(schema::script_tag_assoc::dsl::tag_id)
                        .eq(a.field(schema::script_tag_assoc::dsl::tag_id))
                        .and(b.field(schema::script_tag_assoc::dsl::is_output).eq(false)),
                )
                .count()
                .single_value(),
            ))
            .inner_join(schema::tags::dsl::name);
        println!("{:?}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
        Ok(())
    });
}

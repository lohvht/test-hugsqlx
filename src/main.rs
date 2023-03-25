mod db {
    use hugsqlx::HugSqlx;
    #[derive(HugSqlx)]
    #[queries = "test.sql"]
    pub struct DB {}
}

pub use db::*;
use hugsqlx::params;
// use sqlx::sqlite::SqlitePoolOptions;
use sqlx::mysql::MySqlPoolOptions;

#[derive(Debug, sqlx::FromRow)]
struct Table {
    name_str: String,
}

#[derive(Debug, sqlx::FromRow)]
struct Tables_from_show {
    Tables_in_my_database: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:password@localhost:4000/my_database")
        .await?;

    // let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    for q in (1..100).map(|f| format!("create table a{} (id bigint primary key)", f)) {
        sqlx::query(&q).execute(&pool).await?;
    }
    let abc = DB::show_abc::<_, Table>(&pool, params!()).await?;
    println!("ABC IS: {:?}", abc);

    println!("Hello, world");
    let tables = DB::show_tables::<_, Tables_from_show>(&pool, params!()).await?;

    println!("TABLES ARE: {:?}", tables);

    for q in (1..100).map(|f| format!("drop table a{}", f)) {
        sqlx::query(&q).execute(&pool).await?;
    }

    // use sqlx::postgres::Postgres;
    // use sqlx::postgres::PgArguments;
    // use sqlx::postgres::PgRow;
    // use sqlx::postgres::PgQueryResult;

    // use sqlx::sqlite::Sqlite;
    // use sqlx::sqlite::SqliteArguments<'async_trait;
    // use sqlx::sqlite::SqliteRow;
    // use sqlx::sqlite::SqliteQueryResult;

    // use sqlx::mysql::MySql;
    // use sqlx::mysql::MySqlArguments;
    // use sqlx::mysql::MySqlRow;
    // use sqlx::mysql::MySqlQueryResult;

    Ok(())
}

// docker run --name mysql_container -e MYSQL_ROOT_PASSWORD=password -e MYSQL_DATABASE=my_database -p 4000:3306  -d mysql:latest
// mysql -uroot -ppassword -Dmy_database -P4000

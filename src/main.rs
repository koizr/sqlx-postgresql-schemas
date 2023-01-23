use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    println!("Hello, world!");

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await?;

    // ここの result の型は { id: i32, title: String, body: String }
    let result = sqlx::query!("SELECT * FROM memo").fetch_one(&pool).await?;
    println!("{}", &result.body);

    // ここの result の型は { id: i32, secret_body: Option<String> }
    let result = sqlx::query!("SELECT * FROM secret.memo") // ← スキーマを指定すると、ちゃんとそのスキーマを見に行ってコンパイルしてくれる
        .fetch_one(&pool)
        .await?;
    println!("{}", &result.secret_body.unwrap());

    Ok(())
}

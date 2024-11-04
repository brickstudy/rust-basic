use std::io;
use mysql::*;
use std::env;
use dotenv::dotenv;

pub fn get_input() -> Option<String> {
    let mut buffer: String = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again.");
    };
    // trim whitespace from input
    let input: String = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

pub fn get_mysql_conn() -> Result<PooledConn>{
    dotenv().ok();

    let db_user = env::var("MYSQL_USER").expect("DB_USER must be set");
    let db_pass = env::var("MYSQL_PASSWORD").expect("DB_PASS must be set");
    let db_host = env::var("MYSQL_HOST").expect("DB_HOST must be set");
    let db_port = env::var("MYSQL_PORT").expect("DB_PORT must be set");
    let db_name = env::var("MYSQL_DB").expect("DB_NAME must be set");

    let opts = OptsBuilder::new()
        .user(Some(db_user))
        .pass(Some(db_pass))
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port.parse::<u16>().expect("DB Port must be a valid port number"))
        .db_name(Some(db_name));

    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}

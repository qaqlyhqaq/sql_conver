/*
拉取表结构
 */
use convert_case::{Boundary, Case, Casing};
use sqlx::{Column, Row};
use serde::Serialize;

//没有为 `MyColumn` 实现特征 `Serialize` [E0277]
#[derive(Clone,Serialize,Debug)]
pub struct MyColumn{
    name : String,
    raw_name : String,
    type_info : String,
    type_java : String,
    commentary:String,
}


#[derive(Clone,Serialize,Debug)]
pub struct MyTable{
    pub table_name : String,
    pub column_vec : Vec<MyColumn>,
}

fn mysql_type_map(mysql_type:String)->String{
    match mysql_type.as_str() {
        "bigint" => "long".to_string(),
        "varchar" => "String".to_string(),
        "tinyint" => "int".to_string(),
        "text" => "String".to_string(),
        "int" => "int".to_string(),
        "datetime" => "Timestamp".to_string(),
        unMatch => {panic!("unMatch:{}",unMatch)}
    }
}


pub async fn fetch_table_struct(table_name:String) -> MyTable {
    let database_url = include_str!("../../assets/database_url");
    let mut pool = sqlx::MySqlPool::connect(database_url).await.unwrap();
    let mut conn = pool.acquire().await.unwrap();

    let statements = format!("SELECT * FROM information_schema.columns WHERE TABLE_SCHEMA = 'official_dev' AND TABLE_NAME = '{}';", table_name);
    let mut rows = sqlx::query(statements.as_str())
        .map(|row: sqlx::mysql::MySqlRow| {
            MyColumn{
                raw_name:row.get(3),
                name:row.get::<String, usize>(3).from_case(Case::Snake)
                    .without_boundaries(&[Boundary::DigitUpper, Boundary::DigitLower])
                    .to_case(Case::Camel),
                type_info:String::from_utf8(row.try_get::<Vec<u8>, usize>(7).unwrap()).unwrap() ,
                type_java:mysql_type_map(String::from_utf8(row.try_get::<Vec<u8>, usize>(7).unwrap()).unwrap()) ,
                commentary:String::from_utf8(row.get::<Vec<u8>,usize>(19)).unwrap(),
            }
        })
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    MyTable{
        table_name:table_name,
        column_vec: rows,
    }

}
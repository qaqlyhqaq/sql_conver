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
    commentary:String,
}


#[derive(Clone,Serialize,Debug)]
pub struct MyTable{
    pub table_name : String,
    pub column_vec : Vec<MyColumn>,
}


pub async fn fetch_table_struct() -> MyTable {
    let table_name = "tb_official_resource_store".to_string();
    let mut pool = sqlx::MySqlPool::connect("mysql://").await.unwrap();
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
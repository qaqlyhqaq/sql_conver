/*
拉取表结构
 */
use sqlx::{Column, Row};
use serde::Serialize;

//没有为 `MyColumn` 实现特征 `Serialize` [E0277]
#[derive(Clone,Serialize)]
#[derive(Debug)]
pub struct MyColumn{
    name : String,
    type_info : String,
}


pub async fn fetch_table_struct() -> Vec<MyColumn> {
    let mut pool = sqlx::MySqlPool::connect("").await.unwrap();
    let mut conn = pool.acquire().await.unwrap();

    let mut rows = sqlx::query("SELECT * FROM information_schema.columns WHERE TABLE_SCHEMA = 'official_dev' AND TABLE_NAME = 'tb_official_resource_store';")
        .map(|row: sqlx::mysql::MySqlRow| {
            let type_info = if row.try_get::<Vec<u8>, usize>(7).is_ok(){
                String::from_utf8(row.try_get::<Vec<u8>, usize>(7).unwrap()).unwrap()
            }else{
                "无类型".to_string()
            };
            MyColumn{
                name:row.get(3),
                type_info:type_info ,
            }
        })
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    // for x in rows.iter() {
    //     println!("name:{} type:{}", x.name,x.type_info);
    //     // println!("{:?}", row);
    // }

    rows

}
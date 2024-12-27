/*
生成 实体填充 方法调用
 */
use crate::sql::fetch_table_struct::fetch_table_struct;

async  fn generate_lombok_get_accessors(table_name:String){
    let table = fetch_table_struct(table_name).await;
}


#[cfg(test)]
mod tests {
    use crate::sql::generate_lombok_call::generate_lombok_get_accessors;

    #[tokio::test]
    async fn generate_lombok_call(){
        let table_name = "tb_official_resource_store".to_string();
        generate_lombok_get_accessors(table_name).await;
    }
}
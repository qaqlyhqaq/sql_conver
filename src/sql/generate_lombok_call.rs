/*
生成 实体填充 方法调用
 */
use crate::capitalize_first_letter;
use crate::sql::fetch_table_struct::fetch_table_struct;




async  fn generate_lombok_get_accessors(table_name:String){
    let table = fetch_table_struct(table_name).await;

    let x = table.column_vec.iter()
        .map(|col| { col.showName() })
        .collect::<Vec<_>>();

    x.iter()
        .filter(|x1| {
           return !( x1.contains("updateBy")
               |x1.contains("updateTime")
               |x1.contains("deleted")
            ||x1.contains("createTime")
            ||x1.contains("createBy"));
        })
    .for_each(|col| {
        let string = capitalize_first_letter(&col);
        println!(".set{0}(agg.get{0}())", string)
    });

}


#[cfg(test)]
mod tests {
    use crate::sql::generate_lombok_call::generate_lombok_get_accessors;

    //根据表格 生成方法 测试样例
    #[tokio::test]
    async fn generate_lombok_call(){
        let table_name = "tb_official_resource_store".to_string();
        generate_lombok_get_accessors(table_name).await;
    }
}
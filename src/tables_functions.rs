use mysql::*;

use super::metadata;
use metadata::FieldType;
use metadata::TableDef;

use mysql::prelude::Queryable;

pub fn create_tables(tables: Vec<TableDef>, conn: &mut PooledConn){
    for table in tables {
        let sql = create_table_sql(&table);
        conn.query_drop(&sql).unwrap();
    }
}

fn create_table_sql(table: &TableDef) -> String {
    let mut fields = String::new();
    fields.push_str("id INT AUTO_INCREMENT PRIMARY KEY,");
    for field in &table.field_defs {
        fields.push_str(&format!("{name} {ftype},", name = field.name, ftype = field_type_to_sql(&field.field_type)));
    }

    format!("CREATE TABLE IF NOT EXISTS {name} ({fields})", name = table.name, fields = fields.trim_end_matches(','))
}

fn field_type_to_sql(field_type: &FieldType) -> &str {
    match field_type {
        FieldType::String => "VARCHAR(255)",
        FieldType::Int => "INT",
        FieldType::Date => "DATE",
    }
}
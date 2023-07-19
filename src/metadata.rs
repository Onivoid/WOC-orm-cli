pub enum FieldType { String, Int, Date } //énumémration qui définit les types possibles pour une colonne

pub struct FieldDef { pub name: String, pub field_type: FieldType } //  structure qui contient le nom d'un champ (colonne) et son type de données.

pub struct TableDef { pub name: String, pub field_defs: Vec<FieldDef>} // structure qui contient le nom d'une table et la définition de tous ses champs 

/**
 * @parse
 * convertit une chaîne de caractères en type FieldType.
 */
impl FieldType {
    pub fn parse(text: &str) -> Self {
        match text {
            "int" => FieldType::Int, 
            "string" => FieldType::String,
            "date" => FieldType::Date,
            _=> unimplemented!("bleh")
        }
    }
}

/**
 * @parse
 * crée une instance de FieldDef à partir d'une chaîne de caractères.
 */
impl FieldDef {
    pub fn parse(text: &str) -> Self {
        let mut items = text.trim().split(':');
        let name = items.next().unwrap().trim();
        let field_type = items.next().unwrap().trim();
        FieldDef { name: name.to_string(), field_type: FieldType::parse(field_type) }
    }
}

/**
 * crée une instance de TableDef à partir d'une chaîne de caractères.
 */
impl TableDef {
    pub fn parse(text: &str) -> Self {
        let mut items = text.trim().split('(');
        let name = items.next().unwrap().trim();
        let field_defs = items.next().unwrap().trim().split(')').next().unwrap().split(',');
        TableDef { name: name.to_string(), field_defs: field_defs.map(FieldDef::parse).collect() }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_field_type() {
        assert!(matches!(FieldType::parse("string"), FieldType::String));
        assert!(matches!(FieldType::parse("int"), FieldType::Int));
        assert!(matches!(FieldType::parse("date"), FieldType::Date));
    }
    #[test]
    fn test_parse_field_def() {
        let field_def = FieldDef::parse("age:int");
        assert_eq!(field_def.name, "age");
        assert!(matches!(field_def.field_type, FieldType::Int))
    }

    #[test]
    fn test_parse_table_def() {
        let table_def =  TableDef::parse("users(name:string,surname:string,age:int)");
        assert_eq!(table_def.name, "users");
        assert!(matches!(table_def.field_defs[0].field_type, FieldType::String));
        assert!(matches!(table_def.field_defs[1].field_type, FieldType::String));
        assert!(matches!(table_def.field_defs[2].field_type, FieldType::Int))
    }
}
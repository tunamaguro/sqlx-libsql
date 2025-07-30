use sqlx_core::type_info::TypeInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LibSqlTypeInfo(pub(crate) LibSqlType);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibSqlType {
    Integer,
    Real,
    Text,
    Blob,
    Null,
}

impl LibSqlType {
    fn display_name(&self) -> &str {
        match self {
            LibSqlType::Integer => "INTEGER",
            LibSqlType::Real => "REAL",
            LibSqlType::Text => "TEXT",
            LibSqlType::Blob => "BLOB",
            LibSqlType::Null => "NULL",
        }
    }
}

impl TypeInfo for LibSqlTypeInfo {
    fn is_null(&self) -> bool {
        matches!(self.0, LibSqlType::Null)
    }

    fn name(&self) -> &str {
        self.0.display_name()
    }
}

impl std::fmt::Display for LibSqlTypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display_name().fmt(f)
    }
}

use sqlx_core::value::{Value, ValueRef};

pub struct LibSqlValue {}

pub struct LibSqlValueRef<'r> {
    _phantom: std::marker::PhantomData<&'r ()>,
}

impl Value for LibSqlValue {
    type Database = crate::LibSql;

    fn as_ref(&self) -> <Self::Database as sqlx_core::database::Database>::ValueRef<'_> {
        todo!()
    }

    fn type_info(
        &self,
    ) -> std::borrow::Cow<'_, <Self::Database as sqlx_core::database::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl<'r> ValueRef<'r> for LibSqlValueRef<'r> {
    type Database = crate::LibSql;

    fn to_owned(&self) -> <Self::Database as sqlx_core::database::Database>::Value {
        todo!()
    }

    fn type_info(
        &self,
    ) -> std::borrow::Cow<'_, <Self::Database as sqlx_core::database::Database>::TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

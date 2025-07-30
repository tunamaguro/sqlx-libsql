use sqlx_core::arguments::Arguments;

#[derive(Debug, Default)]
pub struct LibSqlArgument<'q> {
    _phantom: std::marker::PhantomData<&'q ()>,
}

pub struct LibSqlArgumentBuffer<'q> {
    _phantom: std::marker::PhantomData<&'q ()>,
}

impl<'q> Arguments<'q> for LibSqlArgument<'q> {
    type Database = crate::LibSql;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T) -> Result<(), sqlx_core::error::BoxDynError>
    where
        T: 'q
            + sqlx_core::encode::Encode<'q, Self::Database>
            + sqlx_core::types::Type<Self::Database>,
    {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

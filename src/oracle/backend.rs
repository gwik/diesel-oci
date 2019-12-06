use diesel::backend::UsesAnsiSavepointSyntax;
use diesel::backend::*;
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::TypeMetadata;

use super::connection::OracleValue;
use super::query_builder::OciQueryBuilder;
use oracle::types::OciDataType;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Oracle;

impl Backend for Oracle {
    type QueryBuilder = OciQueryBuilder;
    type BindCollector = RawBytesBindCollector<Oracle>;
}

impl<'a> HasRawValue<'a> for Oracle {
    type RawValue = OracleValue<'a>;
}

impl<'a> BinaryRawValue<'a> for Oracle {
    type ByteOrder = byteorder::NativeEndian;

    fn as_bytes(value: Self::RawValue) -> &'a [u8] {
        value.bytes
    }
}

impl TypeMetadata for Oracle {
    type TypeMetadata = OciDataType;
    type MetadataLookup = ();
}

impl UsesAnsiSavepointSyntax for Oracle {}

// TODO: check if Oracle supports this
//impl SupportsDefaultKeyword for Oracle {}
impl SupportsReturningClause for Oracle {}

pub trait HasSqlTypeExt<ST>: HasSqlType<ST, MetadataLookup = ()> {
    fn oci_row_metadata(out: &mut Vec<Self::TypeMetadata>);
}

impl<ST> HasSqlTypeExt<ST> for Oracle
where
    Oracle: HasSqlType<ST>,
{
    default fn oci_row_metadata(out: &mut Vec<Self::TypeMetadata>) {
        out.push(Self::metadata(&()))
    }
}

macro_rules! tuple_impls {
    ($(
        $Tuple:tt {
            $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+
        }
    )+) => {
        $(
            impl<$($T),+> HasSqlTypeExt<($($T,)+)> for Oracle
                where $(Oracle: HasSqlTypeExt<$T>,)*
            {
                fn oci_row_metadata(out: &mut Vec<Self::TypeMetadata>) {
                    $(<Oracle as HasSqlTypeExt<$T>>::oci_row_metadata(out);)+
                }
            }
        )*
    };
}

__diesel_for_each_tuple!(tuple_impls);

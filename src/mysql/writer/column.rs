use crate::mysql::def::{CharSet, ColumnInfo, NumericAttr, StringAttr, Type};
use sea_query::{escape_string, Alias, ColumnDef, Iden};
use std::fmt::Write;

impl ColumnInfo {
    pub fn write(&self) -> ColumnDef {
        let mut col_def = ColumnDef::new(Alias::new(self.name.as_str()));
        col_def = self.write_col_type(col_def);
        if !self.null {
            col_def.not_null();
        }
        if self.extra.auto_increment {
            col_def.auto_increment();
        }
        let mut extras = Vec::new();
        if let Some(default) = self.default.as_ref() {
            let mut string = "".to_owned();
            write!(&mut string, "DEFAULT {}", default.expr).unwrap();
            extras.push(string);
        }
        if self.extra.on_update_current_timestamp {
            extras.push("ON UPDATE CURRENT_TIMESTAMP".to_owned());
        }
        if !self.comment.is_empty() {
            let mut string = "".to_owned();
            write!(&mut string, "COMMENT '{}'", escape_string(&self.comment)).unwrap();
            extras.push(string);
        }
        if !extras.is_empty() {
            col_def.extra(extras.join(" "));
        }
        col_def
    }

    pub fn write_col_type(&self, mut col_def: ColumnDef) -> ColumnDef {
        match &self.col_type {
            Type::Serial => {
                col_def
                    .big_integer()
                    .extra("UNSIGNED".into())
                    .not_null()
                    .auto_increment()
                    .unique_key();
            }
            Type::Bit(num_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::TinyInt(num_attr) => {
                match num_attr.maximum {
                    Some(maximum) => col_def.tiny_integer_len(maximum),
                    None => col_def.tiny_integer(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::Bool => {
                col_def.boolean();
            }
            Type::SmallInt(num_attr) => {
                match num_attr.maximum {
                    Some(maximum) => col_def.small_integer_len(maximum),
                    None => col_def.small_integer(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::MediumInt(num_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Int(num_attr) => {
                match num_attr.maximum {
                    Some(maximum) => col_def.integer_len(maximum),
                    None => col_def.integer(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::BigInt(num_attr) => {
                match num_attr.maximum {
                    Some(maximum) => col_def.big_integer_len(maximum),
                    None => col_def.big_integer(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::Decimal(num_attr) => {
                match (num_attr.maximum, num_attr.decimal) {
                    (Some(maximum), Some(decimal)) => col_def.decimal_len(maximum, decimal),
                    _ => col_def.decimal(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::Float(num_attr) => {
                match num_attr.decimal {
                    Some(decimal) => col_def.float_len(decimal),
                    _ => col_def.float(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::Double(num_attr) => {
                match num_attr.decimal {
                    Some(decimal) => col_def.double_len(decimal),
                    _ => col_def.double(),
                };
                col_def = self.write_num_attr(col_def, num_attr);
            }
            Type::Date => {
                col_def.date();
            }
            Type::Time(time_attr) => {
                match time_attr.fractional {
                    Some(fractional) => col_def.time_len(fractional),
                    _ => col_def.time(),
                };
            }
            Type::DateTime(time_attr) => {
                match time_attr.fractional {
                    Some(fractional) => col_def.date_time_len(fractional),
                    _ => col_def.date_time(),
                };
            }
            Type::Timestamp(time_attr) => {
                match time_attr.fractional {
                    Some(fractional) => col_def.timestamp_len(fractional),
                    _ => col_def.timestamp(),
                };
            }
            Type::Year => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Char(str_attr) => {
                match str_attr.length {
                    Some(length) => col_def.char_len(length),
                    _ => col_def.char(),
                };
                col_def = self.write_str_attr(col_def, str_attr);
            }
            Type::NChar(str_attr) => {
                match str_attr.length {
                    Some(length) => col_def.char_len(length),
                    _ => col_def.char(),
                };
                col_def.extra(format!("CHARACTER SET {}", CharSet::Utf8.to_string()));
            }
            Type::Varchar(str_attr) => {
                match str_attr.length {
                    Some(length) => col_def.string_len(length),
                    _ => col_def.string(),
                };
                col_def = self.write_str_attr(col_def, str_attr);
            }
            Type::NVarchar(str_attr) => {
                match str_attr.length {
                    Some(length) => col_def.string_len(length),
                    _ => col_def.string(),
                };
                col_def.extra(format!("CHARACTER SET {}", CharSet::Utf8.to_string()));
            }
            Type::Binary(str_attr) => {
                match str_attr.length {
                    Some(length) => col_def.binary_len(length),
                    _ => col_def.binary(),
                };
                col_def = self.write_str_attr(col_def, str_attr);
            }
            Type::Varbinary(str_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Text(str_attr) => {
                col_def.text();
                col_def = self.write_str_attr(col_def, str_attr);
            }
            Type::TinyText(str_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::MediumText(str_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::LongText(str_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Blob(blob_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::TinyBlob => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::MediumBlob => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::LongBlob => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Enum(enum_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Set(set_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Geometry(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Point(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::LineString(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Polygon(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::MultiPoint(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::MultiLineString(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::MultiPolygon(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::GeometryCollection(geo_attr) => {
                // FIXME: Unresolved type mapping
                col_def.custom(self.col_type.clone());
            }
            Type::Json => {
                col_def.json();
            }
            Type::Unknown(str) => {
                col_def.custom(self.col_type.clone());
            }
        };
        col_def
    }

    pub fn write_num_attr(&self, mut col_def: ColumnDef, num_attr: &NumericAttr) -> ColumnDef {
        match num_attr.unsigned {
            Some(unsigned) => {
                col_def.extra("UNSIGNED".into());
            }
            None => {}
        };
        match num_attr.zero_fill {
            Some(zero_fill) => {
                col_def.extra("ZEROFILL".into());
            }
            None => {}
        };
        col_def
    }

    pub fn write_str_attr(&self, mut col_def: ColumnDef, str_attr: &StringAttr) -> ColumnDef {
        match &str_attr.charset {
            Some(charset) => {
                col_def.extra(format!("CHARACTER SET {}", charset.to_string()));
            }
            _ => {}
        };
        match &str_attr.collation {
            Some(collation) => {
                col_def.extra(format!("COLLATE {}", collation.to_string()));
            }
            _ => {}
        };
        col_def
    }
}

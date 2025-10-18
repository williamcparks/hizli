use std::fmt::Write;

use quote::ToTokens;
use syn::DataEnum;

pub fn sum_expected_one_of(e: &DataEnum, id: &str) -> String {
    let mut buf = String::new();

    for field in e.variants.iter().filter_map(|v| v.fields.iter().next()) {
        if buf.is_empty() {
            buf.push_str("Error Parsing: ");
            buf.push_str(id);
            buf.push_str(", Expected One Of: ");
        } else {
            buf.push_str(", ");
        }
        write!(&mut buf, "{}", field.ty.to_token_stream()).unwrap();
    }

    buf
}

use lazy_static::lazy_static;
use harsh::Harsh;
use serde::Serializer;
use chrono::FixedOffset;

lazy_static! {
    pub static ref TZ_CN: FixedOffset = FixedOffset::east(8 * 3600);
    static ref HARSH: Harsh = Harsh::builder().length(16).salt("some slat").build().unwrap();
}

pub(crate) fn grable_id<S>(id: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode_i64(id))
}

pub fn encode_i64(id: &i64) -> String {
    HARSH.encode(&[(*id).try_into().unwrap()])
}

pub fn decode_i64(id: &String) -> i64 {
    let x = HARSH.decode(id);
    if x.is_err() {
        return 0;
    }
    for id in x.unwrap().into_iter() {
        return id as i64;
    }
    0
}
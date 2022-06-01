use chrono::{DateTime, Local, TimeZone};
use harsh::Harsh;
use once_cell::sync::OnceCell;
use serde::{Serializer, ser::Error};

static HARSH: OnceCell<Harsh> = OnceCell::new();

pub fn init_harsh(min_len: usize, salt: &str) -> () {
    HARSH
        .set(Harsh::builder().length(min_len).salt(salt).build().unwrap())
        .expect("failed to init harsh");
}

pub fn grable_id<S>(id: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode_u64(*id))
}

pub fn grable_id_u32<S>(id: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode_u64(*id as u64))
}

pub fn encode_u64(id: u64) -> String {
    HARSH.get().unwrap().encode(&[id])
}

pub fn decode_i64(id: &String) -> u64 {
    let x = HARSH.get().unwrap().decode(id);
    if x.is_err() {
        return 0;
    }
    for id in x.unwrap().into_iter() {
        return id;
    }
    0
}

pub fn format_time<S>(id: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if 0 == *id {
        return serializer.serialize_char('0');
    }
    let dt: DateTime<Local> = Local.timestamp((*id) as i64, 0);
    serializer.serialize_str(&dt.to_rfc3339())
}
use chrono::{DateTime, Local, TimeZone};
use harsh::Harsh;
use once_cell::sync::OnceCell;
use serde::Serializer;

static HARSH: OnceCell<Harsh> = OnceCell::new();

pub fn init_harsh(min_len: usize, salt: &str) -> () {
    HARSH
        .set(Harsh::builder().length(min_len).salt(salt).build().unwrap())
        .expect("failed to init harsh");
}

// 混淆ID
pub fn confuse<T, S>(id: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Into<u64> + Copy,
    S: Serializer,
{
    serializer.serialize_str(&encode_u64((*id).into()))
}

pub fn encode_u64(id: u64) -> String {
    HARSH.get().unwrap().encode(&[id])
}

pub fn decode_u64(id: &String) -> u64 {
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
    let dt: DateTime<Local> = Local.timestamp_opt((*id) as i64, 0).unwrap();
    serializer.serialize_str(&dt.to_rfc3339())
}

pub fn is_zero<T>(i: &T) -> bool
where
    T: PartialEq + Default,
{
    i.eq(&T::default())
}

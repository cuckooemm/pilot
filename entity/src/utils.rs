use chrono::FixedOffset;
use harsh::Harsh;
use once_cell::sync::OnceCell;
use serde::Serializer;

static HARSH: OnceCell<Harsh> = OnceCell::new();

static TZ_CN: OnceCell<FixedOffset> = OnceCell::new();

pub fn get_time_zone() -> &'static FixedOffset {
    TZ_CN.get().expect("failed to init time zone")
}

pub fn init_harsh(min_len: usize, salt: &str) -> () {
    HARSH
        .set(Harsh::builder().length(min_len).salt(salt).build().unwrap())
        .expect("failed to init harsh");
    TZ_CN
        .set(FixedOffset::east(8 * 3600))
        .expect("failed init time zone");
}

pub fn grable_id<S>(id: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode_u64(id))
}

pub fn encode_u64(id: &u64) -> String {
    HARSH.get().unwrap().encode(&[*id])
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

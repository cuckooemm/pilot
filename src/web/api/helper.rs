pub fn page(page: Option<String>, page_size: Option<String>) -> (u64, u64) {
    const PAGE: u64 = 1;
    const PAGE_SIZE: u64 = 20;
    let mut page = page
        .and_then(|p| Some(p.parse::<u64>().unwrap_or(PAGE)))
        .unwrap_or(PAGE);
    if page < PAGE {
        page = PAGE;
    }
    let mut page_size = page_size
        .and_then(|p| Some(p.parse::<u64>().unwrap_or(PAGE_SIZE)))
        .unwrap_or(PAGE_SIZE);
    if page_size < 1 {
        page_size = PAGE_SIZE;
    }
    if page_size > 1000 {
        page_size = 1000;
    }
    if page > 1_000_000_u64 / page_size {
        page = 1_000_000_u64 / page_size;
    }
    (page, page_size)
}

pub fn page_to_limit(page: u64, page_size: u64) -> (u64, u64) {
    return ((page - 1) * page_size, page_size);
}

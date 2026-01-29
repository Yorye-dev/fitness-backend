use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_page: Option<u32>,
}

impl PaginationMeta {
    pub fn new(page: u32, per_page: u32, total: u64) -> Self {
        let total_pages =
            ((total as f64) / (per_page as f64)).ceil() as u32;

        let next_page = if page < total_pages {
            Some(page + 1)
        } else {
            None
        };

        let prev_page = if page > 1 {
            Some(page - 1)
        } else {
            None
        };

        Self {
            page,
            per_page,
            total,
            total_pages,
            next_page,
            prev_page,
        }
    }
}

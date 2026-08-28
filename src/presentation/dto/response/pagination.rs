use serde::Serialize;

#[derive(Debug, Serialize)]
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
    pub fn new(
        page: u32,
        per_page: u32,
        total: u64,
    ) -> Self {
        debug_assert!(
            page > 0,
            "page must be greater than zero"
        );

        debug_assert!(
            per_page > 0,
            "per_page must be greater than zero"
        );

        let total_pages = u32::try_from(
            total.div_ceil(u64::from(per_page)),
        )
        .unwrap_or(u32::MAX);

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

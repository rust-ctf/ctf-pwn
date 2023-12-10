pub trait HasCache {
    fn cache_clear(&mut self);
    fn cache_insert(&mut self, data: &[u8]);
    fn has_cache(&self) -> bool;
}

use super::{AddressMap, address_map::Address, address_map::AddressId};
use lru_cache::LruCache;

pub struct LruCachedAddressMap<A: AddressMap> {
  lru_addresses: LruCache<String, AddressId>,
  address_map: A,
}

impl<A: AddressMap> LruCachedAddressMap<A> {
  pub fn new(cache_size: usize, address_map: A) -> LruCachedAddressMap<A> {
    LruCachedAddressMap {
      lru_addresses: LruCache::new(cache_size),
      address_map,
    }
  }
}

impl<A: AddressMap> AddressMap for LruCachedAddressMap<A> {
  fn get_id(&mut self, address: Address) -> AddressId {
    if let Some(&mut address_id) = self.lru_addresses.get_mut(address) {
      address_id
    } else {
      let address_id = self.address_map.get_id(address);
      self
        .lru_addresses
        .insert(String::from(address), address_id);
      address_id
    }
  }
}

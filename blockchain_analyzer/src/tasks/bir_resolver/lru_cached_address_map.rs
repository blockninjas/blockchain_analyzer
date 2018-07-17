use super::{address_map::Address, address_map::AddressId, AddressMap};
use lru_cache::LruCache;
use std::collections::HashMap;

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
      self.lru_addresses.insert(String::from(address), address_id);
      address_id
    }
  }

  fn get_ids(&mut self, addresses: &[String]) -> HashMap<String, AddressId> {
    let mut cached_addresses: Vec<(String, AddressId)> = vec![];
    let mut addresses_to_load: Vec<String> = vec![];

    for address in addresses {
      if let Some(&mut address_id) = self.lru_addresses.get_mut(address) {
        cached_addresses.push((address.clone(), address_id));
      } else {
        addresses_to_load.push(address.clone());
      }
    }

    let mut address_ids = self.address_map.get_ids(&addresses_to_load);

    for (address, &address_id) in address_ids.iter() {
      self.lru_addresses.insert(address.clone(), address_id);
    }

    address_ids.extend(cached_addresses);
    address_ids
  }
}

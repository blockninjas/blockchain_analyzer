use super::{address_map::Address, address_map::AddressId, AddressMap};
use lru_cache::LruCache;
use std::collections::{HashMap, HashSet};

pub struct LruCachedAddressMap<A: AddressMap> {
    lru_addresses: LruCache<String, AddressId>,
    address_map: A,
    cache_hits: usize,
    cache_misses: usize,
}

impl<A: AddressMap> LruCachedAddressMap<A> {
    pub fn new(cache_size: usize, address_map: A) -> LruCachedAddressMap<A> {
        LruCachedAddressMap {
            lru_addresses: LruCache::new(cache_size),
            address_map,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    pub fn get_cache_misses(&self) -> usize {
        self.cache_misses
    }

    pub fn get_cache_hits(&self) -> usize {
        self.cache_hits
    }
}

impl<A: AddressMap> AddressMap for LruCachedAddressMap<A> {
    fn get_id(&mut self, address: Address) -> AddressId {
        if let Some(&mut address_id) = self.lru_addresses.get_mut(address) {
            self.cache_hits += 1;
            address_id
        } else {
            self.cache_misses += 1;
            let address_id = self.address_map.get_id(address);
            self.lru_addresses.insert(String::from(address), address_id);
            address_id
        }
    }

    fn get_ids(&mut self, addresses: &[String]) -> HashMap<String, AddressId> {
        let addresses: HashSet<String> = addresses.into_iter().cloned().collect();

        let mut cached_addresses = HashMap::<String, AddressId>::new();
        let mut addresses_to_load = Vec::<String>::new();

        for address in addresses.iter() {
            if let Some(&mut address_id) = self.lru_addresses.get_mut(address) {
                cached_addresses.insert(address.clone(), address_id);
            } else {
                addresses_to_load.push(address.clone());
            }
        }

        let mut address_ids = self.address_map.get_ids(&addresses_to_load);

        for (address, &address_id) in address_ids.iter() {
            self.lru_addresses.insert(address.clone(), address_id);
        }

        self.cache_hits += cached_addresses.len();
        self.cache_misses += address_ids.len();

        address_ids.extend(cached_addresses);
        address_ids
    }
}

use crate::models::{Entry, Location};

use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct LocationMatcher {
    latitude_map: BTreeMap<Decimal, Vec<Rc<Entry>>>,
    longitude_map: BTreeMap<Decimal, Vec<Rc<Entry>>>,
}

impl LocationMatcher {
    pub fn new(payloads: &[Rc<Entry>]) -> Self {
        let mut latitude_map: BTreeMap<Decimal, Vec<Rc<Entry>>> = BTreeMap::new();
        let mut longitude_map: BTreeMap<Decimal, Vec<Rc<Entry>>> = BTreeMap::new();

        for item in payloads {
            let lats = latitude_map
                .entry(Decimal(item.location.latitude))
                .or_default();

            lats.push(item.clone());

            let longs = longitude_map
                .entry(Decimal(item.location.longitude))
                .or_default();

            longs.push(item.clone());
        }

        LocationMatcher {
            latitude_map,
            longitude_map,
        }
    }

    pub fn find_matches(&self, location: &Location, acceptable_noise: f64) -> Vec<Rc<Entry>> {
        // 0,0 is junk data, we should ignore it
        if location.latitude == 0.0 && location.longitude == 0.0 {
            return Vec::new();
        }

        let start_lat = Decimal(location.latitude - acceptable_noise);
        let end_lat = Decimal(location.latitude + acceptable_noise);

        let start_long = Decimal(location.longitude - acceptable_noise);
        let end_long = Decimal(location.longitude + acceptable_noise);

        let latitude_matches = self.latitude_map.range(start_lat..end_lat);
        let longitude_matches = self.longitude_map.range(start_long..end_long);

        let exact_lat = self.latitude_map.get_key_value(&Decimal(location.latitude));
        let exact_lat_iter = exact_lat.iter().copied();

        let exact_long = self
            .longitude_map
            .get_key_value(&Decimal(location.longitude));
        let exact_long_iter = exact_long.iter().copied();

        let mut items = HashSet::new();

        for lats in latitude_matches.chain(exact_lat_iter) {
            for lat in lats.1 {
                let address = Rc::as_ptr(lat);
                items.insert(address);
            }
        }

        let mut result = Vec::new();

        for longs in longitude_matches.chain(exact_long_iter) {
            for long in longs.1 {
                let address = Rc::as_ptr(long);
                if items.take(&address).is_some() {
                    result.push(long.clone());
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy)]
struct Decimal(f64);

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Decimal {}

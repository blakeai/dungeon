use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::fmt;
use std::hash::Hash;

pub(crate) fn deserialize_comma_separated<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Debug,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.split(',')
        .map(|item| item.trim()
            .parse::<T>()
            .map_err(|err| map_err::<'de, D, T>(err)))
        .collect()
}

pub(crate) fn deserialize_comma_separated_to_map<'de, D, K>(deserializer: D) -> Result<HashMap<K, u8>, D::Error>
where
    D: Deserializer<'de>,
    K: FromStr + Eq + Hash,
    K::Err: fmt::Debug,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let mut map = HashMap::new();

    for item in s.split(',') {
        let parts: Vec<&str> = item.trim().splitn(2, ' ').collect();
        let (key_str, value) = if parts.len() == 2 {
            (parts[1], parts[0].parse().unwrap_or(1))
        } else {
            (parts[0], 1)
        };

        let key = key_str.parse::<K>().map_err(|err| map_err::<'de, D, K>(err))?;
        map.insert(key, value);
    }

    Ok(map)
}

pub(crate) fn deserialize_comma_separated_to_string_map<'de, D>(
    deserializer: D
) -> Result<HashMap<u8, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<u8, String> = Deserialize::deserialize(deserializer)?;
    let map = map.into_iter()
        .map(|(k, v)| {
            let vec = v.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            Ok((k, vec))
        })
        .collect::<Result<HashMap<u8, Vec<String>>, _>>()?;
    Ok(map)
}


fn map_err<'de, D, T>(err: T::Err) -> D::Error
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Debug,
{
    serde::de::Error::custom(format!("{:?}", err))
}
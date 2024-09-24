#![allow(dead_code)]
// TODO
mod store;

use thiserror::Error;

use rand;

pub struct AccessKey {
    id: String,
    key: String,
}

const ACCESS_KEY_ID_CHARS: &[char; 36] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];
const ACCESS_KEY_CHARS: &[char; 62] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5',
    '6', '7', '8', '9', '0',
];

pub struct AccessKeyService<A: store::Store> {
    pub store: A,
}

#[derive(Error, Debug)]
pub enum GenerateAccessKeyError {
    #[error("access key could not be stored")]
    StorageError(#[from] store::StorageError),
}

impl<A: store::Store> AccessKeyService<A> {
    pub fn gen_access_key(&self) -> Result<AccessKey, GenerateAccessKeyError> {
        let id = gen_rand_str(ACCESS_KEY_ID_CHARS, 16);
        let key = gen_rand_str(ACCESS_KEY_CHARS, 64);
        let access_key = AccessKey { id, key };
        self.store.store_access_key(&access_key)?;
        Ok(access_key)
    }
}

fn gen_rand_str(chars: &[char], len: usize) -> String {
    let set_size = chars.len();

    let mut rand_chars = Vec::with_capacity(len);
    for _ in 0..len {
        let idx = rand::random::<usize>() % set_size;
        rand_chars.push(chars[idx]);
    }

    rand_chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::store;
    use super::*;

    struct StubAccessKeyStore {}

    impl store::Store for StubAccessKeyStore {
        fn store_access_key(&self, _key: &AccessKey) -> Result<(), store::StorageError> {
            Ok(())
        }
    }

    #[test]
    fn test_two_access_keys_are_not_the_same() {
        let stub_access_key_store = StubAccessKeyStore {};
        let service = AccessKeyService {
            store: stub_access_key_store,
        };

        let key1 = service.gen_access_key().unwrap();
        let key2 = service.gen_access_key().unwrap();
        assert_ne!(key1.id, key2.id);
        assert_ne!(key1.key, key2.key);
    }
}

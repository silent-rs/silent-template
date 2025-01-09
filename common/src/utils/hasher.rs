#[allow(dead_code)]
///  MD5 Hasher function
pub fn md5_hasher(value: &str, salt: Option<&str>) -> String {
    let s = value.to_owned() + salt.unwrap_or("");
    let digest = md5::compute(s).to_vec();
    hex::encode(digest).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hasher() {
        let value = "test";
        let salt = "salt";
        let expected_hash = "315240C61218A4A861EC949166A85EF0";
        let actual_hash = md5_hasher(value, Some(salt));
        assert_eq!(expected_hash, actual_hash);
    }
}

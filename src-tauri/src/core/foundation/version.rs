use serde::{Deserialize, Serialize};

/// Stable release versions only. Prerelease/build syntax is deliberately unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseVersion(pub(crate) [u32; 3]);
impl ReleaseVersion {
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        if value.len() > 32 {
            return Err("Release version is invalid");
        }
        let mut numbers = [0; 3];
        let mut parts = value.split('.');
        for number in &mut numbers {
            let part = parts.next().ok_or("Release version is invalid")?;
            if part.is_empty()
                || !part.bytes().all(|b| b.is_ascii_digit())
                || (part.len() > 1 && part.starts_with('0'))
            {
                return Err("Release version is invalid");
            }
            *number = part.parse().map_err(|_| "Release version is invalid")?;
        }
        if parts.next().is_some() {
            return Err("Release version is invalid");
        }
        Ok(Self(numbers))
    }
    pub fn components(self) -> [u32; 3] {
        self.0
    }
}
impl Serialize for ReleaseVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let [major, minor, patch] = self.0;
        serializer.serialize_str(&format!("{major}.{minor}.{patch}"))
    }
}
impl<'de> Deserialize<'de> for ReleaseVersion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Self::parse(&text).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn numeric_order_and_strict_versions() {
        assert!(
            ReleaseVersion::parse("1.10.0").unwrap() > ReleaseVersion::parse("1.9.99").unwrap()
        );
        for text in [
            "1",
            "1.2",
            "1.2.3.4",
            "01.2.3",
            "1.2.-3",
            "1.2.3-beta",
            "1.2.3+build",
            "1.2.4294967296",
            " 1.2.3",
        ] {
            assert!(ReleaseVersion::parse(text).is_err(), "{text}");
        }
    }
}

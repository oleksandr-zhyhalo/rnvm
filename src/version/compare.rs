use semver::{Version, VersionReq};
use std::cmp::Ordering;

pub fn compare_versions(a: &str, b: &str) -> Ordering {
    let ver_a = Version::parse(a).unwrap_or_else(|_| Version::new(0, 0, 0));
    let ver_b = Version::parse(b).unwrap_or_else(|_| Version::new(0, 0, 0));
    ver_b.cmp(&ver_a)
}

pub fn matches_requirement(version: &str, requirement: &str) -> bool {
    if let (Ok(ver), Ok(req)) = (Version::parse(version), VersionReq::parse(requirement)) {
        req.matches(&ver)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert_eq!(compare_versions("1.0.0", "2.0.0"), Ordering::Greater);
        assert_eq!(compare_versions("2.0.0", "1.0.0"), Ordering::Less);
        assert_eq!(compare_versions("1.0.0", "1.0.0"), Ordering::Equal);
    }

    #[test]
    fn test_version_requirement() {
        assert!(matches_requirement("1.2.3", "^1.0.0"));
        assert!(matches_requirement("2.0.0", ">=1.0.0"));
        assert!(!matches_requirement("0.9.0", "^1.0.0"));
    }
}
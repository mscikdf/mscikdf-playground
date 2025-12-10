use mscikdf::passphrase_sealing_util::{MscikdfError, PassphraseSealingUtil};
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let uuid = Uuid::new_v4();
        let pass = b"correct horse battery staple";

        let entropy = PassphraseSealingUtil::seal(&uuid, pass).unwrap();
        assert_eq!(entropy.len(), 32);

        let out = PassphraseSealingUtil::unseal(&entropy, pass).unwrap();
        assert_eq!(uuid, out);
    }

    #[test]
    fn test_wrong_passphrase() {
        let uuid = Uuid::new_v4();
        let entropy = PassphraseSealingUtil::seal(&uuid, b"a").unwrap();

        let res = PassphraseSealingUtil::unseal(&entropy, b"b");
        assert!(matches!(res, Err(MscikdfError::InvalidPassphrase)));
    }

    #[test]
    fn test_tamper() {
        let uuid = Uuid::new_v4();
        let mut entropy = PassphraseSealingUtil::seal(&uuid, b"a").unwrap();

        entropy[0] ^= 0x01;

        let res = PassphraseSealingUtil::unseal(&entropy, b"a");
        assert!(matches!(res, Err(MscikdfError::InvalidPassphrase)));
    }

    // let params = Params::new(
    //     131072, // m = 128 MiB (in KiB)
    //     3,      // t = 3 iterations
    //     4,      // p = 4 lanes
    //     Some(32)
    // ).map_err(|_| MscikdfError::KdfError)?;
    #[test]
    fn test_unseal_performance_meets_security_target() {
        use std::time::{Instant, Duration};
        use mscikdf::passphrase_sealing_util::PassphraseSealingUtil;
        use uuid::Uuid;

        const MIN_SECURITY_TIME: Duration = Duration::from_millis(800);

        let uuid = Uuid::new_v4();
        let sealed = PassphraseSealingUtil::seal(&uuid, b"password_for_timing").unwrap();

        let start = Instant::now();
        let _ = PassphraseSealingUtil::unseal(&sealed, b"password_for_timing").unwrap();
        let duration = start.elapsed();

        assert!(
            duration >= MIN_SECURITY_TIME,
            "Argon2id Unseal time ({:?}) is too fast! Must be >= {:?}",
            duration, MIN_SECURITY_TIME
        );
    }
}

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


}

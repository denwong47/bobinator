/// A trait indicating the struct can return a token key.
pub trait HasToken {
    fn key(&self) -> String;
}

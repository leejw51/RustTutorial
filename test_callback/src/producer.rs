

pub trait Producer {
    fn push(&self, data: &str);
}
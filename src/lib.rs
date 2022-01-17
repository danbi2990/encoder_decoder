
use powerpack::{Item};

pub trait ToItem {
    fn to_item(self, subtitle: &str) -> Item;
}

impl ToItem for String {
    fn to_item(self, subtitle: &str) -> Item {
        let item = powerpack::Item::new(self.clone())
            .subtitle(subtitle)
            .arg(self);
        item
    }
}


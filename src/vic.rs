use fake::{faker::lorem::en::Word, Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Dummy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resource {
    // #[dummy(faker = "16..32")]
    // pub id: String,
    #[dummy(faker = "Word()")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Dummy, Eq, PartialEq, Deserialize, Serialize)]
pub struct Input {
    pub resource_id: String,
    #[dummy(faker = "10..20")]
    pub quantity: u32,
}

#[derive(Clone, Debug, Default, Dummy, Eq, PartialEq, Deserialize, Serialize)]
pub struct Output {
    pub resource_id: String,
    #[dummy(faker = "10..20")]
    pub quantity: u32,
}

#[derive(Clone, Debug, Default, Dummy, Eq, PartialEq, Deserialize, Serialize)]
pub struct Building {
    // #[dummy(faker = "16..32")]
    // pub id: String,
    #[dummy(faker = "Word()")]
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

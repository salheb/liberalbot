// Code here the Tweet struct / object and it's functions

pub struct Tweet{
    id: String,
}

impl Tweet{
    // Mutable access.
    fn id(&mut self) -> &mut String {
        &mut self.id
    }
}
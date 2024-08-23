use serde::{
    Serialize,
    ser,
};


/// A search result
#[derive(Serialize)]
pub struct Verse {
    book: String,
    chapter_nr: i32,
    verse_nr: i32,
    verse: String,
}


pub struct Serializer {
    output: Verse,
}


pub fn to_verse<T>(value: &T) -> Result<Verse, &'static str>
where
    T: Serialize,
{
    let mut val = Verse{
        book: String::new(),
        chapter_nr: 0,
        verse_nr: 0,
        verse: String::new(),
    };
    value.serialize(&mut val)?;
    Ok(val)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = String;

    type SerializeSeq = Self;

    fn serialize_i32(self, v: i32) -> Result<(), &'static str> {
        self.
    }

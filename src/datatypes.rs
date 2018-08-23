
#[derive(Debug)]
pub struct NRRDData {
    pub title: String,
    pub headers: Vec<NRRDHeader>,
    pub data: NRRDBody,
}

#[derive(Debug)]
pub struct NRRDBody {
    // TODO
}

#[derive(Debug)]
pub enum NRRDHeader {
    NDimensions(i32),
    DataFormat(String), // NOTE: may want to make this an enum
    Space(String),      // NOTE: may want to have a list of strings, or enums, instead,
                        // depending on interpretation of field
    Sizes(Vec<i32>),
    SpaceDirections(Vec< Vec<f64> >),
    Kinds(Vec<String>),
    Endian(EndianType),
    Encoding(EncodingType),
    SpaceOrigin(Vec<f64>),
    MeasurementFrame(Vec< Vec<i32> >),
    Comment
}

#[derive(Debug)]
pub enum EndianType {
    BigEndian,
    LittleEndian
}

#[derive(Debug)]
pub enum EncodingType {
    GZip, // TODO: more encoding types
}

#[derive(Debug)]
pub struct NRRDParseError {
    pub err: String,
}

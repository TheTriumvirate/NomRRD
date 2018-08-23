use datatypes::*;
use helperfuncs::*;
use nom::types::CompleteStr;

// parser function: runs parsers on the string and returns a result
pub fn parse_nrrd(input: &str) -> Result<NRRDData, NRRDParseError> {
    let res = nrrd(CompleteStr(input));
    match res {
        Ok((rem, nrrdinfo)) => {
            if rem != CompleteStr("") {
                return Err(NRRDParseError { err: format!("Expected end of input, but found '{}'", rem) });
            }
            return Ok(nrrdinfo);
        },
        Err(err) => {
            return Err(NRRDParseError { err: format!("Parse error: {:?}" , err) });
        },
    }
}

named!(nrrd<CompleteStr, NRRDData>, do_parse!(
        title: consume_line
     >> headers: nrrd_parse_headers
     >> data: nrrd_parse_data
     >> (NRRDData { title: title.to_string(), headers, data })));

named!(nrrd_parse_headers<CompleteStr, Vec<NRRDHeader> >, map_res!(many0!(nrrd_any_header), remove_comments));

named!(nrrd_any_header<CompleteStr, NRRDHeader>, do_parse!(
        header: alt!(
            do_parse!(ws!(tag!("type:"))      >> fmt:   nrrd_any_type     >> ( NRRDHeader::DataFormat(fmt.to_string()) ))
          | do_parse!(ws!(tag!("dimension:")) >> dim:   parse_integer     >> ( NRRDHeader::NDimensions(dim) ))
          | do_parse!(ws!(tag!("space:"))     >> space: take_until!("\n") >> ( NRRDHeader::Space(space.to_string()) ))
          | do_parse!(ws!(tag!("sizes:"))     >> sizes: parse_integers    >> ( NRRDHeader::Sizes(sizes) )) // TODO: parse properly
          | do_parse!(ws!(tag!("space directions:")) >> directions: parse_optional_double_vec >> (NRRDHeader::SpaceDirections(directions) ))
          | do_parse!(ws!(tag!("kinds:"))     >> kinds: nrrd_parse_kinds  >> ( NRRDHeader::Kinds(kinds) ))
          | do_parse!(ws!(tag!("endian:"))    >> endian: nrrd_any_endian  >> ( NRRDHeader::Endian(endian) ))
          | do_parse!(ws!(tag!("encoding:"))  >> enc: nrrd_any_encoding   >> ( NRRDHeader::Encoding(enc) ))
          | do_parse!(ws!(tag!("space origin:")) >> origin: parse_double_vec >> ( NRRDHeader::SpaceOrigin(origin) ))
          | do_parse!(ws!(tag!("measurement frame:")) >> frame: parse_optional_integer_vec >> ( NRRDHeader::MeasurementFrame(frame) ))
          | do_parse!(ws!(tag!("#"))          >> take_until!("\n")        >> ( NRRDHeader::Comment ))
        )
        // TODO: more headers
        >> char!('\n')
        >> ( header )));

named!(nrrd_parse_kinds<CompleteStr, Vec<String> >, separated_nonempty_list!(char!(' '), map_res!(nrrd_any_kind, from_complete_str)));

named!(nrrd_parse_data<CompleteStr, NRRDBody>, value!(NRRDBody {}, char!('\n')));
named!(nrrd_any_type<CompleteStr, CompleteStr>, alt!(tag!("float"))); // TODO: add more types
named!(nrrd_any_kind<CompleteStr, CompleteStr>, alt!(tag!("3D-matrix") | tag!("domain")));
named!(nrrd_any_endian<CompleteStr, EndianType>, alt!(value!(EndianType::LittleEndian, tag!("little")) | value!(EndianType::BigEndian, tag!("big"))));
named!(nrrd_any_encoding<CompleteStr, EncodingType>, alt!(value!(EncodingType::GZip, tag!("gzip"))));
named!(parse_optional_double_vec<CompleteStr, Vec< Vec<f64> > >, separated_nonempty_list!(char!(' '), none_or_double_vec));
named!(parse_optional_integer_vec<CompleteStr, Vec< Vec<i32> > >, separated_nonempty_list!(char!(' '), none_or_integer_vec));
named!(none_or_double_vec<CompleteStr, Vec<f64> >, alt!(none_double | parse_double_vec));
named!(none_or_integer_vec<CompleteStr, Vec<i32> >, alt!(none_integer | parse_integer_vec));
named!(none_double<CompleteStr, Vec<f64> >, value!(vec![], tag!("none")));
named!(none_integer<CompleteStr, Vec<i32> >, value!(vec![], tag!("none")));
named!(parse_double_vec<CompleteStr, Vec<f64> >, delimited!(char!('('), separated_nonempty_list!(char!(','), parse_double), char!(')')));
named!(parse_integer_vec<CompleteStr, Vec<i32> >, delimited!(char!('('), separated_nonempty_list!(char!(','), parse_integer), char!(')')));
named!(parse_double<CompleteStr, f64>, map_res!(take_while!(is_double_digit), from_double));
named!(parse_integers<CompleteStr, Vec<i32> >, separated_nonempty_list!(tag!(" "), parse_integer));
named!(parse_integer<CompleteStr, i32>, map_res!(take_while!(is_digit), from_int));

named!(consume_line<CompleteStr, CompleteStr>, take_until_and_consume!("\n"));

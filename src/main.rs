#[macro_use]
extern crate nom;

// compile the modules
mod datatypes;
mod combinators;
mod helperfuncs;

use combinators::parse_nrrd;

fn main() {
    // TODO: look into alternative values/data types for each of the headers, etc, and check for all headers
    // TODO: figure out how NRRD data is stored
    // TODO: read NRRD data
    let input: &str = // multiline string
"NRRD005
# Complete NRRD file format specification at:
# http://teem.sourceforge.net/nrrd/format.html
type: float
dimension: 4
space: right-anterior-superior
sizes: 9 144 144 85
space directions: none (-1.6667,0,0) (0,-1.6667,0) (0,0,-1.7)
kinds: 3D-matrix domain domain domain
endian: little
encoding: gzip
space origin: (119.169,119.169,71.4)
measurement frame: (-1,0,0) (0,-1,0) (0,0,-1)

";
    let res = parse_nrrd(input);

    println!("{:#?}", res);
}

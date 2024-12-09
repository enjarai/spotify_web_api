use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b'/');

/// Escape a string for usage as a single URL path component.
pub fn path_escaped(input: &'_ str) -> impl std::fmt::Display + '_ {
    utf8_percent_encode(input, PATH_SEGMENT_ENCODE_SET)
}

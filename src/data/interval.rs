// ////////////////////////////////////////////////////////////////////////////////
// // DATA STRUCTURE
// ////////////////////////////////////////////////////////////////////////////////

// #[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
// pub struct Interval {
//     pub line: u32,
//     pub column: u32,
// }

// ////////////////////////////////////////////////////////////////////////////////
// // TRAIT FUNCTION
// ////////////////////////////////////////////////////////////////////////////////

// impl Default for Interval {
//     fn default() -> Self {
//         Self { line: 0, column: 0 }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// // PUBLIC FUNCTIONS
// ////////////////////////////////////////////////////////////////////////////////

// impl Interval {
//     pub fn new_as_u32(line: u32, column: u32) -> Self {
//         Self { line, column }
//     }

//     pub fn new_as_span(span: Span) -> Self {
//         Self {
//             line: span.location_line(),
//             column: span.get_column() as u32,
//         }
//     }
// }
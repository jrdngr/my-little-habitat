pub type Color = [f32; 4];

// pub struct Color {
//     r: f32,
//     g: f32,
//     b: f32,
//     a: f32,
// }

// impl Color {
//     pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
//         Color {
//             r: r,
//             g: g,
//             b: b,
//             a: a,
//         }
//     }

//     pub fn from_array(color: [f32; 4]) -> Color {
//         Color {
//             r: color[0],
//             g: color[1],
//             b: color[2],
//             a: color[3],
//         }
//     }

//     pub fn as_array(&self) -> [f32; 4] {
//         [self.r, self.g, self.b, self.a]
//     }
// }
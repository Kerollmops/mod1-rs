use std::str::{self, FromStr};
use nom::{IResult, multispace, digit, line_ending, eof};

named!(u32_digit<u32>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(
    surface_point<&[u8], SurfacePoint>,
    delimited!(
        char!('('),
        chain!(
            many0!(multispace) ~
            x: u32_digit ~
            delimited!(many0!(multispace), char!(','), many0!(multispace)) ~
            y: u32_digit ~
            delimited!(many0!(multispace), char!(','), many0!(multispace)) ~
            z: u32_digit ~ // FIXME i32 can be used ???
            many0!(multispace) ,
            || { SurfacePoint{ x: x, y: y, z: z } }
        ),
        char!(')')
    )
);

named!(
    surface_points<&[u8], Vec<SurfacePoint> >,
    chain!(
        sp: many1!(
            chain!(
                many0!(multispace) ~
                sp: surface_point ~
                many0!(multispace),
                || sp
            )
        ) ~
        eof ,
        || sp
    )
);

#[derive(Clone, Copy, Debug)]
pub struct SurfacePoint {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

#[derive(Debug)]
pub struct SurfacePoints(Vec<SurfacePoint>);

impl SurfacePoints {
    // FIXME real error
    pub fn from_buffer(buffer: &[u8]) -> Result<SurfacePoints, ()> {
        match surface_points(buffer) {
            IResult::Done(_rest, surface_points) => Ok(SurfacePoints(surface_points)),
            _ => Err(())
        }
    }
}

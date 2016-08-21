use std::ops::{Deref, DerefMut};
use std::str::{self, FromStr};
use nom::{IResult, multispace, digit, eof, line_ending, not_line_ending};

named!(u32_digit<u32>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(comment,
    delimited!(
        char!(';'),
        not_line_ending,
        line_ending
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
            z: u32_digit ~
            many0!(multispace) ,
            || SurfacePoint{ x: x, y: y, z: z }
        ),
        char!(')')
    )
);

named!(
    surface_points<&[u8], Vec<SurfacePoint> >,
    chain!(
        sps: fold_many1!(
            alt_complete!(
                chain!(
                    many0!(multispace) ~
                    sp: surface_point ~
                    many0!(multispace),
                    || Some(sp)
                ) |
                map!(comment, |_| None)
            ),
            Vec::new(),
            |mut acc: Vec<_>, sp_opt| {
                if let Some(sp) = sp_opt { acc.push(sp) }
                acc
            }
        ) ~
        eof,
        || sps
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

impl Deref for SurfacePoints {
    type Target = Vec<SurfacePoint>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SurfacePoints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SurfacePoints {
    // FIXME real error
    pub fn from_buffer(buffer: &[u8]) -> Result<SurfacePoints, ()> {
        match surface_points(buffer) {
            IResult::Done(_rest, ref surface_points) if surface_points.is_empty() => Err(()),
            IResult::Done(_rest, surface_points) => Ok(SurfacePoints(surface_points)),
            _ => Err(())
        }
    }
}

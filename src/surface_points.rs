use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::{self, FromStr};
use nom::{IResult, Err, multispace, digit, eof, line_ending, not_line_ending};

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

pub enum ParsingError<'a> {
    NoPoint,
    Unreachable,
    NomError(Err<&'a [u8], u32>),
}

const MIN_STR_ERR_LEN: usize = 5;

impl<'a> fmt::Display for ParsingError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsingError::NoPoint => write!(f, "No point found."),
            ParsingError::Unreachable => write!(f, "Unreachable."),
            ParsingError::NomError(Err::Position(_, pos)) => {
                let len = if pos.len() < MIN_STR_ERR_LEN {
                    pos.len()
                } else { MIN_STR_ERR_LEN };

                write!(f, "\n{}", String::from_utf8_lossy(&pos[..len]))
            },
            ParsingError::NomError(ref err) => write!(f, "{}", err)
        }
    }
}

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
    pub fn from_buffer(buffer: &[u8]) -> Result<SurfacePoints, ParsingError> {
        match surface_points(buffer) {
            IResult::Done(_, ref sps) if sps.is_empty() => Err(ParsingError::NoPoint),
            IResult::Done(_, sps) => Ok(SurfacePoints(sps)),
            IResult::Error(err) => Err(ParsingError::NomError(err)),
            IResult::Incomplete(_) => Err(ParsingError::Unreachable)
        }
    }
}

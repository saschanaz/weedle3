/*
 * Copyright (c) 2014-2019 Geoffroy Couprie
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 * WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Copied from https://github.com/Geal/nom/blob/294ffb3d9e0ade2c3b7ddfff52484b6d643dcce1/src/branch/mod.rs
// to allow more branches, because sub-branching made generate_keywords_enum needlessly complex.

macro_rules! succ (
  (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
  (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
  (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
  (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
  (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
  (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
  (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
  (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
  (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
  (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
  (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
  (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
  (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
  (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
  (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
  (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
  (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
  (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
  (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
  (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
  (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
  (21, $submac:ident ! ($($rest:tt)*)) => ($submac!(22, $($rest)*));
  (22, $submac:ident ! ($($rest:tt)*)) => ($submac!(23, $($rest)*));
  (23, $submac:ident ! ($($rest:tt)*)) => ($submac!(24, $($rest)*));
  (24, $submac:ident ! ($($rest:tt)*)) => ($submac!(25, $($rest)*));
  (25, $submac:ident ! ($($rest:tt)*)) => ($submac!(26, $($rest)*));
  (26, $submac:ident ! ($($rest:tt)*)) => ($submac!(27, $($rest)*));
  (27, $submac:ident ! ($($rest:tt)*)) => ($submac!(28, $($rest)*));
  (28, $submac:ident ! ($($rest:tt)*)) => ($submac!(29, $($rest)*));
  (29, $submac:ident ! ($($rest:tt)*)) => ($submac!(30, $($rest)*));
  (30, $submac:ident ! ($($rest:tt)*)) => ($submac!(31, $($rest)*));
  (31, $submac:ident ! ($($rest:tt)*)) => ($submac!(32, $($rest)*));
  (32, $submac:ident ! ($($rest:tt)*)) => ($submac!(33, $($rest)*));
  (33, $submac:ident ! ($($rest:tt)*)) => ($submac!(34, $($rest)*));
  (34, $submac:ident ! ($($rest:tt)*)) => ($submac!(35, $($rest)*));
  (35, $submac:ident ! ($($rest:tt)*)) => ($submac!(36, $($rest)*));
  (36, $submac:ident ! ($($rest:tt)*)) => ($submac!(37, $($rest)*));
  (37, $submac:ident ! ($($rest:tt)*)) => ($submac!(38, $($rest)*));
  (38, $submac:ident ! ($($rest:tt)*)) => ($submac!(39, $($rest)*));
  (39, $submac:ident ! ($($rest:tt)*)) => ($submac!(40, $($rest)*));
  (40, $submac:ident ! ($($rest:tt)*)) => ($submac!(41, $($rest)*));
  (41, $submac:ident ! ($($rest:tt)*)) => ($submac!(42, $($rest)*));
  (42, $submac:ident ! ($($rest:tt)*)) => ($submac!(43, $($rest)*));
  (43, $submac:ident ! ($($rest:tt)*)) => ($submac!(44, $($rest)*));
  (44, $submac:ident ! ($($rest:tt)*)) => ($submac!(45, $($rest)*));
  (45, $submac:ident ! ($($rest:tt)*)) => ($submac!(46, $($rest)*));
  (46, $submac:ident ! ($($rest:tt)*)) => ($submac!(47, $($rest)*));
  (47, $submac:ident ! ($($rest:tt)*)) => ($submac!(48, $($rest)*));
  (48, $submac:ident ! ($($rest:tt)*)) => ($submac!(49, $($rest)*));
  (49, $submac:ident ! ($($rest:tt)*)) => ($submac!(50, $($rest)*));
  (50, $submac:ident ! ($($rest:tt)*)) => ($submac!(51, $($rest)*));
  (51, $submac:ident ! ($($rest:tt)*)) => ($submac!(52, $($rest)*));
  (52, $submac:ident ! ($($rest:tt)*)) => ($submac!(53, $($rest)*));
  (53, $submac:ident ! ($($rest:tt)*)) => ($submac!(54, $($rest)*));
  (54, $submac:ident ! ($($rest:tt)*)) => ($submac!(55, $($rest)*));
  (55, $submac:ident ! ($($rest:tt)*)) => ($submac!(56, $($rest)*));
  (56, $submac:ident ! ($($rest:tt)*)) => ($submac!(57, $($rest)*));
  (57, $submac:ident ! ($($rest:tt)*)) => ($submac!(58, $($rest)*));
  (58, $submac:ident ! ($($rest:tt)*)) => ($submac!(59, $($rest)*));
  (59, $submac:ident ! ($($rest:tt)*)) => ($submac!(60, $($rest)*));
  (60, $submac:ident ! ($($rest:tt)*)) => ($submac!(61, $($rest)*));
  (61, $submac:ident ! ($($rest:tt)*)) => ($submac!(62, $($rest)*));
  (62, $submac:ident ! ($($rest:tt)*)) => ($submac!(63, $($rest)*));
  (63, $submac:ident ! ($($rest:tt)*)) => ($submac!(64, $($rest)*));
  (64, $submac:ident ! ($($rest:tt)*)) => ($submac!(65, $($rest)*));
  (65, $submac:ident ! ($($rest:tt)*)) => ($submac!(66, $($rest)*));
  (66, $submac:ident ! ($($rest:tt)*)) => ($submac!(67, $($rest)*));
  (67, $submac:ident ! ($($rest:tt)*)) => ($submac!(68, $($rest)*));
  (68, $submac:ident ! ($($rest:tt)*)) => ($submac!(69, $($rest)*));
  (69, $submac:ident ! ($($rest:tt)*)) => ($submac!(70, $($rest)*));
  (70, $submac:ident ! ($($rest:tt)*)) => ($submac!(71, $($rest)*));
  (71, $submac:ident ! ($($rest:tt)*)) => ($submac!(72, $($rest)*));
  (72, $submac:ident ! ($($rest:tt)*)) => ($submac!(73, $($rest)*));
  (73, $submac:ident ! ($($rest:tt)*)) => ($submac!(74, $($rest)*));
  (74, $submac:ident ! ($($rest:tt)*)) => ($submac!(75, $($rest)*));
  (75, $submac:ident ! ($($rest:tt)*)) => ($submac!(76, $($rest)*));
  (76, $submac:ident ! ($($rest:tt)*)) => ($submac!(77, $($rest)*));
  (77, $submac:ident ! ($($rest:tt)*)) => ($submac!(78, $($rest)*));
  (78, $submac:ident ! ($($rest:tt)*)) => ($submac!(79, $($rest)*));
  (79, $submac:ident ! ($($rest:tt)*)) => ($submac!(80, $($rest)*));
  (80, $submac:ident ! ($($rest:tt)*)) => ($submac!(81, $($rest)*));
  (81, $submac:ident ! ($($rest:tt)*)) => ($submac!(82, $($rest)*));
);

use nom::error::ErrorKind;
use nom::error::ParseError;
use nom::{Err, IResult, Parser};

pub trait Alt<I, O, E> {
    /// Tests each parser in the tuple and returns the result of the first one that succeeds
    fn choice(&mut self, input: I) -> IResult<I, O, E>;
}

pub fn alt<I: Clone, O, E: ParseError<I>, List: Alt<I, O, E>>(
    mut l: List,
) -> impl FnMut(I) -> IResult<I, O, E> {
    move |i: I| l.choice(i)
}

macro_rules! alt_trait(
  ($first:ident $second:ident $($id: ident)+) => (
    alt_trait!(__impl $first $second; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
    alt_trait_impl!($($current)*);

    alt_trait!(__impl $($current)* $head; $($id)+);
  );
  (__impl $($current:ident)*; $head:ident) => (
    alt_trait_impl!($($current)*);
    alt_trait_impl!($($current)* $head);
  );
);

macro_rules! alt_trait_impl(
  ($($id:ident)+) => (
    impl<
      Input: Clone, Output, Error: ParseError<Input>,
      $($id: Parser<Input, Output, Error>),+
    > Alt<Input, Output, Error> for ( $($id),+ ) {

      fn choice(&mut self, input: Input) -> IResult<Input, Output, Error> {
        match self.0.parse(input.clone()) {
          Err(Err::Error(e)) => alt_trait_inner!(1, self, input, e, $($id)+),
          res => res,
        }
      }
    }
  );
);

macro_rules! alt_trait_inner(
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident $($id:ident)+) => (
    match $self.$it.parse($input.clone()) {
      Err(Err::Error(e)) => {
        let err = $err.or(e);
        succ!($it, alt_trait_inner!($self, $input, err, $($id)+))
      }
      res => res,
    }
  );
  ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident) => (
    Err(Err::Error(Error::append($input, ErrorKind::Alt, $err)))
  );
);

alt_trait!(I1 I2 I3 I4 I5 I6 I7 I8 I9 I10 I11 I12 I13 I14 I15 I16 I17 I18 I19 I20 I21 I22 I23 I24 I25 I26 I27 I28 I29 I30 I31 I32 I33 I34 I35 I36 I37 I38 I39 I40 I41 I42 I43 I44 I45 I46 I47 I48 I49 I50 I51 I52 I53 I54 I55 I56 I57 I58 I59 I60 I61 I62 I63 I64 I65 I66 I67 I68 I69 I70 I71 I72 I73 I74 I75 I76 I77 I78 I79 I80 I81);

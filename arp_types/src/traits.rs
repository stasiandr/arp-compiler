use std::ops::Range;

use ariadne::{Report, ReportBuilder};

use crate::sources::Source;


pub type RB<'builder> = ReportBuilder<'builder, (String, Range<usize>)>;

pub trait AppendToReport<Err> {
    fn append_to_report<'source>(&self, builder: RB<'source>, source: &'source Source) -> RB<'source>;
    fn build_report<'source>(errors: &'source [Self], source: &'source Source) -> Report<'source, (String, Range<usize>)> where Self: Sized;
}
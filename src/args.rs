//! CLI argument definitions.

use clap::Parser;

/// Convert current time to milliseconds since Unix epoch.
///
/// Returns the number of milliseconds elapsed since 1970-01-01T00:00:00Z
/// (the Unix epoch) for the current UTC time.
///
/// # AI Coding Agent Instructions
///
/// This tool is useful when you need a numeric timestamp in milliseconds,
/// for example to append as a cache-buster query parameter (?ts=<ms>),
/// to generate unique identifiers, or to measure elapsed wall-clock time
/// between two invocations.
///
/// ## Usage Patterns
///
/// Get current epoch milliseconds:
///     ep2ms
///
/// Get epoch milliseconds with the ISO timestamp that produced it:
///     ep2ms -v
///
/// Use as a cache-buster in URLs:
///     echo "https://example.com/style.css?ts=$(ep2ms)"
///
/// Measure elapsed time between two events:
///     START=$(ep2ms)
///     # ... do work ...
///     END=$(ep2ms)
///     echo "Elapsed: $((END - START)) ms"
///
/// ## Output Format
///
/// stdout: a single line containing the integer milliseconds since epoch.
/// stderr (with -v/--verbose): the ISO 8601 / RFC 3339 UTC timestamp used.
///
/// ## Exit Codes
///
///   0  Success
///   1  Error
#[derive(Parser, Debug)]
#[command(name = "ep2ms")]
#[command(disable_version_flag = true)]
#[command(verbatim_doc_comment)]
pub struct Args {
    /// Show version information
    #[arg(short = 'V', long)]
    pub version: bool,

    /// Show the ISO 8601 timestamp used to produce the result
    #[arg(short, long)]
    pub verbose: bool,
}

/// Parse command line arguments.
pub fn parse() -> Args {
    Args::parse()
}

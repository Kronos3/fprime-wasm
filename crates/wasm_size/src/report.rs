//! The size report and the comparison between two of them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write;

use crate::wasm::Sections;

/// One measured wasm module.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Binary {
    pub name: String,
    pub total: usize,
    pub code: usize,
    pub data: usize,
}

/// A whole run's measurements, ordered by name so the JSON is diffable.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    pub binaries: Vec<Binary>,
}

impl Report {
    pub fn new(mut binaries: Vec<Binary>) -> Self {
        binaries.sort_by(|a, b| a.name.cmp(&b.name));

        Self { binaries }
    }

    /// The measured module named `name`, if it was measured.
    fn get(&self, name: &str) -> Option<&Binary> {
        self.binaries.iter().find(|binary| binary.name == name)
    }

    /// Absolute sizes, smallest first, so the fixed cost is the top row and the
    /// table reads as a ladder.
    pub fn markdown(&self) -> String {
        let mut rows = self.binaries.clone();
        rows.sort_by_key(|binary| binary.total);

        let mut out = String::from("| Binary | Total | Code | Data |\n|---|--:|--:|--:|\n");
        for binary in &rows {
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} |",
                binary.name, binary.total, binary.code, binary.data
            );
        }

        out
    }
}

/// One row of a comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub name: String,
    /// `None` when the binary is new or was removed.
    pub base: Option<usize>,
    pub head: Option<usize>,
}

impl Change {
    /// Positive means the binary grew.
    pub fn delta(&self) -> i64 {
        self.head.unwrap_or(0) as i64 - self.base.unwrap_or(0) as i64
    }
}

/// Compare two reports, joined on binary name.
///
/// Sorted by delta descending, so a regression is the first thing read. Binaries
/// present in only one side are kept: a benchmark that stopped building is
/// exactly the thing a size report should not hide.
pub fn compare(base: &Report, head: &Report) -> Vec<Change> {
    let mut names: BTreeMap<&str, ()> = BTreeMap::new();
    for binary in base.binaries.iter().chain(&head.binaries) {
        names.insert(binary.name.as_str(), ());
    }

    let mut changes: Vec<Change> = names
        .into_keys()
        .map(|name| Change {
            name: name.to_string(),
            base: base.get(name).map(|binary| binary.total),
            head: head.get(name).map(|binary| binary.total),
        })
        .collect();

    // Descending by delta, then by name so equal rows are stable.
    changes.sort_by(|a, b| b.delta().cmp(&a.delta()).then(a.name.cmp(&b.name)));
    changes
}

fn cell(size: Option<usize>) -> String {
    match size {
        Some(size) => size.to_string(),
        None => "—".to_string(),
    }
}

fn signed(delta: i64) -> String {
    match delta {
        0 => "0".to_string(),
        delta => format!("{delta:+}"),
    }
}

/// Render a comparison as markdown, with unchanged rows folded away.
pub fn markdown(changes: &[Change]) -> String {
    let changed: Vec<&Change> = changes.iter().filter(|c| c.delta() != 0).collect();

    let base_total: usize = changes.iter().filter_map(|c| c.base).sum();
    let head_total: usize = changes.iter().filter_map(|c| c.head).sum();
    let total_delta = head_total as i64 - base_total as i64;

    let mut out = String::new();

    let _ = writeln!(
        out,
        "**Total** {} → {} bytes ({}){}\n",
        base_total,
        head_total,
        signed(total_delta),
        match total_delta {
            0 => ", no change".to_string(),
            _ => format!(", {:+.2}%", 100.0 * total_delta as f64 / base_total as f64),
        }
    );

    if changed.is_empty() {
        let _ = writeln!(out, "No binary changed size.");

        return out;
    }

    let _ = writeln!(out, "| Binary | Base | Head | Δ | Δ% |");
    let _ = writeln!(out, "|---|--:|--:|--:|--:|");

    for change in &changed {
        let percent = match (change.base, change.head) {
            (Some(base), Some(_)) if base > 0 => {
                format!("{:+.1}%", 100.0 * change.delta() as f64 / base as f64)
            }
            (None, Some(_)) => "new".to_string(),
            (Some(_), None) => "removed".to_string(),
            _ => "—".to_string(),
        };

        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} |",
            change.name,
            cell(change.base),
            cell(change.head),
            signed(change.delta()),
            percent
        );
    }

    let unchanged = changes.len() - changed.len();
    if unchanged > 0 {
        let _ = writeln!(out, "\n{unchanged} unchanged.");
    }

    out
}

impl Binary {
    pub fn from_sections(name: impl Into<String>, sections: Sections) -> Self {
        Self {
            name: name.into(),
            total: sections.total,
            code: sections.code,
            data: sections.data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn binary(name: &str, total: usize) -> Binary {
        Binary {
            name: name.to_string(),
            total,
            code: total / 2,
            data: total / 4,
        }
    }

    #[test]
    fn orders_a_report_by_name() {
        let report = Report::new(vec![binary("b", 2), binary("a", 1)]);

        assert_eq!(
            report.binaries.iter().map(|b| b.name.as_str()).collect::<Vec<_>>(),
            ["a", "b"]
        );
    }

    #[test]
    fn puts_the_worst_regression_first() {
        let base = Report::new(vec![binary("small", 100), binary("big", 100)]);
        let head = Report::new(vec![binary("small", 105), binary("big", 200)]);

        let changes = compare(&base, &head);

        assert_eq!(changes[0].name, "big");
        assert_eq!(changes[0].delta(), 100);
        assert_eq!(changes[1].delta(), 5);
    }

    #[test]
    fn reports_a_shrink_as_negative() {
        let base = Report::new(vec![binary("a", 200)]);
        let head = Report::new(vec![binary("a", 150)]);

        assert_eq!(compare(&base, &head)[0].delta(), -50);
    }

    /// A benchmark that stopped building must not silently vanish from the table.
    #[test]
    fn keeps_added_and_removed_binaries() {
        let base = Report::new(vec![binary("gone", 100)]);
        let head = Report::new(vec![binary("fresh", 100)]);

        let changes = compare(&base, &head);
        let table = markdown(&changes);

        assert!(table.contains("`gone`"), "{table}");
        assert!(table.contains("removed"), "{table}");
        assert!(table.contains("`fresh`"), "{table}");
        assert!(table.contains("new"), "{table}");
    }

    #[test]
    fn folds_away_unchanged_rows() {
        let base = Report::new(vec![binary("same", 100), binary("moved", 100)]);
        let head = Report::new(vec![binary("same", 100), binary("moved", 120)]);

        let table = markdown(&compare(&base, &head));

        assert!(table.contains("`moved`"), "{table}");
        assert!(!table.contains("`same`"), "{table}");
        assert!(table.contains("1 unchanged."), "{table}");
    }

    #[test]
    fn says_so_when_nothing_moved() {
        let report = Report::new(vec![binary("a", 100)]);
        let table = markdown(&compare(&report, &report));

        assert!(table.contains("No binary changed size."), "{table}");
        assert!(table.contains("no change"), "{table}");
    }

    #[test]
    fn round_trips_through_json() {
        let report = Report::new(vec![binary("a", 100), binary("b", 200)]);
        let json = serde_json::to_string(&report).expect("should serialize");

        assert_eq!(
            serde_json::from_str::<Report>(&json).expect("should deserialize"),
            report
        );
    }
}

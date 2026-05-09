use serde::Serialize;

pub const VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Capability {
    pub name: &'static str,
    pub access: &'static str,
    pub risk: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BriefManifest {
    pub v: &'static str,
    pub loop_steps: [&'static str; 4],
    pub caps: Vec<Capability>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BenchmarkSummary {
    pub count: usize,
    pub min_ms: Option<f64>,
    pub max_ms: Option<f64>,
    pub avg_ms: Option<f64>,
}

pub fn capabilities() -> Vec<Capability> {
    vec![
        Capability { name: "manifest", access: "r", risk: "none" },
        Capability { name: "doctor", access: "r", risk: "none" },
        Capability { name: "observe", access: "r", risk: "screen-read" },
        Capability { name: "watch", access: "r", risk: "screen-read" },
        Capability { name: "wait-change", access: "r", risk: "screen-read" },
        Capability { name: "click", access: "w", risk: "ui-action" },
        Capability { name: "type", access: "w", risk: "text-entry" },
        Capability { name: "paste", access: "w", risk: "text-entry" },
        Capability { name: "browser", access: "w", risk: "network-navigation" },
        Capability { name: "sequence", access: "w", risk: "compound-action" },
        Capability { name: "mcp", access: "w", risk: "tool-server" },
        Capability { name: "brain", access: "w", risk: "metadata-write" },
        Capability { name: "bench", access: "r", risk: "timing" },
    ]
}

pub fn brief_manifest() -> BriefManifest {
    BriefManifest {
        v: VERSION,
        loop_steps: ["doctor", "observe", "act", "verify"],
        caps: capabilities(),
    }
}

pub fn benchmark_summary(samples: &[f64]) -> BenchmarkSummary {
    if samples.is_empty() {
        return BenchmarkSummary { count: 0, min_ms: None, max_ms: None, avg_ms: None };
    }

    let mut min = samples[0];
    let mut max = samples[0];
    let mut sum = 0.0;
    for sample in samples {
        min = min.min(*sample);
        max = max.max(*sample);
        sum += sample;
    }

    BenchmarkSummary {
        count: samples.len(),
        min_ms: Some(round_2(min)),
        max_ms: Some(round_2(max)),
        avg_ms: Some(round_2(sum / samples.len() as f64)),
    }
}

fn round_2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brief_manifest_keeps_agent_contract() {
        let manifest = brief_manifest();
        assert_eq!(manifest.v, "0.1.0");
        assert!(manifest.caps.iter().any(|cap| cap.name == "observe" && cap.access == "r"));
        assert!(manifest.caps.iter().any(|cap| cap.name == "sequence" && cap.access == "w"));
    }

    #[test]
    fn benchmark_summary_reports_basic_stats() {
        let summary = benchmark_summary(&[10.0, 20.0, 30.0]);
        assert_eq!(summary.count, 3);
        assert_eq!(summary.min_ms, Some(10.0));
        assert_eq!(summary.max_ms, Some(30.0));
        assert_eq!(summary.avg_ms, Some(20.0));
    }

    #[test]
    fn benchmark_summary_handles_empty_samples() {
        let summary = benchmark_summary(&[]);
        assert_eq!(summary.count, 0);
        assert_eq!(summary.avg_ms, None);
    }
}

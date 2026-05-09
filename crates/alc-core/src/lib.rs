use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub count: usize,
    pub min_ms: Option<f64>,
    pub max_ms: Option<f64>,
    pub avg_ms: Option<f64>,
}

pub fn capabilities() -> Vec<Capability> {
    vec![
        Capability {
            name: "manifest",
            access: "r",
            risk: "none",
        },
        Capability {
            name: "doctor",
            access: "r",
            risk: "none",
        },
        Capability {
            name: "observe",
            access: "r",
            risk: "screen-read",
        },
        Capability {
            name: "watch",
            access: "r",
            risk: "screen-read",
        },
        Capability {
            name: "wait-change",
            access: "r",
            risk: "screen-read",
        },
        Capability {
            name: "click",
            access: "w",
            risk: "ui-action",
        },
        Capability {
            name: "type",
            access: "w",
            risk: "text-entry",
        },
        Capability {
            name: "paste",
            access: "w",
            risk: "text-entry",
        },
        Capability {
            name: "browser",
            access: "w",
            risk: "network-navigation",
        },
        Capability {
            name: "sequence",
            access: "w",
            risk: "compound-action",
        },
        Capability {
            name: "mcp",
            access: "w",
            risk: "tool-server",
        },
        Capability {
            name: "brain",
            access: "w",
            risk: "metadata-write",
        },
        Capability {
            name: "bench",
            access: "r",
            risk: "timing",
        },
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
        return BenchmarkSummary {
            count: 0,
            min_ms: None,
            max_ms: None,
            avg_ms: None,
        };
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaemonEnvelope {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(flatten)]
    pub request: DaemonRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "kebab-case")]
pub enum DaemonRequest {
    Ping,
    Manifest,
    BenchSummary { samples: Vec<f64> },
    Input { steps: Vec<InputStep> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum InputStep {
    Move {
        dx: i32,
        dy: i32,
    },
    Goto {
        x: i32,
        y: i32,
    },
    Click {
        #[serde(default)]
        button: Option<String>,
        #[serde(default)]
        x: Option<i32>,
        #[serde(default)]
        y: Option<i32>,
        #[serde(default)]
        delay_ms: Option<u64>,
    },
    Scroll {
        vertical: i32,
        #[serde(default)]
        horizontal: Option<i32>,
    },
    Key {
        name: String,
    },
    Hotkey {
        chord: String,
    },
    Type {
        text: String,
    },
}

impl InputStep {
    pub fn action_name(&self) -> &'static str {
        match self {
            Self::Move { .. } => "move",
            Self::Goto { .. } => "goto",
            Self::Click { .. } => "click",
            Self::Scroll { .. } => "scroll",
            Self::Key { .. } => "key",
            Self::Hotkey { .. } => "hotkey",
            Self::Type { .. } => "type",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaemonResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn handle_daemon_request(envelope: DaemonEnvelope) -> DaemonResponse {
    let id = envelope.id;
    match envelope.request {
        DaemonRequest::Ping => {
            success_response(id, serde_json::json!({"pong": true, "v": VERSION}))
        }
        DaemonRequest::Manifest => success_response(
            id,
            serde_json::to_value(brief_manifest()).expect("manifest serializes"),
        ),
        DaemonRequest::BenchSummary { samples } => success_response(
            id,
            serde_json::to_value(benchmark_summary(&samples)).expect("summary serializes"),
        ),
        DaemonRequest::Input { .. } => DaemonResponse {
            id,
            ok: false,
            result: None,
            error: Some("input requires daemon runtime".to_string()),
        },
    }
}

fn success_response(id: Option<String>, result: serde_json::Value) -> DaemonResponse {
    DaemonResponse {
        id,
        ok: true,
        result: Some(result),
        error: None,
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
        assert!(
            manifest
                .caps
                .iter()
                .any(|cap| cap.name == "observe" && cap.access == "r")
        );
        assert!(
            manifest
                .caps
                .iter()
                .any(|cap| cap.name == "sequence" && cap.access == "w")
        );
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

    #[test]
    fn daemon_ping_preserves_id() {
        let response = handle_daemon_request(DaemonEnvelope {
            id: Some("req-1".to_string()),
            request: DaemonRequest::Ping,
        });
        assert!(response.ok);
        assert_eq!(response.id.as_deref(), Some("req-1"));
        assert_eq!(response.result.unwrap()["pong"], true);
    }

    #[test]
    fn daemon_request_parses_json_line() {
        let request: DaemonEnvelope =
            serde_json::from_str(r#"{"id":"b1","cmd":"bench-summary","samples":[10,20,30]}"#)
                .unwrap();
        let response = handle_daemon_request(request);
        assert_eq!(response.id.as_deref(), Some("b1"));
        assert_eq!(response.result.unwrap()["avg_ms"], 20.0);
    }

    #[test]
    fn daemon_input_request_parses_batch() {
        let request: DaemonEnvelope = serde_json::from_str(
            r#"{"id":"i1","cmd":"input","steps":[{"action":"move","dx":2,"dy":-3},{"action":"key","name":"esc"}]}"#,
        )
        .unwrap();
        match request.request {
            DaemonRequest::Input { steps } => {
                assert_eq!(steps.len(), 2);
                assert_eq!(steps[0].action_name(), "move");
                assert_eq!(steps[1].action_name(), "key");
            }
            other => panic!("unexpected request: {other:?}"),
        }
    }

    #[test]
    fn core_rejects_input_without_daemon_runtime() {
        let response = handle_daemon_request(DaemonEnvelope {
            id: Some("i1".to_string()),
            request: DaemonRequest::Input {
                steps: vec![InputStep::Move { dx: 1, dy: 1 }],
            },
        });
        assert!(!response.ok);
        assert_eq!(response.id.as_deref(), Some("i1"));
        assert_eq!(
            response.error.as_deref(),
            Some("input requires daemon runtime")
        );
    }
}

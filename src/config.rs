use crate::Result;
use serde_json::{Value, json};
use std::time::Duration;

pub const DEFAULT_ADDR: &str = "ws://127.0.0.1:8765";
pub const COMMANDS: &[&str] = &[
    "watch", "version", "snapshot", "capture", "stream", "status", "wait", "assert", "diagnose",
    "top", "diff",
];

#[derive(Clone, Debug)]
pub struct Config {
    pub command: String,
    pub json: bool,
    pub addr: String,
    pub timeout: Duration,
    pub duration: Duration,
    pub wait_for: String,
    pub name: String,
    pub level: String,
    pub metric: String,
    pub max_p95: f64,
    pub min_fps: f64,
    pub max_frame: f64,
    pub kind: String,
    pub path: String,
    pub prefix: String,
    pub sort: String,
    pub limit: i64,
    pub raw: bool,
    pub traces: bool,
    pub out: String,
    pub file: String,
    pub budget: String,
    pub positional: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            command: "watch".into(),
            json: false,
            addr: DEFAULT_ADDR.into(),
            timeout: Duration::from_secs(10),
            duration: Duration::ZERO,
            wait_for: "snapshot".into(),
            name: String::new(),
            level: String::new(),
            metric: String::new(),
            max_p95: -1.,
            min_fps: -1.,
            max_frame: -1.,
            kind: String::new(),
            path: String::new(),
            prefix: String::new(),
            sort: "p95".into(),
            limit: 0,
            raw: false,
            traces: false,
            out: "gd-observe-cli-capture".into(),
            file: String::new(),
            budget: String::new(),
            positional: Vec::new(),
        }
    }
}

pub const HELP: &str = "gd-observe-cli: Godot observability CLI and Ratatui dashboard\n\nUsage: gd-observe-cli [watch|snapshot|capture|stream|status|wait|assert|diagnose|top|diff|version] [options]\n\n  --addr, --url URL       WebSocket endpoint (default ws://127.0.0.1:8765)\n  --timeout DURATION     Machine command timeout (default 10s)\n  --kind KIND            timer, gauge or counter\n  --path PATH            Exact metric path\n  --path-prefix PREFIX   Metric path prefix\n  --sort FIELD           p95, total, max, avg, p50, p99, latest or value\n  --limit COUNT          Maximum returned metrics\n  --raw, --traces        Include raw samples / recent traces\n  --file FILE            Read an offline snapshot (- reads stdin)\n  --budget FILE          Assert a TOML budget\n  --metric PATH          Metric path or key for assertions\n  --max-p95-usec NUMBER  --min-fps NUMBER  --max-frame-usec NUMBER\n  --for TYPE             Wait for snapshot, event, log or trace\n  --name NAME            Event name to wait for\n  --level LEVEL          Log level to wait for\n  --duration DURATION    Optional capture duration\n  --out DIRECTORY       Capture destination (default gd-observe-cli-capture)\n\nEndpoint: --addr/--url overrides GDOBS_URL in the process environment.\nAuthentication: GDOBS_AUTH_TOKEN in the process environment only.\nMachine results: --json selects finite commands; snapshot/status/analysis already emit JSON. watch and stream retain their terminal/stream protocols.\nDashboard: click tabs and rows; mouse wheel scrolls; Ctrl+C exits.\n";

impl Config {
    pub fn parse(args: &[String]) -> Result<Self> {
        let mut cfg = Self {
            addr: std::env::var("GDOBS_URL")
                .ok()
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| DEFAULT_ADDR.into()),
            ..Self::default()
        };
        let mut args = args.iter().peekable();
        if args.peek().is_some_and(|v| COMMANDS.contains(&v.as_str())) {
            cfg.command = args.next().unwrap().clone();
        }
        while let Some(arg) = args.next() {
            if arg == "--" {
                cfg.positional.extend(args.cloned());
                break;
            }
            if !arg.starts_with('-') {
                cfg.positional.push(arg.clone());
                continue;
            }
            let (key, inline) = arg
                .trim_start_matches('-')
                .split_once('=')
                .map_or((arg.trim_start_matches('-'), None), |(k, v)| (k, Some(v)));
            if matches!(key, "raw" | "traces" | "json") {
                let value = match inline.unwrap_or("true").to_ascii_lowercase().as_str() {
                    "true" | "t" | "1" => true,
                    "false" | "f" | "0" => false,
                    _ => return Err(format!("invalid boolean for --{key}").into()),
                };
                match key {
                    "raw" => cfg.raw = value,
                    "json" => cfg.json = value,
                    _ => cfg.traces = value,
                }

                continue;
            }
            let value = inline
                .map(str::to_owned)
                .or_else(|| args.next().cloned())
                .ok_or_else(|| format!("--{key} needs a value"))?;
            match key {
                "addr" | "url" => {
                    cfg.addr = value;
                }
                "timeout" => cfg.timeout = duration(&value)?,
                "duration" => cfg.duration = duration(&value)?,
                "for" => cfg.wait_for = value,
                "name" => cfg.name = value,
                "level" => cfg.level = value,
                "metric" => cfg.metric = value,
                "max-p95-usec" => cfg.max_p95 = value.parse()?,
                "min-fps" => cfg.min_fps = value.parse()?,
                "max-frame-usec" => cfg.max_frame = value.parse()?,
                "kind" => cfg.kind = value,
                "path" => cfg.path = value,
                "path-prefix" => cfg.prefix = value,
                "sort" => cfg.sort = value,
                "limit" => cfg.limit = value.parse()?,
                "out" => cfg.out = value,
                "file" => cfg.file = value,
                "budget" => cfg.budget = value,
                _ => return Err(format!("unknown option --{key}").into()),
            }
        }
        if ![cfg.max_p95, cfg.min_fps, cfg.max_frame]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err("assertion thresholds must be finite".into());
        }
        Ok(cfg)
    }

    pub fn snapshot_options(&self) -> Value {
        let mut options = json!({"include_raw_samples":self.raw,"include_traces":self.traces,"include_runtime":true,"summary_limit":20});
        for (key, value) in [
            ("kind", &self.kind),
            ("path", &self.path),
            ("path_prefix", &self.prefix),
            ("sort", &self.sort),
        ] {
            if !value.is_empty() {
                options[key] = json!(value);
            }
        }
        if self.limit > 0 {
            options["limit"] = json!(self.limit);
        }
        options
    }
}

pub fn duration(value: &str) -> Result<Duration> {
    if value.is_empty() {
        return Err("duration must not be empty".into());
    }
    if value == "0" {
        return Ok(Duration::ZERO);
    }
    let mut rest = value;
    let mut seconds = 0.;
    while !rest.is_empty() {
        let count = rest
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .ok_or("duration needs a unit")?;
        let amount: f64 = rest[..count].parse()?;
        rest = &rest[count..];
        let (unit, scale) = [
            ("ns", 1e-9),
            ("us", 1e-6),
            ("µs", 1e-6),
            ("μs", 1e-6),
            ("ms", 1e-3),
            ("s", 1.),
            ("m", 60.),
            ("h", 3600.),
        ]
        .into_iter()
        .find(|(unit, _)| rest.starts_with(unit))
        .ok_or("invalid duration unit")?;
        seconds += amount * scale;
        rest = &rest[unit.len()..];
    }
    if !seconds.is_finite() || seconds < 0. || seconds > i64::MAX as f64 / 1e9 {
        return Err("duration must be finite and nonnegative".into());
    }
    Ok(Duration::try_from_secs_f64(seconds)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_preserve_aliases_defaults_and_interspersed_top_arguments() {
        let args = [
            "top",
            "timers",
            "--url=ws://localhost:7777",
            "-limit",
            "3",
            "--raw=false",
            "--traces",
            "--timeout",
            "1m2.5s",
        ]
        .map(str::to_owned);
        let config = Config::parse(&args).unwrap();
        assert_eq!(config.addr, "ws://localhost:7777");
        assert_eq!(config.positional, ["timers"]);
        assert_eq!(config.timeout, Duration::from_millis(62500));
        assert_eq!(config.snapshot_options()["include_raw_samples"], false);
        assert_eq!(config.snapshot_options()["include_traces"], true);
        assert_eq!(config.snapshot_options()["limit"], 3);
        for retired in ["--env", "--project"] {
            assert!(Config::parse(&[retired.into(), "missing-file".into()]).is_err());
        }
        for invalid in ["", "NaN", "-1s", "1", "999999999999h"] {
            assert!(duration(invalid).is_err(), "{invalid}");
        }
        assert_eq!(duration("50ms").unwrap(), Duration::from_millis(50));
        assert!(Config::parse(&["--min-fps=NaN".into()]).is_err());
    }
}

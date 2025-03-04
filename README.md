# info-pingrtt

## Overview

`info-pingrtt` is a lightweight, configurable Rust-based utility for monitoring network connectivity and ping response times. It provides a simple JSON-formatted output with a world globe emoji, making it easy to integrate with status bars and monitoring tools.

## Features

- 🌐 Ping status monitoring for any hostname
- 🚦 Configurable performance thresholds
- 🌍 Consistent world globe emoji output
- 📊 JSON output for easy integration with status bars and monitoring tools

## Prerequisites

- Rust programming language (stable version)
- Cargo package manager
- `ping` utility installed on the system

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/its-a-unixsystem/info-pingrtt.git
   cd info-pingrtt
   ```

2. Install dependencies:
   ```bash
   cargo build --release
   ```

## Configuration

The script can be configured using environment variables:

| Environment Variable   | Description                     | Default Value       |
|-----------------------|--------------------------------|---------------------|
| `PING_HOSTNAME`       | Target host to ping             | `www.google.com`    |
| `PING_MID_THRESHOLD`  | Threshold for "mid" performance | `50` (milliseconds) |
| `PING_BAD_THRESHOLD`  | Threshold for "bad" performance | `150` (milliseconds)|

### Performance Thresholds

- **Good**: < `PING_MID_THRESHOLD` ms
- **Mid**: Between `PING_MID_THRESHOLD` and `PING_BAD_THRESHOLD` ms
- **Bad**: > `PING_BAD_THRESHOLD` ms
- **Down**: Ping fails or no response

## Usage Examples

### Basic Usage
```bash
cargo run
```

### Custom Hostname
```bash
PING_HOSTNAME=example.com cargo run
```

### Custom Thresholds
```bash
PING_MID_THRESHOLD=75 PING_BAD_THRESHOLD=200 cargo run
```

## Output Format

The script outputs a JSON object with the following structure:

```json
{
  "text": "🌍",          // Always a world globe emoji
  "tooltip": "42",       // Ping round-trip time in milliseconds
  "class": "good"        // Status: "good", "mid", "bad", or "down"
}
```

## Integration

This script is designed to be easily integrated with status bars, monitoring dashboards, or other tools that can parse JSON output.

### Example i3status/swaybar Integration

Add to your i3status or swaybar configuration to display ping status:

```json
{
    "full_text": "%output_of_info-pingrtt%"
}
```

For waybar please use something like this in the config:
```
"custom/network": {
    "exec": "~/bin/info-pingrtt.sh",
    "interval": 10,
    "return-type": "json",
    "format": "{}"
},
```
and for the style.css
```
#custom-network {
    padding-left: 3px;
    border-color: #073642;
    padding-right: 3px
}
#custom-network.good {
    color: #859900;
}
#custom-network.mid {
    color: #b58900;
}
#custom-network.bad {
    color: #dc322f;
}
#custom-network.down {
    color: red;
    animation-name: blink-critical;
    animation-duration: 2s;    
}
```
## Dependencies

- `serde`: JSON serialization
- `serde_json`: JSON processing
- `regex`: Pattern matching for RTT extraction

## Building for Release

```bash
cargo build --release
```

The compiled binary will be located at `target/release/info-pingrtt`

<<<<<<< HEAD
## License

Apache 2.0
=======
## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

[Specify your license here, e.g., MIT, Apache 2.0]
>>>>>>> origin/main

## Disclaimer

This tool is for network diagnostics and monitoring. Always ensure you have proper permissions when monitoring network resources.

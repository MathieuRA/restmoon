# RESTMoon

🦀 A lightweight HTTP proxy written in Rust that analyzes REST API traffic and provides real-time performance metrics.

## Features

- **Raw TCP/HTTP Implementation**: Built from scratch without heavy HTTP frameworks
- **Flexible Routing**: Support for both environment-based and header-based destination routing
- **Zero Configuration**: Works out of the box with sensible defaults
- **Lightweight**: Minimal dependencies, maximum performance

## Quick Start

### Installation

```bash
git clone https://github.com/mathieura/restmoon
cd restmoon
cargo build --release
```

### Basic Usage

#### Option 1: Fixed Destination (Environment Variable)
```bash
export DESTINATION="localhost:8080" # Optional
export PROXY_PORT="8081"  # Optional, defaults to 8081
cargo run
```

#### Option 2: Dynamic Destination (Headers)
```bash
cargo run

# Then make requests with destination header:
curl -H "X-Proxy-Destination: api.example.com" http://localhost:8081/api/users
```

### Example Output

```
🚀 Proxy Analyzer starting (Raw TCP/HTTP)...
   Listening on: 127.0.0.1:8081
   Default destination: localhost:8080

📊 Request Analytics:
----------------------------------------
[19:44:07] GET /objects -> http://api.restful-api.dev:80 (250.57ms) [Response body: 1.2 KB]
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DESTINATION` | Default target server (host:port) | None |
| `PROXY_PORT` | Port for the proxy to listen on | 8081 |

### Header-based Routing

For dynamic routing, include the destination in your request headers:

```http
GET /api/endpoint HTTP/1.1
Host: localhost:8081
X-Proxy-Destination: backend.example.com:8080
```

The proxy will:
1. Check for `X-Proxy-Destination` header first
2. Fall back to `DESTINATION` environment variable
3. Return 500 error if neither is provided

## Use Cases

- **API Performance Monitoring**: Track response times and identify slow endpoints
- **Development Debugging**: Monitor API calls during frontend development
- **Load Testing Analysis**: Observe API behavior under different loads
- **Microservices Debugging**: Route and analyze traffic between services

## Architecture

```
Client → [Proxy Analyzer :8081] → Target API Server
                ↓
             Metrics
```

The proxy:
1. Accepts incoming TCP connections
2. Parses HTTP requests manually  
3. Forwards requests to the target server
4. Measures response times and logs analytics
5. Returns responses to the client

## Contributing

Contributions are welcome! This project is great for learning Rust networking concepts.

## Future Features

- [x] Mesure response time 
- [x] Add payload size (body/response)
- [ ] Support keep-alive (source -> proxy -> destination)
- [X] Parse the HTTP response to get status code
- [ ] Support all HTTP verbs
- [ ] Support SSE enpoints
- [ ] Metrics persistence (SQLite/JSON)
- [ ] WebUI dashboard for metrics visualization

## License

This project is licensed under either of MIT license [LICENSE-MIT](https://opensource.org/license/mit)

## Acknowledgments

Built with ❤️ in Rust. Perfect for learning low-level HTTP handling and network programming concepts.

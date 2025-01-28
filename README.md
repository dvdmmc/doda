# DODA - P2P OSINT Network Node

A peer-to-peer OSINT (Open Source Intelligence) network node implementation in Rust, featuring both GUI and CLI interfaces.

## Architecture Overview

DODA is built with a modular architecture, separating concerns into distinct components:

```
doda/
├── src/
│   ├── main.rs         # Application entry point, GUI/CLI implementations
│   ├── identity/       # Identity management and user profiles
│   ├── network/        # P2P networking and communication
│   ├── crypto/         # Cryptographic operations
│   ├── data/          # Data structures and verification
│   └── search/        # Search engine functionality
```

### Core Components

1. **Identity Management** (`identity/`)
   - Handles user identity creation and management
   - Provides recovery key export/import functionality
   - Manages persistent storage of identity information

2. **Network Layer** (`network/`)
   - Implements P2P networking using libp2p
   - Manages peer connections and communication
   - Handles network protocol implementation

3. **Cryptography** (`crypto/`)
   - Provides cryptographic operations
   - Handles secure key generation and management
   - Implements encryption/decryption functionality

4. **Data Management** (`data/`)
   - Defines core data structures
   - Implements data verification mechanisms
   - Manages data persistence and retrieval

5. **Search Engine** (`search/`)
   - Implements search functionality
   - Handles data indexing and retrieval
   - Provides search operation interfaces

### User Interfaces

The application provides two interface modes:

1. **GUI Mode** (Default)
   - Built using egui framework
   - Provides a modern, user-friendly interface
   - Features intuitive identity management controls

2. **CLI Mode**
   - Traditional command-line interface
   - Accessible via `--cli` flag
   - Provides full feature parity with GUI mode

## Technical Stack

- **Language**: Rust
- **GUI Framework**: egui/eframe
- **P2P Networking**: libp2p
- **Async Runtime**: tokio
- **Serialization**: serde, serde_json
- **Cryptography**: ring, blake3
- **CLI Parsing**: clap

## Usage

### Running the Application

1. GUI Mode (Default):
```bash
cargo run
```

2. CLI Mode:
```bash
cargo run -- --cli
```

### Identity Management

The application supports the following operations:
- Generating new identities (with optional names)
- Exporting recovery keys for backup
- Importing identities using recovery keys

## Development

### Building from Source

1. Clone the repository
2. Install Rust (if not already installed)
3. Build the project:
```bash
cargo build
```

### Project Structure

- `main.rs`: Application entry point and UI implementations
- `identity/mod.rs`: Identity management implementation
- `network/mod.rs`: P2P networking implementation
- `crypto/mod.rs`: Cryptographic operations
- `data/mod.rs`: Data structure definitions
- `search/mod.rs`: Search functionality implementation

## License

[License Information]
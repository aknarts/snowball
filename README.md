# Snowball

A web-based financial literacy simulation game built with Rust and WebAssembly.

[![CI](https://github.com/aknarts/snowball/workflows/CI/badge.svg)](https://github.com/aknarts/snowball/actions)
[![Deploy](https://github.com/aknarts/snowball/workflows/Deploy/badge.svg)](https://github.com/aknarts/snowball/actions)

## Play Online

Visit [https://aknarts.github.io/snowball/](https://aknarts.github.io/snowball/) to play the game.

## About

Snowball teaches financial literacy through an interactive simulation where you manage a simulated life through monthly planning cycles. Learn about:

- Investing and compound growth
- Budgeting and expense management
- Behavioral finance (happiness, burnout, lifestyle creep)
- Tax-advantaged retirement accounts
- The path to Financial Independence (FIRE)

### International by Design

Snowball supports multiple financial systems:
- **Czech Republic** (initial implementation)
- **USA** (planned)
- **UK** (planned)

Each market has its own tax rules, retirement accounts, and financial vehicles accurately simulated.

## Technology Stack

- **Language**: Rust
- **Frontend**: Yew (WebAssembly)
- **Architecture**: Client-only (no backend server)
- **Storage**: Browser Local Storage
- **Styling**: Tailwind CSS
- **Build Tool**: Trunk
- **Deployment**: GitHub Pages

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) - Install with `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

### Building and Running

```bash
# Clone the repository
git clone https://github.com/aknarts/snowball.git
cd snowball

# Start development server with hot reload (for local development)
cd snowball_web
trunk serve
# Opens at http://127.0.0.1:8080

# Build for testing/verification
cd snowball_web
trunk build

# Build for production
cd snowball_web
trunk build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

**Note**: This is a client-only application. All game logic runs in the browser via WebAssembly, with game saves stored in browser Local Storage. No backend server is required.

### Project Structure

```
snowball/
├── fin_engine/          # Core financial engine (library)
│   ├── src/
│   │   ├── market.rs    # MarketProfile trait
│   │   └── markets/     # Country implementations (czech, usa, uk)
│   └── Cargo.toml
├── snowball_web/        # Yew frontend (binary)
│   ├── src/
│   │   ├── main.rs
│   │   └── app.rs
│   ├── index.html
│   └── Trunk.toml
└── Cargo.toml           # Workspace configuration
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p fin_engine

# Run tests for specific market
cargo test -p fin_engine --features czech
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Pre-commit Hook

The project automatically formats code before each commit. The git pre-commit hook is set up during initialization.

## Game Design

The game uses a hybrid "Plan & Sim" approach with three monthly phases:

1. **Monthly Planning** - Allocate income to budget, savings, and investments
2. **Execution Sim** - Watch the month unfold with random events
3. **Monthly Ledger** - Review your progress and outcomes

### Current Features (v0.1 - Early Development)

**Implemented:**
- ✅ Three-phase game loop (Planning → Execution → Review)
- ✅ Initialization with market and starting job selection
- ✅ Career system with 5 job levels and auto-progression
- ✅ Housing market with 10 Czech housing options (5,000 - 57,000 Kč/month)
- ✅ Moving costs (2 months security deposit + moving fees)
- ✅ Essential budget (Food & Groceries) with 3,500 Kč/month minimum
- ✅ Discretionary budgets (Lifestyle, Health, Transportation, Education, Other)
- ✅ Monthly financial settlement with Czech tax calculations
- ✅ Starting cash based on first job salary

**In Progress:**
- ⏳ Execution phase day-by-day simulation
- ⏳ Review phase monthly summary
- ⏳ Random events and emergencies
- ⏳ Investment accounts (savings, DIP, ETFs)
- ⏳ Behavioral mechanics (happiness, burnout, lifestyle creep)

See [AGENTS.md](./AGENTS.md) for detailed game design and roadmap, and [CLAUDE.md](./CLAUDE.md) for development guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Author

Milan Šťastný

## Acknowledgments

Built with Claude Code and powered by the Rust/WASM ecosystem.

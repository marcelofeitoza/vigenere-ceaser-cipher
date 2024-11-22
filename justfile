# List all available commands
default:
    @just --list

# Run Rust cipher breaker
rust-break:
    @echo "Running Rust cipher breaker..."
    cd src/rust && cargo run

# Run Go frequency analysis
go-analyze:
    @echo "Running Go frequency analysis..."
    cd src/go/frequency && go run main.go
    cd src/go/monoalphabetic/caesar && go run main.go

# Run Python post-processing
python-process:
    @echo "Running Python analysis..."
    python src/python/main.py

# Clean output files
clean:
    @echo "Cleaning output files..."
    rm -f data/output/*.csv
    rm -f data/*.csv

# Run complete analysis pipeline
analyze: clean rust-break go-analyze python-process
    @echo "Complete analysis finished!"
    @echo "Check data/output/ for results"

# Format all code
fmt:
    cd src/rust && cargo fmt
    cd src/go/frequency && go fmt .
    cd src/go/monoalphabetic && go fmt ./...
    python3 -m black src/python/

# Build all
build: 
    @echo "Building Rust components..."
    cd src/rust && cargo build --release
    @echo "Building Go components..."
    cd src/go/frequency && go build
    cd src/go/monoalphabetic/caesar && go build

run-ciphers:
    @echo "Running Rust ciphers..."
    # cd src/rust && cargo run
    @echo "Running Python analysis..."
    python3 src/python/main.py

# Regex Dummy Data Generator

## Project Overview

This project is a comprehensive tool for generating random dummy data based on regex patterns and exporting it to multiple data formats. The system is designed to help developers, testers, and data analysts quickly create realistic test data for various use cases.

## Core Concept

- **Input**: User-defined regex patterns
- **Process**: Generate random data matching the regex patterns
- **Output**: Export data to multiple standardized formats

## Architecture

### Technology Stack

#### Backend/Core Logic (Rust)
- **Regex Engine**: Parse and validate regex patterns
- **Data Generation**: Create random data matching regex patterns
- **Format Conversion**: Transform data into various output formats
- **Performance Optimization**: Handle large datasets efficiently

#### Frontend Interfaces
1. **Web Interface (Svelte + Flowbite)**
    - Static website deployment using SvelteKit
    - Flowbite Svelte Admin Dashboard template as base
    - Tailwind CSS + Flowbite components for modern UI
    - User-friendly GUI for pattern input with form validation
    - Real-time preview and data validation
    - Responsive design for desktop and mobile
    - Download generated files with progress indicators

2. **CLI Tool (Rust/Python)**
    - Command-line interface for automation
    - Batch processing capabilities
    - Integration with CI/CD pipelines
    - Server environment support

### Supported Output Formats (Phase 1)

1. **CSV (Comma-Separated Values)**
    - Standard spreadsheet format
    - Headers and data rows
    - Proper escaping and quoting

2. **TSV (Tab-Separated Values)**
    - Tab-delimited format
    - Alternative to CSV for certain systems

3. **JSON (JavaScript Object Notation)**
    - Structured data format
    - Nested objects and arrays support
    - API testing data

4. **XML (eXtensible Markup Language)**
    - Hierarchical markup format
    - Schema validation support
    - Legacy system compatibility

### Future Format Expansion (Phase 2+)
- Excel (XLSX, XLS)
- Database Dump (SQL)
- Parquet
- JSONL/NDJSON
- YAML
- Avro
- ORC
- Protocol Buffers
- HDF5
- JSON-LD

## Key Features

### Regex Pattern Support
- Standard regex syntax compliance
- Pattern validation and error handling
- Complex pattern combinations
- Character classes, quantifiers, and groups

### Data Generation Options
- Configurable dataset size
- Seed-based reproducible generation
- Field relationships and dependencies
- Data type inference from patterns

### Export Capabilities
- Multi-format simultaneous export
- Format-specific optimizations
- Compression options
- Streaming for large datasets

## Use Cases

### Development & Testing
- API endpoint testing
- Database population
- Load testing data
- Integration testing

### Data Analysis
- Algorithm testing
- Visualization prototyping
- Statistical analysis samples
- Machine learning datasets

### Education & Training
- Data format learning
- Regex pattern practice
- Database design examples
- ETL pipeline testing

## Technical Requirements

### Core Dependencies (Rust)
```toml
[dependencies]
regex = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.1"
quick-xml = "0.31"
clap = "4.0"  # For CLI
tokio = "1.0"  # For async operations
```

### Web Interface Dependencies (Svelte + Flowbite)
```json
{
  "dependencies": {
    "svelte": "^4.0.0",
    "@sveltejs/kit": "^1.0.0",
    "flowbite-svelte": "^0.44.0",
    "flowbite": "^2.0.0",
    "tailwindcss": "^3.0.0",
    "@tailwindcss/forms": "^0.5.0",
    "@tailwindcss/typography": "^0.5.0"
  }
}
```

## Project Structure

```
regex-dummy-generator/
├── core/                   # Rust core library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── regex_engine.rs
│   │   ├── data_generator.rs
│   │   ├── exporters/
│   │   │   ├── mod.rs
│   │   │   ├── csv.rs
│   │   │   ├── json.rs
│   │   │   ├── xml.rs
│   │   │   └── tsv.rs
│   │   └── cli.rs
│   └── Cargo.toml
├── web/                    # Svelte web interface (Flowbite-based)
│   ├── src/
│   │   ├── app.html
│   │   ├── routes/
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── PatternInput.svelte
│   │   │   │   ├── DataPreview.svelte
│   │   │   │   ├── ExportOptions.svelte
│   │   │   │   └── ResultsTable.svelte
│   │   │   └── stores/
│   │   └── app.css
│   ├── static/
│   ├── package.json
│   ├── tailwind.config.js
│   └── vite.config.js
├── cli/                    # CLI wrapper
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
└── docs/
    ├── README.md
    ├── API.md
    └── examples/
```

## UI/UX Design Framework

### Flowbite Svelte Integration
The web interface will be built using **Flowbite Svelte Admin Dashboard** template, providing:

#### Design System
- **Modern Tailwind CSS**: Utility-first CSS framework for rapid development
- **Flowbite Components**: 60+ pre-built, accessible UI components
- **Consistent Theming**: Built-in dark/light mode support
- **Responsive Design**: Mobile-first approach with tablet and desktop optimization

#### Key UI Components for Data Generation Tool
- **Form Elements**: Advanced input fields, validation states, and form layouts
- **Data Display**: Tables, cards, and lists for showing generated data
- **Navigation**: Sidebar navigation and breadcrumb components
- **Modals & Overlays**: Settings dialogs and help documentation
- **Progress Indicators**: Real-time feedback for data generation process
- **Download Actions**: Button components with loading states

#### Layout Structure
```
┌─────────────────────────────────────────┐
│ Header (Logo, Navigation, Theme Toggle) │
├─────────────────────────────────────────┤
│ Main Dashboard Layout                   │
│ ┌─────────────────┬─────────────────────┤
│ │ Pattern Input   │ Live Preview        │
│ │ Panel          │ & Settings Panel     │
│ │ - Regex Field   │ - Data Preview      │
│ │ - Validation    │ - Format Options    │
│ │ - Examples      │ - Export Settings   │
│ └─────────────────┼─────────────────────┤
│                   │ Results Dashboard   │
│                   │ - Generated Data    │
│                   │ - Download Actions  │
│                   │ - Statistics        │
└───────────────────┴─────────────────────┘
```

### Phase 1: MVP (Core + 4 Formats)
- Basic regex parsing
- CSV, TSV, JSON, XML export
- Simple web interface
- Basic CLI tool

### Phase 2: Enhanced Features
- Complex regex patterns
- Data relationships
- Performance optimizations
- Advanced CLI options

### Phase 3: Format Expansion
- Additional formats (Excel, SQL, etc.)
- Streaming support
- Cloud deployment
- API endpoints

---

## Claude Development WBS (Work Breakdown Structure)

### 1. Project Setup & Planning
**Duration**: 1-2 sessions
- [ ] 1.1 Repository initialization
- [ ] 1.2 Cargo workspace setup
- [ ] 1.3 Basic project structure creation
- [ ] 1.4 Development environment configuration

### 2. Core Rust Library Development
**Duration**: 4-6 sessions

#### 2.1 Regex Engine Module
- [ ] 2.1.1 Regex pattern validation
- [ ] 2.1.2 Pattern parsing and compilation
- [ ] 2.1.3 Error handling for invalid patterns
- [ ] 2.1.4 Unit tests for regex functionality

#### 2.2 Data Generation Engine
- [ ] 2.2.1 Random data generation from regex
- [ ] 2.2.2 Configurable dataset size
- [ ] 2.2.3 Seed-based reproducible generation
- [ ] 2.2.4 Performance optimization for large datasets

#### 2.3 Export System Architecture
- [ ] 2.3.1 Generic exporter trait definition
- [ ] 2.3.2 Data structure standardization
- [ ] 2.3.3 Format-agnostic data pipeline

### 3. Format Exporters Implementation
**Duration**: 3-4 sessions

#### 3.1 CSV Exporter
- [ ] 3.1.1 Basic CSV writing functionality
- [ ] 3.1.2 Header handling
- [ ] 3.1.3 Proper escaping and quoting
- [ ] 3.1.4 Large dataset streaming

#### 3.2 TSV Exporter
- [ ] 3.2.1 Tab-delimited format implementation
- [ ] 3.2.2 TSV-specific optimizations

#### 3.3 JSON Exporter
- [ ] 3.3.1 JSON object/array structure
- [ ] 3.3.2 Nested data support
- [ ] 3.3.3 Pretty printing options

#### 3.4 XML Exporter
- [ ] 3.4.1 XML structure generation
- [ ] 3.4.2 Proper escaping and encoding
- [ ] 3.4.3 Schema compliance options

### 4. CLI Tool Development
**Duration**: 2-3 sessions
- [ ] 4.1 Command-line argument parsing
- [ ] 4.2 Interactive pattern input
- [ ] 4.3 Batch processing capabilities
- [ ] 4.4 Help documentation and examples

### 5. Web Interface Development
**Duration**: 4-5 sessions

#### 5.1 Svelte Setup & Basic UI (Flowbite-based)
- [ ] 5.1.1 SvelteKit project initialization with Flowbite template
- [ ] 5.1.2 Flowbite Svelte Admin Dashboard setup
- [ ] 5.1.3 Tailwind CSS configuration and theming
- [ ] 5.1.4 Responsive layout design with Flowbite components

#### 5.2 Pattern Input Interface (Flowbite Forms)
- [ ] 5.2.1 Regex pattern input field with Flowbite form components
- [ ] 5.2.2 Real-time pattern validation with error states
- [ ] 5.2.3 Pattern preview functionality using Flowbite alerts
- [ ] 5.2.4 Example patterns library with Flowbite dropdown/modal

#### 5.3 Configuration Options (Flowbite UI Components)
- [ ] 5.3.1 Dataset size controls using Flowbite range sliders
- [ ] 5.3.2 Format selection interface with Flowbite checkboxes/toggles
- [ ] 5.3.3 Export options configuration using Flowbite forms
- [ ] 5.3.4 Advanced settings modal using Flowbite modal component

#### 5.4 Integration & Download (Flowbite Data Display)
- [ ] 5.4.1 Rust-WASM integration with SvelteKit
- [ ] 5.4.2 File download functionality with Flowbite buttons
- [ ] 5.4.3 Progress indicators using Flowbite progress bars
- [ ] 5.4.4 Results display using Flowbite tables and cards
- [ ] 5.4.5 Success/error notifications using Flowbite toast components

### 6. Testing & Quality Assurance
**Duration**: 2-3 sessions
- [ ] 6.1 Unit test coverage for core modules
- [ ] 6.2 Integration tests for exporters
- [ ] 6.3 End-to-end testing for web interface
- [ ] 6.4 CLI tool testing and validation

### 7. Documentation & Examples
**Duration**: 1-2 sessions
- [ ] 7.1 API documentation generation
- [ ] 7.2 Usage examples and tutorials
- [ ] 7.3 README and installation guides
- [ ] 7.4 Performance benchmarks

### 8. Deployment & Distribution
**Duration**: 1-2 sessions
- [ ] 8.1 Web interface deployment setup
- [ ] 8.2 CLI tool packaging and distribution
- [ ] 8.3 Release workflow automation
- [ ] 8.4 Documentation hosting

### 9. Future Enhancements (Post-MVP)
**Duration**: Ongoing
- [ ] 9.1 Additional format support planning
- [ ] 9.2 Performance optimization analysis
- [ ] 9.3 User feedback integration
- [ ] 9.4 Feature roadmap development

---

## Estimated Total Duration: 18-28 Claude sessions

### Critical Path:
1. Core Library (Sessions 1-8)
2. Basic Exporters (Sessions 9-12)
3. CLI Tool (Sessions 13-15)
4. Web Interface (Sessions 16-20)
5. Testing & Documentation (Sessions 21-24)

### Parallel Development Opportunities:
- CLI and Web interface can be developed simultaneously after core library
- Different format exporters can be implemented in parallel
- Documentation can be written alongside feature development
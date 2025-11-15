# Pokemon TCG DexId Updater

A Rust tool that automatically adds missing `dexId` fields to Pokemon Trading Card Game TypeScript card definitions.

## Overview

This tool scans a directory of Pokemon card definitions and:
1. Builds a map of Pokemon names to their National Pokedex IDs from existing cards
2. Automatically adds the `dexId` field to cards that are missing it
3. Handles special card variants (Mega evolutions, ex cards, etc.)

## Features

- 🔍 Recursively scans nested directories
- 🗺️ Builds a comprehensive name → dexId mapping
- ✏️ Automatically updates files with missing dexId
- 🎯 Handles special cases:
  - Mega Pokemon (e.g., "Mega Charizard X" → uses "Charizard" dexId)
  - ex cards (e.g., "Pikachu ex" → uses "Pikachu" dexId)
- 🚫 Skips "Pokémon TCG Pocket" folder
- 📊 Provides detailed logging of all operations

## Prerequisites

- Rust 1.70 or higher
- Cargo

## Installation

1. Clone this repository
2. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage

1. Update the `cards_dir` path in `main.rs` to point to your cards database:
```rust
let cards_dir = "/path/to/your/cards-database/data";
```

2. Run the tool:
```bash
cargo run --release
```

### Example Output

```
📊 Building name -> dexId map...
✅ Found 151 Pokémon with dexId

🔄 Updating files without dexId...
  ✓ Updated: ./cards/series1/Bulbasaur.ts (dexId: [1])
  ✓ Updated: ./cards/series2/Mega Charizard X.ts (dexId: [6])
  ✓ Updated: ./cards/series3/Pikachu ex.ts (dexId: [25])
✨ Done! Updated 42 files
```

## Input File Format

The tool expects TypeScript files with the following structure:

```typescript
import { Card } from "../../../interfaces"
import Set from "../Mega Evolution"
const card: Card = {
	set: Set,
	name: {
		en: "Bulbasaur",
		fr: "Bulbizarre",
		// ... other languages
	},
	stage: "Basic",
	dexId: [1],  // This field will be added if missing
	attacks: [
		// ...
	],
	// ... other fields
}
export default card
```

## How It Works

### Phase 1: Building the Map
The tool scans all `.ts` files and extracts:
- The English name (`name.en`)
- The dexId (if present)

This creates a lookup table: `"Bulbasaur" → [1]`, `"Pikachu" → [25]`, etc.

### Phase 2: Updating Files
For each file without a `dexId`:
1. Extract the English name
2. Normalize special variants:
   - "Mega Charizard X" → "Charizard"
   - "Pikachu ex" → "Pikachu"
3. Look up the dexId in the map
4. Insert the dexId after the `stage` field (or before `attacks` if no stage)
5. Write the updated content back to the file

## Configuration

Edit `main.rs` to customize:

- **Cards directory**: Change `cards_dir` variable
- **Skip folders**: Modify the folder skip logic in `update_files_without_dexid()`
- **Insertion position**: Adjust the regex patterns in `add_dex_id()`


## Safety

⚠️ **Important**: This tool modifies files in place. Consider:
- Making a backup of your cards database before running
- Using version control (git) to track changes
- Running on a test subset first

## Troubleshooting

### No files updated
- Check that the `cards_dir` path is correct
- Verify that files have the expected TypeScript format
- Ensure some files already have `dexId` fields to build the map

### DexId not found for some Pokemon
- Check for typos in the `name.en` field
- Verify that at least one card for each Pokemon has a dexId
- Check the logs to see which names weren't matched

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - feel free to use this tool for your projects.

## Acknowledgments

Built for managing Pokemon TCG card databases with consistent dexId references across all card variants.

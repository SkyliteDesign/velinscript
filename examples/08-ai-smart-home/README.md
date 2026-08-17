# VelinScript Example: AI Smart Home System

This example demonstrates the advanced capabilities of VelinScript v3.0, focusing on AI-driven compilation passes, system generation, and the new standard library modules.

## Key Features Showcased

### 1. AI Optimization (`@Optimize`)
In `automation.velin`, we define functions with `@Optimize` annotations.
```velin
@Optimize(target="energy_efficiency")
fn calculate_hvac_load(...) { ... }
```
The VelinScript compiler's AI pass analyzes these functions to optimize for the specific target metric (e.g., reducing computational complexity or prioritizing branches that favor energy efficiency logic).

### 2. Transactional Flows (`@Flow`)
The `good_night_routine` demonstrates a transactional workflow. If any step in the sequence fails (e.g., smart lock fails to engage), the `@Flow` decorator ensures the system state is consistent (rolling back previous actions if supported).

### 3. System Generation (`@Generate`)
The `SmartDevice` struct in `devices.velin` is marked with `@Generate(api=true)`.
When running `velin generate api`, this struct is automatically exposed as a RESTful endpoint (e.g., `GET /api/devices`, `POST /api/devices`).

### 4. New Standard Library
This project uses several new modules:
- **`alerting`**: For defining and checking security rules.
- **`scheduler`**: For cron-based automation.
- **`event_bus`**: For decoupled communication between system components.
- **`env`**: For type-safe environment variable loading.

## Project Structure

- `main.velin`: Entry point orchestrating the system.
- `devices.velin`: Data models and device definitions.
- `automation.velin`: Business logic and AI-optimized algorithms.

## Running the Example

```bash
velin run main.velin
```

## Compilation Analysis

When compiling with the `--analyze` flag, VelinScript reports optimization opportunities:

```bash
velin build --analyze
> AI Optimization Pass:
> - 'calculate_hvac_load': Optimized branch prediction for 'energy_efficiency' target.
> - 'process_sensor_stream': Loop unrolling applied for latency reduction.
```

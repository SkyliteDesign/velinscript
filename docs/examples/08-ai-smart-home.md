# Tutorial: Building an AI-Powered Smart Home with VelinScript

This guide walks through the creation of the `08-ai-smart-home` example project. It highlights how VelinScript's unique features—AI compilation passes, transactional flows, and system generation—can be used to build robust, efficient systems.

## 1. Project Architecture

We are building a simplified home automation system with three layers:
1.  **Data Layer (`devices.velin`)**: Defines the physical entities (lights, thermostats) and their state.
2.  **Logic Layer (`automation.velin`)**: Contains the intelligence—rules, schedules, and optimization algorithms.
3.  **Orchestration Layer (`main.velin`)**: Ties everything together using the Event Bus and Scheduler.

## 2. Defining Data Models with Generation

VelinScript allows you to define data models that automatically generate their own infrastructure.

In `devices.velin`, we define the `SmartDevice` struct:

```velin
@Generate(api=true)
struct SmartDevice {
    id: string,
    name: string,
    type: DeviceType,
    // ...
}
```

**What happens here?**
The `@Generate(api=true)` decorator tells the VelinScript System Generator to:
1.  Create database schemas (SQL/NoSQL) for this struct.
2.  Generate REST API endpoints (`GET`, `POST`, `PUT`, `DELETE`).
3.  Create OpenAPI documentation.

This eliminates the boilerplate of writing CRUD controllers manually.

## 3. Implementing AI-Optimized Logic

One of VelinScript's flagship features is the ability to hint the compiler about optimization targets.

In `automation.velin`:

```velin
@Optimize(target="energy_efficiency")
fn calculate_hvac_load(current_temp: number, target_temp: number, occupancy: boolean): number {
    // Logic ...
}
```

**How the AI Pass works:**
1.  **Static Analysis**: The compiler analyzes the control flow graph.
2.  **Pattern Matching**: It identifies inefficient patterns (e.g., redundant calculations in hot loops).
3.  **Target-Specific Optimization**:
    *   For `energy_efficiency`, it might favor integer arithmetic over floating-point where possible, or minimize wake-cycles in generated embedded code.
    *   For `latency`, it might unroll loops and inline functions aggressively.

## 4. Transactional Reliability

Home automation actions often need to be atomic. You don't want the front door to unlock if the alarm fails to disarm.

```velin
@Flow(name="good_night_routine", transactional=true)
fn run_good_night_sequence(home_id: string): boolean {
    // ...
}
```

The `@Flow` decorator wraps the function in a transaction context. If an exception occurs or `false` is returned, the system can trigger compensation logic (rollback), ensuring the home isn't left in a partial, insecure state.

## 5. Using the Standard Library

The `main.velin` file showcases the richness of the modern VelinScript stdlib:

*   **`event_bus`**: `event_bus.publish(...)` allows for a decoupled architecture where sensors publish data without knowing who consumes it.
*   **`alerting`**: `alerting.create_rule(...)` provides a declarative way to define business rules, separating policy from implementation.
*   **`scheduler`**: `scheduler.schedule(...)` handles complex timing requirements using standard Cron syntax.

## Conclusion

This example demonstrates that VelinScript is more than just a language; it's a platform for building modern systems. By combining code generation, AI optimization, and robust standard libraries, developers can focus on the *what* (business logic) rather than the *how* (boilerplate and infrastructure).

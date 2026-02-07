### Dependencies explanation:

**embassy-executor**:
This is the async runtime (the scheduler). It replaces the standard `loop` in `main` and manages switching between tasks.
- `arch-cortex-m`: Tells the executor it's running on an ARM Cortex-M processor (handles assembly-level context switching).
- `executor-thread`: Enables the "Thread Mode" executor. This is the main loop that runs tasks when interrupts aren't firing.

**embassy-stm32**
This is the Hardware Abstraction Layer (HAL) for specific chip. It replaces `stm32l4xx-hal`.
- `stm32l476rg`: It pulls in the specific register maps (PAC) and pin definitions for that exact silicon. If you have an L432 or L412, you must change this.
- `time`: Enables the driver to use the hardware clocks/timers associated with embassy-time.
- `memory-x`: Automatically generates the linker script (memory.x) based on the chip feature selected above. No need to manually write memory boundaries anymore.
- `unstable-pac`: Exposes the pac (Peripheral Access Crate) module inside the HAL, allowing access to raw registers (like dp.RCC) if the HAL doesn't support a specific feature yet.
- `time-driver-any`: Tells the HAL to just pick one timer (usually TIM2) and use it to implement the global clock.

**embassy-time**
Handles timekeeping, delays, and timeouts.
- `defmt-timestamp-uptime`: Configures logs to show the timestamp (uptime) when a log message occurs.
- `tick-hz-32_768`: Sets the global "tick" rate of the system. 32.768kHz is the default standard for Embassy. It provides good resolution (approx 30us) while keeping power usage low and allowing timers to run for years without overflowing.

**cortex-m**
- `critical-section-single-core`: Embassy relies on "critical sections" to share data safely. This feature tells the compiler that there is the only one core, so a critical section just means disabling interrupts globally.
---
In the previously used `stm32l4xx-hal`, everything was lumped into one crate. Embassy splits things up to be modular:
- **Executor**: Runs the code.
- **HAL**: Talks to the chip hardware.
- **Time**: Handles the clock.

This allows to swap out the Executor (e.g., run the same code on a PC simulator) or swap the Time driver without rewriting the hardware access code.

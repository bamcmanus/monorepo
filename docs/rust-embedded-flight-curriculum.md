# Rust Embedded-Style Learning Curriculum

This document defines a 16-lesson Rust learning curriculum for this repository.
It is written as an implementation contract for future agents that scaffold the
exercises, and as a learning guide for the person implementing them.

The primary goal is to learn Rust. Embedded and flight-control context should
motivate constraints, examples, and testing habits, but should not add domain
complexity that obscures the Rust concept being taught.

## Priority Rule

When Rust pedagogy and domain realism conflict, choose the simpler Rust-learning
exercise. Embedded and flight context should motivate bounded memory, explicit
errors, deterministic behavior, and careful tests. It should not require
aerospace, controls, or certification expertise.

## Audience

This curriculum is for a Rust learner who has already explored basic Rust
concepts such as variables, primitive types, functions, control flow, and simple
command-line programs.

The exercises prepare the learner for embedded-style Rust by practicing:

- ownership, borrowing, and `Copy` behavior
- arrays, slices, and fixed-size data
- explicit error handling with `Result`
- small structs, methods, enums, and newtypes
- state machines and transition validation
- traits and test fakes
- byte parsing, checksums, endianness, and bitfields
- deterministic single-threaded coordination
- practical representation concepts
- `#![no_std]` library APIs with host-runnable tests
- panic-free, allocation-free design

## Repository Conventions

Future lesson crates should live under one directory:

```text
rust-embedded-flight/
  lesson-01-sensor-scaling/
  lesson-02-ownership-copy/
  ...
```

Each lesson should be added to the root Cargo workspace in `Cargo.toml` and
should include Bazel targets. This repository prefers Bazel verification.

For each lesson:

- crate path: `rust-embedded-flight/lesson-XX-name`
- Cargo package name: `lesson-XX-name`
- Bazel library target: `//rust-embedded-flight/lesson-XX-name:lib`
- Bazel test target: `//rust-embedded-flight/lesson-XX-name:test`
- verification command: `bazel test //rust-embedded-flight/lesson-XX-name:test`

Lesson crates should be libraries by default. Add a tiny binary only when the
lesson clearly benefits from runtime demonstration. The core implementation in
`src/lib.rs` should stay under 100 lines, excluding tests, comments, `Cargo.toml`,
and `BUILD.bazel`.

## Agent Scaffolding Rules

When an agent is asked to scaffold a lesson from this document, it should
scaffold one lesson at a time unless explicitly told otherwise.

The agent should create:

- lesson directory
- `Cargo.toml`
- `BUILD.bazel`
- `src/lib.rs`
- public function, type, trait, or enum stubs required by the lesson
- inline unit tests by default
- integration tests only when the lesson explicitly benefits from them
- a top-level lesson comment in `src/lib.rs` that clearly states the learning
  objectives and the learner-owned work to complete
- a learning-journal section in `src/lib.rs` that copies the lesson's
  self-assessment prompts and provides a clear place for learner answers
- explanatory comments near important Rust teaching points, such as derives,
  tuple structs, ownership, borrowing, lifetimes, trait bounds, enum matching,
  buffer ownership, `no_std`, checked arithmetic, and error variants
- short TODO comments where useful

The agent should update:

- root `Cargo.toml` workspace members
- Bazel target wiring for the lesson

The agent must not:

- provide complete reference solutions in the scaffold
- create `SOLUTION.md` files unless explicitly requested later
- use `todo!()`, `unimplemented!()`, `panic!()`, `unwrap()`, or `expect()` in
  library code
- add external dependencies to the 16 core lessons
- turn light embedded examples into domain-heavy flight simulations

Starter implementations should compile with safe placeholder values. Prefer
wrong-but-safe defaults such as `0`, `false`, `None`, or
`Err(Error::NotImplemented)` over stubs that intentionally panic. Tests should
fail by assertion until the learner implements the logic.

Scaffold comments should make the lesson self-teaching without becoming a
reference solution. Explain what generated derives do and why they are present,
what each public API is meant to teach, and what each test proves about the Rust
concept. Comments may point out hazards such as unsigned underflow, moved values,
out-of-bounds indexing, partial writes, or panic paths, but should not spell out
the final implementation line-by-line.

Because this repository also acts as a learning journal, each lesson scaffold
should include the lesson's self-assessment prompts in `src/lib.rs`, with an
obvious answer area. Put this learning-journal section at the bottom of the file
by default, after the tests, so the lesson flows from problem setup, to concepts,
to implementation, to tests, to learner reflection. The scaffold should leave
answers blank. The learner may fill them in as ordinary comments when completing
the lesson. Future agents reviewing completion should accept answers in the
source file, in chat, or an explicit learner decision to skip them.

Compiler-error teaching moments should appear as notes in this document or in
lesson instructions, not as intentionally broken scaffolded code.

## Learner Workflow

Recommended workflow for each lesson:

1. Ask an agent to scaffold exactly one lesson from this document.
2. Read the lesson goal, constraints, and tests.
3. Implement only the learner-owned logic.
4. Run the lesson Bazel test target.
5. Review whether the implementation satisfies the constraints, not just the
   tests.
6. Answer the self-assessment prompts.
7. Ask for code review when stuck or after completing a phase.

Every 4 lessons, do a phase checkpoint before moving on.

## Progress Tracking

Use this checklist as the source of truth for curriculum progress. Future agents
should read this section before scaffolding or reviewing lessons.

- [x] Lesson 1: Sensor Scaling Basics
- [x] Lesson 2: Ownership And Copy Semantics
- [ ] Lesson 3: Slices And Fixed Windows
- [ ] Lesson 4: Caller-Provided Buffers
- [ ] Phase 1 Review Checkpoint
- [ ] Lesson 5: Structs, Methods, And Newtypes
- [ ] Lesson 6: Enums, Match, Option, And Result
- [ ] Lesson 7: Error Taxonomy
- [ ] Lesson 8: State Machines
- [ ] Phase 2 Review Checkpoint
- [ ] Lesson 9: Calibration With Lifetimes
- [ ] Lesson 10: Traits And Test Fakes
- [ ] Lesson 11: Byte Parsing And Endianness
- [ ] Lesson 12: Checksums And Bitfields
- [ ] Phase 3 Review Checkpoint
- [ ] Lesson 13: Deterministic Tick Coordination
- [ ] Lesson 14: Representation And Wire Values
- [ ] Lesson 15: `no_std` Core APIs
- [ ] Lesson 16: Panic-Free Allocation-Free API Design
- [ ] Phase 4 Review Checkpoint
- [ ] Optional Capstone: Mini Flight Supervisor

Do not mark a lesson complete just because it has been scaffolded. Scaffolding
and completion are separate states. A lesson is complete only after the learner
implementation satisfies the completion definition below.

## Agent Next-Lesson Protocol

When an agent is asked to continue this curriculum, it should:

1. Read this document, especially `Progress Tracking` and the target lesson spec.
2. Select the first unchecked lesson in `Progress Tracking`, unless the user
   explicitly names a different lesson.
3. If the first unchecked item is a review checkpoint, perform that checkpoint
   instead of scaffolding the next lesson.
4. Scaffold only one lesson at a time unless the user explicitly asks for more.
5. Do not implement learner-owned logic.
6. Do not provide a reference solution.
7. Use safe placeholder implementations that compile and fail tests by assertion.
8. Add the lesson to the Cargo workspace and Bazel target graph.
9. Run the lesson's Bazel test target if feasible and report the expected initial
   failing status.

After scaffolding, the agent should report:

- lesson scaffolded
- files created or updated
- Bazel test command
- expected initial test status
- where the in-file lesson objectives, learner tasks, and explanatory comments
  were added
- where the self-assessment learning-journal prompts were added
- what the learner should implement next

An agent should mark checklist items only when explicitly asked to update
progress or when the user clearly confirms completion. If the worktree contains
learner changes, the agent must preserve them and avoid overwriting
learner-owned implementation code.

## Lesson Completion Definition

A lesson is complete when all of these are true:

- the lesson's Bazel test target passes
- core implementation in `src/lib.rs` is under 100 lines, excluding tests and
  comments
- forbidden APIs are absent from library code
- tests cover the required success criteria
- boundary cases and listed invariants are tested where applicable
- learner-owned code satisfies the lesson constraints
- the learner has answered the self-assessment prompts or explicitly skipped
  them
- any phase checkpoint required after the lesson has been completed before
  moving to the next phase

When marking a lesson complete, update only the relevant checkbox in `Progress
Tracking`. Do not rewrite lesson specs or alter unrelated checklist items.

## Global Constraints

These constraints apply to all 16 core lessons unless a lesson says otherwise:

- Rust learning objectives are primary.
- Keep examples lightly embedded-flavored, not domain-heavy.
- Keep the core implementation under 100 lines.
- Prefer clear, direct code over abstractions.
- Do not use external Rust dependencies.
- Do not use heap allocation unless explicitly allowed.
- Do not use `panic!`, `unwrap`, or `expect` in library code.
- Use `Result` for recoverable errors.
- Use explicit bounds checks for caller-provided buffers and byte parsing.
- Use checked, saturating, or wrapping arithmetic intentionally.
- Prefer named constants over magic numbers for limits, masks, and wire values.
- Tests should include boundary cases and at least one invariant where useful.
- Format code with the repository Rust formatting conventions.

Additional constraints for `no_std` lessons:

- library code must use `#![no_std]`
- `std` may appear only under `#[cfg(test)]`
- no heap allocation
- no `Vec`, `String`, `Box`, or allocator-dependent APIs
- no OS-backed APIs such as files, networking, threads, stdin, or stdout

## Dependency Map

The lesson order intentionally builds these dependencies:

- ownership and borrowing before caller-provided buffers
- `Copy` and move semantics before practical representation concepts
- arrays and slices before fixed windows and `no_std` buffer APIs
- structs and methods before newtypes and state holders
- enums and `Result` before error taxonomy, parsers, and fault handling
- integer arithmetic before fixed-point-style scaling
- borrowed slices before explicit lifetime-bearing structs
- traits before hardware-like abstraction and test fakes
- byte parsing before checksums and wire values
- bit operations before register-style bitfields
- panic-free error handling before `no_std`
- caller-provided buffers before allocation-free API design

## Review Checkpoints

After lessons 4, 8, 12, and 16, perform a review checkpoint.

Each checkpoint should verify:

- all lesson tests pass through Bazel
- core implementations stay under 100 lines
- forbidden APIs are absent from library code
- tests cover boundaries and important invariants
- the learner can explain ownership and error-handling choices
- the embedded framing did not overtake the Rust lesson

## Phase 1: Rust Foundations For Bounded Logic

### Lesson 1: Sensor Scaling Basics

- path: `rust-embedded-flight/lesson-01-sensor-scaling`
- package: `lesson-01-sensor-scaling`
- Bazel test: `//rust-embedded-flight/lesson-01-sensor-scaling:test`

Concept goals:

- write small functions with explicit input and output types
- practice integer arithmetic and explicit returns
- use constants for bounds and scale factors
- understand truncating integer division

Embedded relevance:

- raw sensor counts often need conversion into scaled engineering units
- bounded integer math is easier to test and reason about than hidden floating
  point behavior

Agent scaffolds:

- `pub const MAX_ADC_COUNTS: u16 = 4095`
- `pub const REFERENCE_MILLIVOLTS: u16 = 3300`
- `pub fn adc_to_millivolts(counts: u16) -> u16`
- `pub fn clamp_counts(counts: u16) -> u16`
- tests for zero, midpoint, maximum, and above-maximum inputs

Learner implements:

- clamping raw counts to `MAX_ADC_COUNTS`
- converting ADC counts to millivolts with integer arithmetic
- preserving deterministic truncation behavior

Constraints:

- no floating point
- no panics
- no heap allocation
- use named constants for limits

Success criteria:

- `0` counts converts to `0` mV
- `MAX_ADC_COUNTS` converts to `REFERENCE_MILLIVOLTS`
- above-range counts are clamped before conversion
- midpoint conversion is deterministic and covered by tests

Stretch goal:

- add a second conversion function that rounds to nearest instead of truncating

Self-assessment:

- Where does integer truncation happen?
- What inputs are outside the expected sensor range?
- Could this function panic?
- What invariant should hold for every output millivolt value?

Try-this note:

- Try changing the return type to `u8` and observe the compiler errors or test
  failures caused by narrowing the output range.

### Lesson 2: Ownership And Copy Semantics

- path: `rust-embedded-flight/lesson-02-ownership-copy`
- package: `lesson-02-ownership-copy`
- Bazel test: `//rust-embedded-flight/lesson-02-ownership-copy:test`

Concept goals:

- distinguish moved values from copied values
- pass small scalar values by copy
- borrow larger inputs immutably
- understand when a value remains usable after a function call

Embedded relevance:

- embedded APIs often pass small values by copy and larger buffers by reference
- knowing what is copied helps reason about stack use and ownership

Agent scaffolds:

- `#[derive(Copy, Clone, Debug, PartialEq, Eq)] pub struct Sample(pub u16)`
- `pub fn newer_sample(previous: Sample, current: Sample) -> Sample`
- `pub fn sample_delta(previous: &Sample, current: &Sample) -> u16`
- tests that call functions and then reuse the original values

Learner implements:

- selecting the newer/current sample
- computing absolute delta using borrowed inputs
- preserving caller ownership where references are used

Constraints:

- no heap allocation
- no panics
- use references where tests require caller reuse

Success criteria:

- copied `Sample` values remain usable after function calls
- borrowed samples remain usable after delta calculation
- delta is symmetric and cannot underflow

Stretch goal:

- add a non-`Copy` `SampleLog` wrapper and note how passing by value changes
  ownership behavior

Self-assessment:

- Which values are copied?
- Which values are borrowed?
- Why does `Sample` implement `Copy` safely?
- What would change if `Sample` contained a heap-owned string?

Try-this note:

- Remove `Copy` from `Sample` and observe which tests or call sites stop
  compiling.

### Lesson 3: Slices And Fixed Windows

- path: `rust-embedded-flight/lesson-03-slices-fixed-windows`
- package: `lesson-03-slices-fixed-windows`
- Bazel test: `//rust-embedded-flight/lesson-03-slices-fixed-windows:test`

Concept goals:

- work with arrays and slices
- iterate without allocation
- return `Option` for empty input
- compute min, max, and average over bounded samples

Embedded relevance:

- fixed windows of recent sensor samples are common in embedded systems
- slices allow APIs to accept caller-owned storage of different fixed sizes

Agent scaffolds:

- `#[derive(Debug, PartialEq, Eq)] pub struct WindowStats { pub min: u16, pub max: u16, pub average: u16 }`
- `pub fn summarize_window(samples: &[u16]) -> Option<WindowStats>`
- tests for empty, one-element, multiple-element, and boundary-value windows

Learner implements:

- empty-slice handling
- min and max calculation
- integer average calculation without allocation

Constraints:

- no `Vec`
- no panics
- no indexing without prior bounds knowledge
- handle empty slices explicitly

Success criteria:

- empty input returns `None`
- non-empty input returns correct min, max, and average
- average behavior is deterministic for non-even division
- tests include max `u16` values without overflow

Stretch goal:

- use a wider accumulator type to safely average large `u16` samples

Self-assessment:

- Who owns the sample storage?
- Why is a slice better than a fixed array parameter here?
- What overflow risk exists when summing many `u16` values?
- What invariant relates `min`, `average`, and `max`?

Try-this note:

- Try returning a reference to a local array and observe why the compiler rejects
  it.

### Lesson 4: Caller-Provided Buffers

- path: `rust-embedded-flight/lesson-04-caller-provided-buffers`
- package: `lesson-04-caller-provided-buffers`
- Bazel test: `//rust-embedded-flight/lesson-04-caller-provided-buffers:test`

Concept goals:

- use mutable slices safely
- write output into caller-owned memory
- return the number of bytes written
- represent buffer-capacity errors with `Result`

Embedded relevance:

- allocation-free APIs often write into buffers supplied by the caller
- explicit capacity checks prevent memory corruption and panic paths

Agent scaffolds:

- `#[derive(Debug, PartialEq, Eq)] pub enum EncodeError { BufferTooSmall }`
- `pub fn encode_samples(samples: &[u16], output: &mut [u8]) -> Result<usize, EncodeError>`
- tests for empty input, exact capacity, oversized output, and too-small output

Learner implements:

- required capacity calculation
- big-endian encoding of each `u16` sample
- returning written length without modifying beyond that length

Constraints:

- no allocation
- no panics
- no partial writes on `BufferTooSmall`
- use `to_be_bytes` for numeric conversion

Success criteria:

- function writes exactly `samples.len() * 2` bytes
- too-small output returns `Err(BufferTooSmall)`
- bytes after the written range are unchanged
- empty input succeeds with `0` bytes written

Stretch goal:

- add a decode function that reverses the encoding into a caller-provided sample
  buffer

Self-assessment:

- Which slice is immutable and which is mutable?
- What prevents writing past the end of `output`?
- Why return `usize`?
- What behavior should callers rely on after an error?

## Phase 2: Embedded Modeling Patterns

### Lesson 5: Structs, Methods, And Newtypes

- path: `rust-embedded-flight/lesson-05-structs-methods-newtypes`
- package: `lesson-05-structs-methods-newtypes`
- Bazel test: `//rust-embedded-flight/lesson-05-structs-methods-newtypes:test`

Concept goals:

- define tuple structs as lightweight newtypes
- implement methods
- prevent mixing raw counts and scaled units
- keep validation near type construction

Embedded relevance:

- explicit unit types reduce accidental mixing of raw sensor counts, millivolts,
  and command percentages

Agent scaffolds:

- `pub struct AdcCounts(pub u16)`
- `pub struct Millivolts(pub u16)`
- `pub struct ActuatorPercent(pub u8)`
- `impl ActuatorPercent { pub fn new(value: u8) -> Option<Self> }`
- `pub fn counts_to_millivolts(counts: AdcCounts) -> Millivolts`
- tests for valid and invalid actuator percentages and unit conversion

Learner implements:

- bounded constructor logic
- conversion logic using newtypes
- simple methods for reading inner values if needed

Constraints:

- do not expose invalid `ActuatorPercent` through `new`
- no floating point
- no panics
- avoid generic unit abstractions

Success criteria:

- actuator values above 100 are rejected
- raw counts cannot be passed where actuator percent is expected
- conversion tests preserve expected units

Stretch goal:

- add `is_zero` or `is_full_scale` methods for actuator commands

Self-assessment:

- What bug can newtypes prevent?
- Which constructors can fail?
- Is the inner field public, and what does that imply?
- When is a tuple struct better than a type alias?

### Lesson 6: Enums, Match, Option, And Result

- path: `rust-embedded-flight/lesson-06-enums-option-result`
- package: `lesson-06-enums-option-result`
- Bazel test: `//rust-embedded-flight/lesson-06-enums-option-result:test`

Concept goals:

- model alternatives with enums
- use exhaustive `match`
- distinguish absent data from invalid data
- use `Option` and `Result` intentionally

Embedded relevance:

- sensors can be valid, missing, stale, or invalid; explicit states avoid
  ambiguous sentinel values

Agent scaffolds:

- `pub enum Reading { Valid(u16), Missing, Stale }`
- `pub enum ReadingError { Missing, Stale, OutOfRange }`
- `pub fn usable_reading(reading: Reading, max: u16) -> Result<u16, ReadingError>`
- tests for every enum variant and out-of-range values

Learner implements:

- exhaustive match handling
- conversion from reading state to success/error
- out-of-range validation

Constraints:

- no sentinel values like `u16::MAX` for missing data
- no panics
- explicit match arms for each state

Success criteria:

- every reading state maps to an explicit result
- invalid valid-readings return `OutOfRange`
- tests cover all enum variants

Stretch goal:

- add a helper that converts `Option<u16>` into `Reading`

Self-assessment:

- Why is `Missing` different from `OutOfRange`?
- What does exhaustive matching protect against?
- When should this API return `Option` instead of `Result`?
- Could a caller ignore the error accidentally?

### Lesson 7: Error Taxonomy

- path: `rust-embedded-flight/lesson-07-error-taxonomy`
- package: `lesson-07-error-taxonomy`
- Bazel test: `//rust-embedded-flight/lesson-07-error-taxonomy:test`

Concept goals:

- design small error enums
- distinguish malformed input, out-of-range input, unavailable data, and unsafe
  requests
- map low-level validation failures into domain-level errors
- avoid collapsing all failures into one catch-all error

Embedded relevance:

- control logic often needs different responses for bad input, missing data, and
  unsafe commands

Agent scaffolds:

- `pub enum CommandError { Malformed, OutOfRange, DataUnavailable, UnsafeRequest, NotImplemented }`
- `pub struct Command { pub actuator_percent: u8, pub armed: bool }`
- `pub fn validate_command(raw_percent: Option<u8>, armed: bool) -> Result<Command, CommandError>`
- tests for each error variant and the valid path

Learner implements:

- missing data handling
- range validation
- unsafe request rejection, such as non-zero actuator command while disarmed
- successful command construction

Constraints:

- no stringly typed errors
- no catch-all `_` arm when matching known cases
- no panics

Success criteria:

- each error variant has a test that intentionally triggers it
- valid command returns structured data
- unsafe command is not silently clamped into validity

Stretch goal:

- add a `severity` method that classifies each error without using strings

Self-assessment:

- Which errors are caused by malformed data?
- Which errors are caused by unsafe system state?
- Why not use a single `Invalid` error?
- Should validation clamp or reject unsafe commands?

### Lesson 8: State Machines

- path: `rust-embedded-flight/lesson-08-state-machines`
- package: `lesson-08-state-machines`
- Bazel test: `//rust-embedded-flight/lesson-08-state-machines:test`

Concept goals:

- model states with enums
- validate transitions explicitly
- use `match` for transition logic
- reject illegal transitions with typed errors

Embedded relevance:

- many embedded systems are state machines; valid transitions matter more than
  ad hoc booleans

Agent scaffolds:

- `pub enum Mode { Idle, Standby, Active, Faulted }`
- `pub enum TransitionError { InvalidTransition, FaultRequiresReset }`
- `pub fn transition(current: Mode, requested: Mode) -> Result<Mode, TransitionError>`
- tests for every allowed transition and several forbidden transitions

Learner implements:

- allowed transition table
- rejection of invalid transitions
- special handling for leaving `Faulted`

Constraints:

- no hidden global state
- no boolean mode flags
- match all relevant states explicitly

Success criteria:

- legal transitions succeed
- illegal transitions return `InvalidTransition`
- faulted state cannot be left except by an explicitly allowed reset path, if
  specified by the tests

Stretch goal:

- add an event-based API: `apply_event(current, event) -> Result<Mode, ...>`

Self-assessment:

- What invalid state combinations are impossible with an enum?
- Does this function mutate state or return new state?
- How would adding a new mode affect the compiler?
- Which tests prove forbidden transitions?

## Phase 3: Abstractions And Protocols

### Lesson 9: Calibration With Lifetimes

- path: `rust-embedded-flight/lesson-09-calibration-lifetimes`
- package: `lesson-09-calibration-lifetimes`
- Bazel test: `//rust-embedded-flight/lesson-09-calibration-lifetimes:test`

Concept goals:

- define a struct that borrows immutable configuration
- write one focused explicit lifetime parameter
- apply integer scaling with calibration data
- avoid copying or allocating configuration

Embedded relevance:

- calibration data is often static or long-lived and should be shared safely

Agent scaffolds:

- `pub struct Calibration { pub offset_mv: i16, pub scale_per_mille: u16 }`
- `pub struct SensorScaler<'a> { pub calibration: &'a Calibration }`
- `impl<'a> SensorScaler<'a> { pub fn scale(&self, raw_mv: i16) -> i16 }`
- tests for positive offset, negative offset, scaling, and boundary values

Learner implements:

- borrowed calibration storage
- offset plus per-mille scaling
- saturating or checked arithmetic where tests require it

Constraints:

- no allocation
- no copied calibration inside `SensorScaler`
- no floating point
- no panics on boundary values

Success criteria:

- scaler uses borrowed calibration data
- scaling handles offset and multiplier correctly
- boundary tests do not overflow or panic

Stretch goal:

- add a second scaler borrowing two calibration tables and explain the lifetimes

Self-assessment:

- What does `'a` mean in this struct?
- Who owns the calibration data?
- Why does the scaler not need to clone calibration?
- What arithmetic operation is most likely to overflow?

Try-this note:

- Try constructing a `SensorScaler` from a calibration value that goes out of
  scope before the scaler is used and observe the borrow checker error.

### Lesson 10: Traits And Test Fakes

- path: `rust-embedded-flight/lesson-10-traits-test-fakes`
- package: `lesson-10-traits-test-fakes`
- Bazel test: `//rust-embedded-flight/lesson-10-traits-test-fakes:test`

Concept goals:

- define a small trait
- write generic logic over a trait
- use fake implementations in tests
- keep hardware-like dependencies outside core logic

Embedded relevance:

- embedded HALs commonly expose peripherals through traits
- fakes make control logic testable without real hardware

Agent scaffolds:

- `pub enum SensorError { Unavailable, OutOfRange }`
- `pub trait Altimeter { fn altitude_cm(&self) -> Result<i32, SensorError>; }`
- `pub fn altitude_hold_ok<A: Altimeter>(altimeter: &A, target_cm: i32, tolerance_cm: i32) -> Result<bool, SensorError>`
- test-only fake altimeter types
- tests for within tolerance, outside tolerance, and sensor errors

Learner implements:

- generic trait-bound function
- absolute difference logic without panic
- error propagation

Constraints:

- avoid dynamic dispatch unless explicitly requested
- no global fake state
- no panics

Success criteria:

- logic works for multiple fake altimeters
- sensor errors propagate unchanged
- tolerance bounds are tested

Stretch goal:

- add a second trait for an actuator command sink and test both together

Self-assessment:

- Why use a trait instead of a concrete fake type?
- Where is the hardware boundary?
- What owns the fake sensor?
- What would change if dynamic dispatch were used?

### Lesson 11: Byte Parsing And Endianness

- path: `rust-embedded-flight/lesson-11-byte-parsing-endianness`
- package: `lesson-11-byte-parsing-endianness`
- Bazel test: `//rust-embedded-flight/lesson-11-byte-parsing-endianness:test`

Concept goals:

- parse fixed-format packets from byte slices
- validate packet length before indexing
- use explicit endianness conversion
- return typed parse errors

Embedded relevance:

- embedded systems commonly exchange compact byte packets with fixed layouts

Agent scaffolds:

- `pub struct TelemetryPacket { pub sequence: u8, pub altitude_cm: u16, pub voltage_mv: u16 }`
- `pub enum ParseError { WrongLength, InvalidHeader }`
- `pub fn parse_telemetry(bytes: &[u8]) -> Result<TelemetryPacket, ParseError>`
- tests for valid big-endian packet, wrong length, invalid header, and byte-order
  behavior

Learner implements:

- header validation
- exact length validation
- `u16::from_be_bytes` conversions
- packet struct construction

Constraints:

- no manual indexing before length validation
- no panics
- no external serialization crates
- use standard endian conversion methods

Success criteria:

- valid packet parses into expected fields
- wrong lengths are rejected
- invalid header is rejected
- tests would fail if little-endian conversion were used accidentally

Stretch goal:

- add an encoder using `to_be_bytes` and a caller-provided output buffer

Self-assessment:

- What byte order does the packet use?
- Which indexes are safe after length validation?
- Why is `WrongLength` different from `InvalidHeader`?
- Could this parser read beyond the input slice?

### Lesson 12: Checksums And Bitfields

- path: `rust-embedded-flight/lesson-12-checksums-bitfields`
- package: `lesson-12-checksums-bitfields`
- Bazel test: `//rust-embedded-flight/lesson-12-checksums-bitfields:test`

Concept goals:

- compute a simple checksum over bytes
- use bit masks and shifts intentionally
- define named constants for flags
- reject reserved-bit combinations

Embedded relevance:

- packed status flags and simple checksums are common in device protocols

Agent scaffolds:

- `pub const FLAG_READY: u8`
- `pub const FLAG_FAULT: u8`
- `pub const FLAG_CALIBRATING: u8`
- `pub const RESERVED_MASK: u8`
- `pub struct StatusFlags { pub ready: bool, pub fault: bool, pub calibrating: bool }`
- `pub enum StatusError { ReservedBitsSet, BadChecksum }`
- `pub fn checksum(bytes: &[u8]) -> u8`
- `pub fn parse_status(status: u8, expected_checksum: u8) -> Result<StatusFlags, StatusError>`
- tests for each flag, reserved bits, checksum success, and checksum failure

Learner implements:

- wrapping checksum calculation
- flag extraction with masks
- reserved-bit validation
- checksum validation as specified by tests

Constraints:

- no external `bitflags` crate
- no magic numeric masks in implementation
- use `wrapping_add` or document chosen overflow behavior
- no panics

Success criteria:

- every named flag has a test
- reserved bits are rejected
- checksum behavior is deterministic across overflow
- invalid checksum returns `BadChecksum`

Stretch goal:

- add a builder function that constructs a raw status byte from `StatusFlags`

Self-assessment:

- Which bits are reserved?
- What happens when checksum addition overflows?
- Why are named masks safer than raw literals?
- What invariant should parsed flags satisfy?

## Phase 4: Deterministic And `no_std` APIs

### Lesson 13: Deterministic Tick Coordination

- path: `rust-embedded-flight/lesson-13-deterministic-ticks`
- package: `lesson-13-deterministic-ticks`
- Bazel test: `//rust-embedded-flight/lesson-13-deterministic-ticks:test`

Concept goals:

- model periodic updates without threads
- pass immutable snapshots into update logic
- return explicit next state and command output
- avoid shared mutable global state

Embedded relevance:

- embedded control software often runs deterministic periodic loops or scheduled
  tasks

Agent scaffolds:

- `pub struct Snapshot { pub altitude_cm: i32, pub target_cm: i32, pub sensor_ok: bool }`
- `pub struct ControllerState { pub last_error_cm: i32 }`
- `pub struct Command { pub climb: bool, pub descend: bool }`
- `pub fn tick(state: ControllerState, snapshot: &Snapshot) -> (ControllerState, Command)`
- tests for climb, descend, hold, and sensor failure behavior

Learner implements:

- deterministic state update
- command selection from snapshot data
- safe behavior on invalid sensor data

Constraints:

- no `std::thread`
- no async runtime
- no global mutable state
- no panics

Success criteria:

- same input state and snapshot always produce same output
- sensor failure produces safe command behavior
- returned state captures the latest error
- tests model multiple ticks without hidden state

Stretch goal:

- add a deadband threshold to avoid toggling commands near target altitude

Self-assessment:

- What data is input snapshot versus persistent state?
- Is the update deterministic?
- Where would shared mutable state make this harder to test?
- How could this map to a real periodic task later?

### Lesson 14: Representation And Wire Values

- path: `rust-embedded-flight/lesson-14-representation-wire-values`
- package: `lesson-14-representation-wire-values`
- Bazel test: `//rust-embedded-flight/lesson-14-representation-wire-values:test`

Concept goals:

- use `Copy` and `Clone` intentionally
- map enum variants to explicit wire values
- understand practical use of `#[repr(u8)]` conceptually
- avoid relying on unspecified memory layout

Embedded relevance:

- wire protocols and register values often require explicit numeric codes

Agent scaffolds:

- `#[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum WireMode { Idle, Standby, Active, Faulted }`
- `pub enum WireModeError { UnknownValue }`
- `pub fn mode_to_wire(mode: WireMode) -> u8`
- `pub fn mode_from_wire(value: u8) -> Result<WireMode, WireModeError>`
- tests for every known wire value and unknown values

Learner implements:

- explicit mapping from enum variants to `u8`
- explicit mapping from `u8` to enum variants
- unknown-value rejection

Constraints:

- do not use unsafe casts or transmute
- do not rely on enum memory layout
- no panics
- explicit match arms for known values

Success criteria:

- every enum variant round-trips through wire conversion
- unknown values return `UnknownValue`
- implementation remains safe Rust

Stretch goal:

- add a `#[repr(u8)]` annotation and explain why the conversion functions are
  still clearer and safer than raw casts

Self-assessment:

- Why not cast arbitrary `u8` values directly into an enum?
- What does `Copy` mean for this enum?
- What layout assumptions does this code avoid?
- What happens when a new enum variant is added?

### Lesson 15: `no_std` Core APIs

- path: `rust-embedded-flight/lesson-15-no-std-core-apis`
- package: `lesson-15-no-std-core-apis`
- Bazel test: `//rust-embedded-flight/lesson-15-no-std-core-apis:test`

Concept goals:

- write a true `#![no_std]` library
- use `core`-available types and traits
- keep tests host-runnable with `std`
- avoid OS-backed APIs and allocation

Embedded relevance:

- many bare-metal embedded crates cannot depend on the Rust standard library

Agent scaffolds:

- `#![no_std]` at the top of `src/lib.rs`
- `#[cfg(test)] extern crate std;`
- `pub enum LimitError { MinGreaterThanMax }`
- `pub fn clamp_i16(value: i16, min: i16, max: i16) -> Result<i16, LimitError>`
- tests for in-range, below-range, above-range, and invalid bounds

Learner implements:

- `core`-only clamp logic
- invalid-bound error handling
- host-runnable tests without using `std` in library code

Constraints:

- library code must compile as `#![no_std]`
- no heap types
- no `println!`
- no panics

Success criteria:

- tests pass locally through Bazel
- `std` appears only under `#[cfg(test)]`
- invalid bounds return `Err(MinGreaterThanMax)`
- all logic uses `core`-compatible APIs

Stretch goal:

- add a generic clamp for types that implement `Ord + Copy`

Self-assessment:

- What does `no_std` remove?
- Why can tests still run locally?
- Which APIs used here come from `core`?
- What would require `alloc` or `std`?

### Lesson 16: Panic-Free Allocation-Free API Design

- path: `rust-embedded-flight/lesson-16-panic-free-api-design`
- package: `lesson-16-panic-free-api-design`
- Bazel test: `//rust-embedded-flight/lesson-16-panic-free-api-design:test`

Concept goals:

- combine `no_std`, caller-provided buffers, and explicit errors
- design APIs with bounded memory behavior
- avoid panic paths in normal library logic
- document output-buffer behavior clearly through tests

Embedded relevance:

- allocation-free and panic-free APIs are central to reliable embedded libraries

Agent scaffolds:

- `#![no_std]`
- `#[cfg(test)] extern crate std;`
- `pub enum FormatError { BufferTooSmall, InvalidInput }`
- `pub struct Reading { pub id: u8, pub millivolts: u16 }`
- `pub fn format_reading(reading: Reading, output: &mut [u8]) -> Result<usize, FormatError>`
- tests for valid formatting, too-small buffers, invalid input, unchanged tail
  bytes, and local host execution

Learner implements:

- validation of reading fields as specified by tests
- fixed-format byte output into caller-provided buffer
- exact written length return
- no writes on error where specified by tests

Constraints:

- true `#![no_std]` library
- no heap allocation
- no panics
- no partial writes on error
- no `std` outside tests

Success criteria:

- valid reading writes exact expected bytes
- too-small output returns `BufferTooSmall`
- invalid input returns `InvalidInput`
- bytes outside the written range remain unchanged
- no forbidden APIs are used in library code

Stretch goal:

- add a matching parser that reads the formatted representation back into a
  `Reading`

Self-assessment:

- What are this API's memory bounds?
- What inputs are invalid?
- Can any caller-triggered path panic?
- What guarantees does the function make after an error?

## Optional Capstone: Mini Flight Supervisor

- path: `rust-embedded-flight/capstone-mini-flight-supervisor`
- package: `capstone-mini-flight-supervisor`
- Bazel test: `//rust-embedded-flight/capstone-mini-flight-supervisor:test`

The capstone is optional and separate from the 16 core lessons. It may be
scaffolded only after the core curriculum is complete or when explicitly
requested.

Unlike the core lessons, the capstone may introduce one or two realistic
embedded ecosystem dependencies if desired, such as `heapless` and/or
`embedded-hal`. If dependencies are added, the scaffold agent must follow this
repository's Rust dependency workflow and update Cargo, `Cargo.lock`, Bazel
crate metadata, and Bazel targets correctly.

Preferred default: keep the first capstone scaffold dependency-free unless the
user explicitly asks to include `heapless` or `embedded-hal`.

Concept goals:

- integrate fixed buffers, newtypes, traits, byte parsing, state transitions,
  deterministic ticks, and explicit errors
- keep Rust concepts primary while using a light embedded supervisor example
- practice reviewing a larger but still bounded API surface

Suggested API surface:

- `pub struct SupervisorConfig`
- `pub struct TelemetrySnapshot`
- `pub struct SupervisorState`
- `pub struct ActuatorCommand`
- `pub enum SupervisorMode`
- `pub enum SupervisorFault`
- `pub enum SupervisorError`
- `pub trait AltitudeSource`
- `pub trait CommandSink`
- `pub fn parse_telemetry(bytes: &[u8]) -> Result<TelemetrySnapshot, SupervisorError>`
- `pub fn supervisor_tick(state: SupervisorState, snapshot: &TelemetrySnapshot, config: &SupervisorConfig) -> (SupervisorState, ActuatorCommand)`
- `pub fn format_command(command: ActuatorCommand, output: &mut [u8]) -> Result<usize, SupervisorError>`

Agent scaffolds:

- library crate under the capstone path
- tests covering packet parsing, state transitions, command formatting, fault
  handling, and deterministic tick behavior
- optional tiny binary only if useful for printing a short deterministic trace
- no complete reference solution

Learner implements:

- packet parsing with explicit length, header, endianness, and checksum checks
- supervisor state transition logic
- safe command generation under normal and fault conditions
- caller-provided command formatting buffer
- fake trait implementations for tests if trait-based APIs are scaffolded

Constraints:

- preserve bounded memory behavior
- avoid panics in library code
- avoid hidden global mutable state
- keep domain logic simple and Rust-focused
- use explicit error variants
- if dependencies are included, keep them limited and justified

Success criteria:

- tests cover normal, boundary, and fault paths
- repeated identical ticks produce identical outputs
- command outputs remain within configured bounds
- malformed packets are rejected without panic
- supervisor faults are explicit and testable
- no reference solution is included in the scaffold

Suggested capstone tests:

- valid telemetry packet parses correctly
- wrong-length packet returns a parse error
- invalid checksum returns a parse error
- unknown mode or command value returns an error
- normal tick produces bounded command
- sensor fault transitions supervisor into fault behavior
- fault behavior commands a safe output
- command formatting rejects too-small output buffers
- command formatting does not modify bytes outside the written range

Self-assessment:

- Which parts of the design own data, and which borrow it?
- Where are memory bounds enforced?
- Which errors are recoverable?
- Which invariants do the tests prove?
- Did any domain realism make the Rust code harder than necessary?

## Future Curriculum Ideas

These topics are intentionally deferred from the first 16 lessons:

- real microcontroller target setup and flashing
- linker scripts, panic handlers, and startup runtimes
- interrupts, atomics, critical sections, and RTOS integration
- async embedded Rust
- DMA and peripheral ownership models
- deeper memory layout, alignment, padding, ABI, and `repr(C)`
- unsafe Rust for MMIO, volatile access, and FFI
- formal methods, coverage tooling, and certification workflows
- advanced numeric methods, filtering, PID, and sensor fusion

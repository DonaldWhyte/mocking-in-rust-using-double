# Mocking in Rust

<p>
    <a href="http://donsoft.io">Donald Whyte</a>
    / <a href="http://twitter.com/donald_whyte">@donald_whyte</a>
</p>

<div id="logo-notice">
  <img src="images/fosdem.svg" alt="fosdem" />
  <p><strong>FOSDEM 2018</strong></p>
</div>

[NEXT]
### About Me

<div class="left-col1of3">
  ![small_portrait](images/donald.jpg)
</div>
<div class="right-col2of3" style="text-center: left">
  <div style="height: 27px"></div>
  <ul>
    <li>Software Engineer @ <strong>Engineers Gate</strong></li>
    <li>Real-time trading systems</li>
    <li>Scalable data infrastructure</li>
    <li>C++ developer and Rust enthusiast</li>
  </ul>
</div>
<div class="clear-col"></div>

_note_
TODO: intro to me, write down after saying out loud

Empahsise background building high-performance enterprise software in C++ for several years.

[NEXT]
## Outline

1. Unit Testing in Rust
2. Why Mock?
3. Mocking in Rust with `double`
4. Pattern Matching
5. Library Constraints


[NEXT SECTION]
## 1. Unit Testing

![unit_testing](images/unit_testing.svg)

_note_
* classist vs mockist testing
    - look up newer literature for this
* say that we're going to start w/ classist testing then move to mockist
* basic Rust unit test
* chosen unit test framework
* same unit tests as before but in new framework

Correctness in our programs means that our code does what we intend for it to do. Rust is a programming language that cares a lot about correctness, but correctness is a complex topic and isn’t easy to prove. Rust’s type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness. As such, Rust includes support for writing software tests within the language itself.

(source: https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html)

[NEXT]
Create library: `cargo new`

```bash
cargo new some_lib
cd some_lib
```

[NEXT]
Test fixture automatically generated:

```bash
> cat src/lib.rs
```

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // test code in here
    }
}
```

[NEXT]
`cargo test`

```cpp
> cargo test

   Compiling some_lib v0.1.0 (file:///Users/donaldwhyte/some_lib)
    Finished dev [unoptimized + debuginfo] target(s) in 2.99 secs
     Running target/debug/deps/some_lib-4ea7f66796617175

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests some_lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

[NEXT]
Write unit tests for a module by defining a private `tests` module in its source file.

<pre><code data-noescape class="rust">// production code
pub fn add_two(num: i32) -> i32 {
    num + 2
}

<mark>#[cfg(test)]</mark>
<mark>mod tests {</mark>
<mark>    // test code in here</mark>
<mark>}</mark>
</code></pre>

_note_
Annotate tests module with `#[cfg(test)]` so it's only built with `cargo test`.

This module will also _run_ when `cargo test` is invoked.

```
This is the automatically generated test module. The attribute cfg stands for configuration, and tells Rust that the following item should only be included given a certain configuration option. In this case, the configuration option is test, provided by Rust for compiling and running tests. By using this attribute, Cargo only compiles our test code if we actively run the tests with cargo test. This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].
```

Source: https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html

[NEXT]
Add isolated test functions to private `tests` module.

<pre class="medium"><code data-noescape class="rust">// ...prod code...

#[cfg(test)]
mod tests {
    use super::*;  // import production symbols from parent module

<mark>    #[test]</mark>
    fn ensure_two_is_added_to_negative() {
        assert_eq!(0, add_two(-2));
    }
    #[test]
    fn ensure_two_is_added_to_zero() {
        assert_eq!(2, add_two(0));
    }
    #[test]
    fn ensure_two_is_added_to_positive() {
        assert_eq!(3, add_two(1));
    }
}
</code></pre>

_note_
Emphasise the fact that each function is a separate, isolated test.

[NEXT]
```
dwhyte-mbp2:some_lib donaldwhyte$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/some_lib-4ea7f66796617175

running 3 tests
test tests::ensure_two_is_added_to_negative ... ok
test tests::ensure_two_is_added_to_positive ... ok
test tests::ensure_two_is_added_to_zero ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests some_lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

[NEXT]
Rust has native support for:

* documentation tests
* integration tests

Focus of talk is mocking, so these are not covered here.


[NEXT SECTION]
## 2. Why Mock?

![why_mock](images/why_mock.svg)

[NEXT]
![whymock](images/whymock1.png)

_note_
Here's a component hierarchy.

[NEXT]
![whymock](images/whymock2.png)

_note_
Suppose we want a test for the red component at the top there.

The component has three dependencies, or collaborators, which we build and pass
into component at construction.

These collaborators might be rely on external systems or require a large amoun
of setup. This makes testing the component difficult, because we either have to
ensure these external systems are available and in the right state, or write
lots more test code to setup the collaborators.

Since we aim to write tests for most of our components (or should), this extra
effort builds up and results in huge amounts of development time taken up by
tests.

...so then teams end up just not writing tests.

[NEXT]
![whymock](images/whymock3.png)

_note_
To avoid this, we replace the implementations of these collaborators with much
simpler, fake implementations.

[NEXT]
![whymock](images/whymock4.png)

_note_
No more environment dependencies, no more massive setup. It becomes much
quicker and easier to write the tests.

It also makes them less brittle. That is, they're less likely to break when
the real, concrete dependencies are changed (this is a good and bad thing).

[NEXT]
## What to Eliminate

Anything non-deterministic that can't be reliably controlled within a unit test.

[NEXT]
**External data sources** &mdash; files, databases

**Network connections** &mdash; services

**External code dependencies** &mdash; libraries

[NEXT]
### You Might Also Want to Eliminate

**Heavyweight internal code dependencies.**

* simpler test code
* makes individual tests less brittle
* downsides to eliminating these dependencies

_note_
Downsides to testing internal code dependencies:

* component is tested with mock collaborators that behave like you *think* they do
* real collaborators may behave differently
* real collaborators behaviour may *change*
* unit test with mocks won't pick that up
* still need integration tests to ensure real components work together

Despite these downsides, some believe the cost is worth simpler tests, because
they:

* encourage developers to write more tests, since it requires less work
* tests are smaller, generally test one thing
    * failures easier to understand
* tests are more maintainable
    * easy to understand
    * easy to change

[NEXT]
## Solution: Use Test Double

![stunt_double](images/brad_double_small.jpg)

[NEXT]
Term originates from a notion of a _"stunt double"_ in films.

A **test double** is an object or function substituted for a "real" (production ready) code during testing.

Should appear exactly the same as a **"real"** production instance to its clients (collaborators).

_note_
This is how we eliminate these unwanted dependencies from our tests.

Similar to using a stunt double in films, where viewers don't notice that
stunts are performed by a different actor.

[NEXT]
## Types

* **Stubs** return hard-coded values
* **Spies** record the code's interaction with collaborators
    - times method called and passed arguments
* **Mocks** return hard-coded values and verify interaction
    - both a stub and a spy

_note_
Stubs provide canned answers to calls made during the test, usually not responding at all to anything outside what's programmed in for the test.

Spies are stubs that also record some information based on how they were called. One form of this might be an email service that records how many messages it was sent.

Mocks are what we are talking about here: objects pre-programmed with expectations which form a specification of the calls they are expected to receive.

Souce: https://martinfowler.com/articles/mocksArentStubs.html

[NEXT]
<div class="left-col">
  <h4>State Verification</h4>
  <p>
    Test code by asserting on the its and its collaboator's <strong>post-test state</strong>.
  </p>
  <hr />
  <ul>
    <li>Stubs</li>
    <li>Spies</li>
  </ul>
</div>
<div class="right-col">
  <h4>Behaviour Verification</h4>
  <p>
    Test code by asserting on its <strong>interaction</strong> with its collaborators.
  </p>
  <hr />
  <ul>
    <li>Mocks</li>
  </ul>
</div>
<div class="clear-col"></div>

_note_
Of these kinds of doubles, only mocks insist upon behavior verification.

Souce: https://martinfowler.com/articles/mocksArentStubs.html

[NEXT]
TODO: empahsise use cases

[NEXT]
Behaviour verification with **mocks** is the focus of this talk.

_note_
WHY? Mocks are the most flexible. They're a superset of stubs and spies.

[NEXT SECTION]
## 3. Mocking in Rust Using `double`

![double](images/double.svg)

[NEXT]
## Coin Flipper
![coin_flip](images/coin-flip.jpg)

* A simple game to flip a coin
* `CoinFlipper` class implements the game
* It interacts with a random number generator
* We can change a number generator at runtime
* Goal is to **test** `CoinFlipper`

_note_
We can change a number generator at runtime, using dependency injection

[NEXT]
## Interfaces

```cpp
// Simplified version of `Rng` trait in the `rand` crate
pub trait Rng {
    fn next_f64(&mut self) -> f64;
}

pub enum CoinFlip {
    Heads,
    Tails,
}
```

[NEXT]
## Implementation

```rust
pub struct CoinFlipper {
    rng: Rng,
}

impl CoinFlipper {
    pub fn new(rng: Rng) -> CoinFlipper {
        CoinFlipper {
            rng: rng
        }
    }

    pub fn flip_coin(&mut self) -> {
        let r = rng.next_f64();
        if r < 0.5 {
            CoinFlip::Heads
        } else {
            CoinFlip::Tails
        }
    }
}
```
<!-- .element class="medium" -->

TODO: write code and ensure it builds

[NEXT]
## Playing the Game

`flip` is either `Heads` or `Tails`.

```rust
fn play() {
    // Construct a particular RNG implementation
    let rng = SomeRngImplementation();

    // Create a game
    let mut game = CoinFlipper(rng);

    // Start playing
    let flip = game.flip_coin();
    if flip == CoinFlip::Heads {
        println!("Heads!");
    } else {
        println!("Tails!");
    }
}
```

[NEXT]
![coin_flip_collaborators](images/coin-flip-collaborators.png)

* One collaborator &mdash; `Rng`
* Real RNG is non-deterministic
* We want to test `CoinFlipper` produces both results
  - we also want these tests to be repeatable
  - without relying on an external environment

[NEXT]
<!-- .slide: class="large-slide" -->
**Let's mock `Rng`.**

[NEXT]
## `double` to the Rescue!

* **generate** mock trait implementations using macros
* flexible configuration of mock's **behaviour**
* can make simple and complex **assertions** on mock calls
* **pattern matching** for call arguments

[NEXT]
### Core Design Principles
<br />
(1) Rust stable first <!-- .element: class="fragment" data-fragment-index="1" -->

(2) Requires no changes to production code <!-- .element: class="fragment" data-fragment-index="2" -->

_note_
Emphasise how these goals has had the biggest influence on the design of the
library. It's at the core of the library and what differentiates it from other
mocking libraries in Rust.

Other Mocking Libraries

Supports rust stable via code generation and less features:
  - https://github.com/kriomant/mockers

Supports Rust stable, but lots of code boilerplate and less features:
  - https://github.com/iredelmeier/pseudo

Require changing prod code (and thus, can't be used for external `traits`) and require nightly:
  - https://github.com/craftytrickster/mock_me
  - https://github.com/DavidDeSimone/mock_derive
  - https://github.com/CodeSandwich/Mocktopus
  - https://github.com/mindsbackyard/galvanic-mock

[NEXT]
How to use `double`?

[NEXT]
## Defining Mock Collaborators

[NEXT]
**`mock_trait!`**

<pre><code data-noescape class="rust">pub trait Rng {
    fn next_f64(&mut self) -> f64;
}

<mark>mock_trait!(</mark>
<mark>    MockRng,</mark>
<mark>    next_f64() -> f64);</mark>
</code></pre>

[NEXT]
**`mock_trait!`**

```rust
mock_trait!(
    NameOfMockStruct,
    method1_name(arg1_type, ..., argM_type) -> return_type,
    method2_name(arg1_type, ..., argM_type) -> return_type
    ...
    methodN_name(arg1_type, ..., argM_type) -> return_type);
```

[NEXT]
**`mock_method!`**

Generate implementations of all methods in mock `struct`.

<pre><code data-noescape class="rust">pub trait Rng {
    fn next_f64(&mut self) -> f64;
}

mock_trait!(
    MockRng,
    next_f64() -> f64);

<mark>impl Rng for MockRng {</mark>
<mark>    mock_method!(next_f64(&mut self) -> f64);</mark>
<mark>}</mark>
</code></pre>

[NEXT]
**`mock_method!`**

```rust
impl TraitToMock for NameOfMockStruct {

  mock_method!(
    method1_name(&self, arg1_type, ..., argM_type) -> return_type);

  mock_method!(
    method2_name(&mut self, arg1_type, ..., argM_type) -> return_type);

  ...

  mock_method!(
    methodN_name(&mut self, arg1_type, ..., argM_type) -> return_type);

}
```
<!-- .element class="medium-large" -->

[NEXT]
Full code to generate a mock implementation of a `trait`:

```rust
mock_trait!(
    MockRng,
    next_f64() -> f64);

impl Rng for MockRng {
    mock_method!(next_f64(&mut self) -> f64);
}
```

_note_
Emphasise this is the only boilerplate needed.

[NEXT]
Construct mock object:

```rust
let rng = MockRng::default();
```

Configure behaviour:

```rust
rng.next_f64.return_value(0.25);
```

Assert mock was called:

```
assert_eq!(1, rng.next_f64.num_calls());
```

[NEXT]
## Using Generated Mocks in Tests

<pre class="medium"><code data-noescape class="rust">#[test]
fn test_coin_flipper_yielding_heads() {
    // GIVEN:
<mark>    let rng = MockRng::default();</mark>
<mark>    rng.next_f64.return_value(0.25);</mark>

    // WHEN:
    let mut game = CoinFlipper::new(rng);
    let flip = game.flip_coin();

    // THEN:
    assert_eq!(CoinFlip::Heads, flip);
<mark>    assert_eq!(1, rng.next_f64.num_calls());</mark>
}
</code></pre>

[NEXT]
### GIVEN: Setting Mock Behaviour

* Define value to return for mocked method:
  - for all calls
  - for specific input arguments
* Define sequence of values to return
* Define `fn` or closure that transforms input args

_note_
Mocks can be configured to return a single value, a sequence of values (one
value for each call) or invoke a function/closure. Additionally, it is possible
to make a mock return special value /invoke special functions when specific
arguments are passed in.

[NEXT]
```rust
pub trait ProfitForecaster {
    fn profit_at(timestamp: u64) -> f64;
}

pub fn forecast_profit_over_time(&forecaster: &ProfitForecaster,
                                 start: u64,
                                 end: u64) -> Vec<f64>
{
  (start..end)
      .map(|t| forecaster.profit_at(t))
      .collect()
}
```

[NEXT]
```
mock_trait!(
    MockForecaster,
    profit_at(u64) -> f64);

impl ProfitForecaster for MockForecaster {
    mock_method!(profit_at(&self, timestamp: u64) -> f64);
}
```

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn no_return_value_specified() {
  // GIVEN:
  let mock = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
  // default value of return type is used if no value is specified
<mark>  assert_eq!(vec!(0, 0, 0), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn single_return_value() {
  // GIVEN:
  let mock = MockForecaster::default();
<mark>  mock.profit_at.return_value(10);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(10, 10, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn multiple_return_values() {
  // GIVEN:
  let mock = MockForecaster::default();
<mark>  mock.profit_at.return_values(1, 5, 10);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(1, 5, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn return_value_for_specific_arguments() {
  // GIVEN:
  let mock = MockForecaster::default();
<mark>  mock.profit_at.return_value(10);</mark>
<mark>  mock.profit_at.return_value_for((1), 5);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(10, 5, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn using_closure_to_compute_return_value() {
  // GIVEN:
  let mock = MockForecaster::default();
<mark>  mock.profit_at.use_closure(|t| t * 5 + 1);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(0, 6, 11), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn using_closure_for_specific_return_value() {
  // GIVEN:
  let mock = MockForecaster::default();
<mark>  mock.profit_at.return_value(10);</mark>
<mark>  mock.profit_at.use_closure_for((2), |t| t * 5 + 1);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(0, 10, 11), profit_over_time);</mark>
}
</code></pre>

[NEXT]
### Precedence Order

|   |   |
| - | - |
|   | **Behaviour for specific inputs** |
| 0 | `use_closure_for((args), closure)` |
| 1 | `use_fn_for((args), func)` |
| 2 | `return_value_for((args), value)` |
|   | **Behaviour for any inputs** |
| 3 | `use_fn(func)` |
| 4 | `use_closure(closure)` |
| 5 | `return_value(value)` |
|   | **When no behaviour is set** |
| 6 | `ReturnType::default()` |
<!-- .element class="medium-table-text" -->

[NEXT]
### THEN: Code Used Mock as Expected

Verify mocks are called:

* the right number of times
* with the right arguments

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn asserting_mock_was_called() {
  // GIVEN:
  let mock = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
  // called at least once
<mark>  assert!(mock.profit_at.called());</mark>
  // called with argument 1 at least once
<mark>  assert!(mock.profit_at.called_with((1));</mark>
  // called at least once with argument 1 and 0
<mark>  assert!(mock.profit_at.has_calls((1), (0));</mark>
}
</code></pre>

[NEXT]
<pre class="medium"><code data-noescape class="rust">#[test]
fn asserting_mock_was_called_with_precise_constraints() {
  // GIVEN:
  let mock = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(&mock, 0, 3);

  // THEN:
  // called at least once with argument 0 and 1, in that order
<mark>  assert!(mock.profit_at.has_calls_in_order((0), (1));</mark>
  // called exactly three times, once with 0, once with 1 and once with 2
<mark>  assert!(mock.profit_at.has_calls_exactly(</mark>
<mark>      (1), (0), (2));</mark>
  // called exactly three times, once with 0, once with 1 and once with 2,
  // and the calls were made in the specified order
<mark>  assert!(mock.profit_at.has_calls_exactly_in_order(</mark>
<mark>      (0), (1), (2));</mark>
}
</code></pre>

[NEXT]
### Mocking Free Functions

[NEXT]
`double` can also be used to mock free functions.

Useful for testing code that takes function objects for runtime polymorphism.

[NEXT]
<pre class="medium"><code data-noescape class="rust">fn generate_sequence(
    <mark>func: &Fn(i32) -> i32,</mark>
    min: i32,
    max: i32) -> Vec&lt;i32&gt;
{
    // exclusive range
    (min..max).map(func).collect()
}
</pre></code>

[NEXT]
<!-- .slide: class="small-slide" -->
Construct a `double::Mock` object directly.

Format of generic params is: `<(arg_types...), retval_type>`.

```rust
extern crate double;
use double::mock;

let mock = Mock::<(i32), i32>::default();
```

[NEXT]
**`mock_func!`**

Wrap mock object in a closure.

```rust
mock_func!(&mock, retval_type, arg_types...);
```

[NEXT]
<pre class="medium"><code data-noescape class="rust">#[test]
fn test_function_used_correctly() {
    // GIVEN:
<mark>    let mock = Mock::<(i32), i32>::default();</mark>
<mark>    mock.use_closure(Box::new(|x| x * 2));</mark>

    // WHEN:
    let sequence = generate_sequence(
<mark>        &mock_func!(mock, i32, i32),</mark>
        1,
        5);

    // THEN:
    assert_eq!(vec!(2, 4, 6, 8), sequence);
    assert!(mock.has_calls_exactly(vec!(
      1, 2, 3, 4
    )));
}
</code></pre>

[NEXT SECTION]
## 4. Pattern Matching

![pattern_matching](images/pattern_matching.svg)

_note_
When a mock function has been used in a test, we typically want to make assertions about what the mock has been called with.

[NEXT]
### Robot Decision Making
![actuator_large](images/actuator.svg)

[NEXT]
![robot_scenario](images/robot_scenario1.svg)

|              |                                                            |
| ------------ | ---------------------------------------------------------- |
| `WorldState` | Struct containing current state of world. |
| `Robot`      | Processes state of the world and makes decisions on what do to next. |
| `Actuator`   | Manipulates the world. Used by `Robot` to act on the decisions its made. |

[NEXT]
![world_state](images/world_state.svg)

```rust
pub struct WorldState {
    ...
}
```

[NEXT]
![robot](images/robot.svg)

<pre class="medium"><code data-noescape class="rust">pub struct Robot {
    actuator: &mut Actuator
}

impl Robot {
    pub fn new(actuator: &mut Actuator) -> Robot {
        Robot { actuator: actuator }
    }

    pub fn take_action(state: WorldState) {
<mark>        // Complex business logic that decides what actions</mark>
<mark>        // the robot should take.</mark>
<mark>        // This is what we want to test.</mark>
    }
  }
}
</code></pre>

[NEXT]
![actuator](images/actuator.svg)

```rust
pub trait Actuator {
    fn move_forward(&mut self, amount: i32);
    fn speak(&mut self, message: &str, volume: u32);
}
```

[NEXT]
### Testing Robot's Decisions
![robot_scenario](images/robot_scenario2.svg)

[NEXT]
### Testing Robot's Decisions
![robot_scenario](images/robot_scenario3.svg)

[NEXT]
![mock_actuator](images/mock_actuator.svg)

```rust
mock_trait!(
    MockActuator,
    move_forward(i32) -> (),
    speak(String, u32) -> ());

impl Actuator for MockActuator {
    mock_method!(move_forward(&mut self, amount: i32));
    mock_method!(speak(&mut self, message: &str, volume: u32));
}
```

[NEXT]
<pre><code data-noescape class="rust">#[test]
fn test_the_robot() {
    // GIVEN:
    let input_state = WorldState { ... };
    let actuator = MockActuator::default();

    // WHEN:
    {
        let robot = Robot::new(&actuator);
        robot.take_action(input_state);    
    }
    
    // THEN:
<mark>    assert!(actuator.move_forward.called_with(100));</mark>
}
</code></pre>

_note_
For example, suppose we're testing some logic that determines the next action of a robot. We might want to assert what this logic told the robot to do.

[NEXT]
Do we really care that the robot moved **_exactly_** 100 units?

_note_
Sometimes you might not want to be this specific. This can make tests being too rigid. Over specification leads to brittle tests and obscures the intent of tests. Therefore, it is encouraged to specify only what's necessary &mdash; no more, no less.

[NEXT]
<div class="behaviour_space_wrapper">
  ![behaviour_space](images/behaviour_space1.svg)
</div>

[NEXT]
<div class="behaviour_space_wrapper">
  ![behaviour_space](images/behaviour_space2.svg)
</div>

[NEXT]
<div class="behaviour_space_wrapper">
  ![behaviour_space](images/behaviour_space3.svg)
</div>

[NEXT]
<div class="behaviour_space_wrapper">
  ![behaviour_space](images/behaviour_space4.svg)
</div>

[NEXT]
<div class="behaviour_space_wrapper">
  ![behaviour_space](images/behaviour_space5.svg)
</div>

[NEXT]
Behaviour verification can **overfit** the implementation.

Lack of tooling makes this more likely.

_note_
Without proper tooling, developers are more likely to use unnecessarily tight assertions when verifying behaviour.

Writing loose assertions can be surprisingly cumbersome.

[NEXT]
### Pattern Matching to the Rescue

[NEXT]
Match argument values to a patterns.

**_Not exact values._**

Loosens test expectations, making them less brittle.

[NEXT]
**`called_with_pattern()`**

<pre><code data-noescape class="rust">fn is_greater_than_or_equal_to_100(arg: i32) -> bool {
    arg >= 100
}

#[test]
fn test_the_robot() {
    let robot = MockRobot::default();
    test_complex_business_logic_that_makes_decisions(&robot);
<mark>    assert!(robot.move_forward.called_with_pattern(</mark>
<mark>        is_greater_than_or_equal_to_100</mark>
<mark>    ));</mark>
}
</code></pre>

[NEXT]
Parametrised matcher functions:

```rust
/// Matcher that matches if `arg` is greater than or equal to
/// `target_val`.
pub fn ge<T: PartialEq + PartialOrd>(
    arg: &T,
    target_val: T) -> bool
{
    *arg >= target_val
}
```

[NEXT]
Use `p!` to generate matcher functions on-the-fly.

```rust
use double::matcher::ge;

let is_greater_than_or_equal_to_100 = p!(ge, 100);
```

[NEXT]
<pre><code data-noescape class="rust"><mark>use double::matcher::*;</mark>

#[test]
fn test_the_robot() {
    let robot = MockRobot::default();
    test_complex_business_logic_that_makes_decisions(&robot);
<mark>    assert!(robot.move_forward.called_with_pattern(</mark>
<mark>        p!(ge, 100)</mark>
<mark>    ));</mark>
}
</code></pre>

[NEXT]
### Built-in Matchers

[NEXT]
##### Wildcard
|         |                                               |
| ------- | --------------------------------------------- |
| `any()` | argument can be any value of the correct type |
<!-- .element class="medium-table-text" -->

[NEXT]
##### Comparison Matchers
|                    |                                                                 |
| ------------------ | --------------------------------------------------------------- |
| `eq(value)`        | `argument == value`                                             |
| `ne(value)`        | `argument != value`                                             |
| `lt(value)`        | `argument < value`                                              |
| `le(value)`        | `argument <= value`                                             |
| `gt(value)`        | `argument > value`                                              |
| `ge(value)`        | `argument >= value`                                             |
| `is_some(matcher)` | argument is an `Option::Some`, whose contents matches `matcher` |
| `is_ok(matcher)`   | argument is an `Result::Ok`, whose contents matches `matcher`   |
| `is_err(matcher)`  | argument is an `Result::er`, whose contents matches `matcher`   |
<!-- .element class="medium-table-text" -->

[NEXT]
##### Floating-Point Matchers
|                               |                                                                                             |
| ----------------------------- | ------------------------------------------------------------------------------------------- |
| `f32_eq(value)`               | argument is a value approximately equal to the `f32` `value`, treating two NaNs as unequal. |
| `f64_eq(value)`               | argument is a value approximately equal to the `f64` `value`, treating two NaNs as unequal. |
| `nan_sensitive_f32_eq(value)` | argument is a value approximately equal to the `f32` `value`, treating two NaNs as equal.   |
| `nan_sensitive_f64_eq(value)` | argument is a value approximately equal to the `f64` `value`, treating two NaNs as equal.   |
<!-- .element class="medium-table-text" -->

[NEXT]
##### Container Matchers
|                                    |                                                                                                               |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `is_empty`                         | argument implements `IntoIterator` and contains no elements.                                                  |
| `has_length(size_matcher)`         | argument implements `IntoIterator` whose element count matches `size_matcher`.                                |
| `contains(elem_matcher)`           | argument implements `IntoIterator` and contains at least one element that matches `elem_matcher`.             |
| `each(elem_matcher)`               | argument implements `IntoIterator` and all of its elements match `elem_matcher`.                              |
| `unordered_elements_are(elements)` | argument implements `IntoIterator` that contains the same elements as the vector `elements` (ignoring order). |
| `when_sorted(elements)`            | argument implements `IntoIterator` that, when its elements are sorted, matches the vector `elements`.         |
<!-- .element class="small-table-text" -->

[NEXT]
##### String Matchers
|                       |                                                   |
| --------------------- | ------------------------------------------------- |
| `contains(string)`    | argument contains `string` as a sub-string.       |
| `starts_with(prefix)` | argument starts with string `prefix`.             |
| `starts_with(suffix)` | argument ends with string `suffix`.               |
| `eq_nocase(string)`   | argument is equal to `string`, ignoring case.     |
| `ne_nocase(value)`    | argument is not equal to `string`, ignoring case. |
<!-- .element class="medium-table-text" -->


[NEXT]
### Composite Matchers

[NEXT]
Assert that a single arg should match many patterns.

```rust
// Assert robot moved between 100 and 200 units.
assert!(robot.move_forward.called_with_pattern(
    p!(all_of, vec!(
        p!(ge, 100),
        p!(le, 200)
    ))
));
```

[NEXT]
|                            |                                                    |
| -------------------------- | -------------------------------------------------- |
| `all_of(vec!(m1, ... mn))` | argument matches all of the matchers `m1` to `mn`. |
| `any_of(vec!(m1, ... mn))` | matches at least one of the matchers `m1` to `mn`. |
| `not(m)`                   | argument doesn't match matcher `m`.                |
<!-- .element class="medium-table-text" -->

[NEXT]
### Methods with Multiple Arguments

<pre><code data-noescape class="rust">pub trait Actuator {
    fn move_forward(&mut self, amount: i32);
<mark>    fn speak(&mut self, message: &str, volume: u32);</mark>
}
</code></pre>

[NEXT]
**`matcher!`**

Create tuple of arg matchers for multi-arg methods.

<pre><code data-noescape class="rust">use double::matcher::*;

#[test]
fn test_the_robot() {
    let robot = MockRobot::default();
    test_complex_business_logic_that_makes_decisions(&robot);
<mark>    assert!(robot.speak.called_with_pattern(</mark>
<mark>        matcher!( contains("Hello FOSDEM"), ge(0.7) )</mark>
<mark>    ));</mark>
}
</code></pre>

[NEXT]
### Custom Matchers

_note_
If none of the built-in matchers fit your use case, you can define your own.

[NEXT]
### Custom Matchers

Let's test a HTTP request processor.

It responds to clients with JSON.

_note_
Suppose we were testing a restful service. We have some request handling logic. We want to test the handling logic responded to the request correctly. In this context, "correctly" means it responded with a JSON object that contains the "time" key.

[NEXT]
```rust
trait ResponseSender {
    fn send_response(&mut self, response: String);
}

fn request_handler(response_sender: &mut ResponseSender) {
    let num_records = /* ... business logic here ... */
    let response = format!(
        "{{ \"num_records\": {} }}",
        num_records);
    response_sender.send_response(response);
}
```

[NEXT]
Step 1: Mock the relevant `trait`.

```rust
mock_trait!(
    MockResponseSender,
    send_response(&str) -> ());

impl ResponseSender for MockResponseSender {
    mock_method!(send_response(&mut self, response: &str));
}
```

[NEXT]
Step 2: Write the test.

<pre><code data-noescape class="rust">#[test]
fn ensure_num_records_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();

    // WHEN:
    request_handler(&mock_sender);

    // THEN:
<mark>    // check the sender received a response that contains a</mark>
<mark>    // "num_records" field</mark>
}
</code></pre>

[NEXT]
How do we test this?

[NEXT]
Could check using exact string equality...

<pre><code data-noescape class="rust">#[test]
fn ensure_num_records_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();

    // WHEN:
    request_handler(&mock_sender);

    // THEN:
<mark>    assert!(mock_sender.send_response.called_with(</mark>
<mark>        "{ \"num_records\": 42 }"</mark>
<mark>    ));</mark>
}
</code></pre>

[NEXT]
Test is now tightly bound to implementation. Will break if:

* JSON spacing/formatting changes
* the value of "num_records" changes
* there are other fields in the response

**It's a brittle test!**

[NEXT]
Could use a substring matcher...

<pre><code data-noescape class="rust">#[test]
fn ensure_num_records_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();

    // WHEN:
    request_handler(&mock_sender);

    // THEN:
<mark>    assert!(mock_sender.send_response.called_with_pattern(</mark>
<mark>        p!(contains, "\"num_records\"")</mark>
<mark>    ));</mark>
}
</code></pre>

[NEXT]
No guarantee `num_records` is stored in the root JSON object.

[NEXT]
Could extract the call arg, parse it as JSON and check the field exists.

<pre class="small"><code data-noescape class="rust">extern crate json;
use self::json;

#[test]
fn ensure_num_records_field_is_returned() {
    ...

    // THEN:
<mark>    let calls = mock_sender.calls();</mark>
<mark>    assert_eq!(1, calls.size());</mark>

    match json::parse(calls[0]) {
        Ok(json_value) => match json_value {
            Object(object) => match object.get("num_records") {
                Some(_) => {}  // JSON object and has key -- success!
                None => panic!("JSON object but doesn't have key");
            },
            _ => panic!("not an object (must be another JSON type)");
        },
        Err(_) => panic!("not valid JSON");
    }
}
</code></pre>

[NEXT]
**Verbose.**

What if we want to test many requests types/cases?

This approach would result in lots of **copy/paste**.

[NEXT]
#### Solution

Define a custom matcher.

[NEXT]
```rust
extern crate json;
use self::json;

fn is_json_object_with_key(arg: &str, key: &str) -> bool {
    match json::parse(str) {
        Ok(json_value) => match json_value {
            Object(object) => match object.get(key) {
                Some(_) => true // JSON object and has key
                None => false   // JSON object but doesn't have key
            },
            _ => false  // not an object (must be another JSON type)
        },
        Err(_) => false  // not valid JSON
    }
}
```

[NEXT]
<pre><code data-noescape class="rust">fn ensure_num_records_field_is_returned() {
    // GIVEN:
    let mut mock_sender = MockResponseSender::default();
    // WHEN:
    request_handler(&mock_sender);
    // THEN:
    // expect a "num_records" field to be in the response JSON
    assert(response_sender.send_response.called_with_pattern(
<mark>        p!(is_json_object_with_key, "num_records")</mark>
    ));
    // DO NOT expect an "error" field to be in the response JSON
    assert(!response_sender.send_response.called_with_pattern(
<mark>        p!(is_json_object_with_key, "error")</mark>
    ));
}
</code></pre>

_note_
Using the matcher then requires binding it to a parameter (using `p!`) and passing it to a mock assertion method.

[NEXT]
### Interim Summary

Mocking/behaviour verification can **overfit implementation**.

Pattern matching **expands the asserted behaviour space**.

Reduces overfitting.

[NEXT]
### However...

Pattern matching tests are cumbersome to write manually.

`double` has built-in support for patterns to make it easier.

_note_
Writing assertions that use pattern matching is often cumbersome to write.

This encourages developers to use eact value matching and overfit the implementation, because it's easier.

Mocking libraries like `double` have tooling to make this easy, encouraging developers to write looser assertions.


[NEXT SECTION]
## 5. Library Constraints
![limitations](images/limitations.svg)

[NEXT]
### Core Design Philosophy of `double`

1. Rust stable first
2. No changes to production code required

_note_
The vision for `double` is that must work with stable Rust. It must don't impose code changes to the user's production code either. This makes supporting some features difficult.

[NEXT]
#### 1. Rust Stable First

Almost all mocking libraries require nightly.

_note_
The vast majority of other mocking libraries that use nightly compiler plugins. This gives them more flexibility at the cost of restricting the user to nightly Rust.

[NEXT]
#### 2. No Changes to Production Code Required

All current mocking libraries require users changes to production code.

Makes mocking `traits` from the standard library or external crates **impossible**.

_note_
The following other mocking libraries have similar feature sets to `double`, require nightly:
  * mockers (has partial support for stable)
  * mock_derive
  * galvanic-mock
  * mocktopus

And none of them support mocking traits from the standard library or external crates.

[NEXT]
**Rust stable and no code changes comes at a cost.**

[NEXT]
### Main Limitation
##### Traits with Generic Types

```rust
trait Comparator {
   fn is_equal<T: PartialEq>(&self, a: &T, b: &T) -> bool;
}
```

[NEXT]
These can be mocked using `double`.

Sometimes requires boilerplate to be added to production code.

_note_
The detailed reasons why this is currently the case is out of scope for this talk. Note that there is ongoing work on allowing these traits to be used without neeeding production code changes.

Speak to me after this talk if you're interested in learning more about this.

[NEXT]
### All Limitations

1. Argument types must implement `Clone` and `Eq`
2. Return value types must implement `Clone`
3. Private `trait`s cannot be mocked
4. Limited support for generic type arguments

_note_
(1) and (2) are constraints caused by the current implementation of `double`.

I believe no existing mocking library, using nightly or otherwise, have properly solved (3) and (4).

[NEXT]
Ongoing work on `double` to remove these limitations.

_note_
There are a lot of nightly features that would make removing these constraints much easier.

However, I believe none of these constraints are impossible to solve using the current Rust stable compiler.

As such, there are ongoing work on the library to remove these limitations.


[NEXT SECTION]
## Fin
![fin](images/fin.svg)

[NEXT]
Mocking can be used to isolate unit tests from exernal resources or complex dependencies.

Mocking can be achieved in Rust by dummying `trait`s and functions.

[NEXT]
Mocking is controversial.

Used incorrectly, tests with mocks overfit the implementation.

Which introduces a huge burden on development.

**Using pattern matching to loosen test constraints reduces overfitting.**

_note_
Mocking has often been a divisive topic.

[NEXT]
`double` is a crate for generating `trait`/function mocks.

Wide array of mock behaviours and call assertions.

First-class pattern matching support.

_note_
First-class pattern matching for writing non-brittle mocking tests.

[NEXT]
`double` supports stable and needs no prod code changes.

Introduces constraints.

Requires users to write more boilerplate code.

_note_
Ongoing work on the library to remove these limitations.

[NEXT]
### Alternative Mocking Libraries

* [mockers](https://github.com/kriomant/mockers)
* [mock_derive](https://github.com/DavidDeSimone/mock_derive)
* [galvanic-mock](https://github.com/mindsbackyard/galvanic-mock)
* [mocktopus](https://github.com/CodeSandwich/Mocktopus)

_note_
For completeness, here's a list of other Rust mocking crates. In additional to checking out `double`, I encourage you to look at these too. Depending on your use case and preference, one of these might be more suitable for you.

[NEXT]
<!-- .slide: class="small-slide" -->
### Links

* these slides:
  - http://donsoft.io/mocking-in-rust-using-double
* double repository:
  - https://github.com/DonaldWhyte/double
* double documentation:
  - https://docs.rs/double/0.2.0/double/
* example code from this talk:
  - https://github.com/DonaldWhyte/mocking-in-rust-using-double/tree/master/code

[NEXT]
### Get In Touch

<div class="left-col" style="text-center: left">
  <br />
  [don@donsoft.io](mailto:don@donsoft.io)<br />
  [@donald_whyte](http://twitter.com/donald_whyte)<br />
  <span class="github">https://github.com/DonaldWhyte</span>
</div>
<div class="right-col">
  ![small_portrait](images/donald.jpg)
</div>
<div class="clear-col"></div>

[NEXT]
## Questions?

[NEXT]
### Image Credits

* [Gregor Cresnar](https://www.flaticon.com/authors/gregor-cresnar)
* [Zurb](https://www.flaticon.com/authors/zurb)
* [Freepik](http://www.flaticon.com/authors/freepik)
* [Dave Gandy](http://fontawesome.io/)
* [Online Web Fonts](https://www.onlinewebfonts.com/icon/548360)

[NEXT SECTION]
## Appendix

[NEXT]
### Classist vs. Mockist

TODO


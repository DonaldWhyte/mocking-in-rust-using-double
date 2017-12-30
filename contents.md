# Mocking in Rust

<p>
    <a href="http://donaldwhyte.co.uk">Donald Whyte</a>
    / <a href="http://twitter.com/donald_whyte">@donald_whyte</a>
</p>

[NEXT]
### About Me

<table class="bio-table">
  <tr>
    <td>![portrait](images/donald.jpg)</td>
  </tr>
  <tr>
    <td>TODO</td>
  </tr>
</table>

[NEXT]
## Outline

1. Unit Testing in Rust
2. Why Mock?
3. Mocking in Rust with `double`
4. Pattern Matching
5. Advanced `double` Features
6. Rust Limitations


[NEXT SECTION]
## 1. Unit Testing

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
Create library:

```bash
cargo new some_lib
cd some_lib
```

Test fixture automatically generated:

```rust
> cat src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // test code in here
    }
}
```
<!-- .element class="small" -->

[NEXT]
Run all tests:

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

Annotate module so it's only built with `cargo test`.

`src/foo.rs`:

```rust
// production code
pub fn add_two(num: i32) -> i32 {
    num + 2
}

// annotate module with this to ensure this code is not built
// into the production binary
#[cfg(test)]
mod tests {
    // test code in here
}
```

[NEXT]
Add test functions to private `tests` module.

Each function is a separate, isolated test.

```rust
// ...prod code...

#[cfg(test)]
mod tests {
    use super::*;  // import production symbols from parent module

    // Annotate functions with `#[test]` so they're execute by `cargo test`
    #[test]
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
```
<!-- .element class="medium" -->

[NEXT]
```bash
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

Anything non-deterministic that can't be reliably controlled within a unit test

* External data sources (e.g. files, databases)
* Network connections (e.g. services)
* External code dependencies (libraries)
* Internal code dependencies
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

* A **test double** is an object or function substituted for a "real" (production ready) object during testing.
* Should appear exactly the same as a "real" production instance to its clients (collaborators).
* Term originates from a notion of a _"stunt double"_ in films

_note_
This is *how* we eliminate these unwanted dependencies from our tests.

Similar to using a stunt double in films, where viewers don't notice that
stunts are performed by a different actor.

[NEXT]
## Types

* **Stubs** return hard-coded values
* **Spies** record the code's interaction with collaborators
    * times method called and passed arguments
* **Mocks** return hard-coded values and verify interaction
    * both a stub and a spy

[NEXT]
**Mocks** are the focus of this talk

_note_
WHY? Mocks are the most flexible. They're a superset of stubs and spies.

[NEXT SECTION]
## 3. Mocking in Rust Using `double`

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

enum CoinFlip {
    Heads,
    Tails,
}
```

[NEXT]
## Implementation

TODO: compile and test

```cpp
struct CoinFlipper {
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
<!-- .element class="small" -->

[NEXT]
## Playing the Game

```cpp
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

`flip` is either `Heads` or `Tails`

[NEXT]
![coin_flip_collaborators](images/coin-flip-collaborators.png)

* One collaborator &mdash; `Rng`
* Real RNG is non-deterministic
* We want to test `CoinFlipper` produces both results
  - we also want these tests to be repeatable
  - without relying on an external environment
* Have to mock `Rng`

[NEXT]
## Double to the Rescue!

* create mock trait implementations using simple macros
* rich set of **matchers**
* TODO: other good stuff

[NEXT]
## Defining Mock Collaborators

Generate mock `struct`:

```rust
mock_trait!(
    NameOfMockStruct,
    method1_name(arg1_type, ..., argM_type) -> return_type,
    method2_name(arg1_type, ..., argM_type) -> return_type
    ...
    methodN_name(arg1_type, ..., argM_type) -> return_type);
```

```rust
mock_trait!(
    MockRng,
    next_f64() -> f64);
```

[NEXT]
## Defining Mock Collaborators

Generate implementations of all methods in mock `struct`:

```
impl TraitToMock for NameOfMockStruct {
  mock_method!(method1_name(&self, arg1_type, ..., argM_type) -> return_type);
  mock_method!(method2_name(&mut self, arg1_type, ..., argM_type) -> return_type);
  ...
  mock_method!(methodN_name(&mut self, arg1_type, ..., argM_type) -> return_type);
}
```
<!-- .element class="small" -->

```
impl Rng for MockRng {
    mock_method!(next_f64(&mut self) -> f64);
}
```

[NEXT]
## Using Generated Mocks in Tests

```rust
#[test]
fn test_coin_flipper_yielding_heads() {
    // GIVEN:
    let rng = MockRng::default();
    rng.next_f64.return_value(0.25);

    // WHEN:
    let mut game = CoinFlipper::new(rng);
    let flip = game.flip_coin();

    // THEN:
    assert_eq!(CoinFlip::Heads, flip);

    assert!(rng.next_f64.called());
    assert!(rng.next_f64.called_with(()));
    assert_eq!(1, rng.next_f64.num_calls());
}
```
<!-- .element class="medium" -->

[NEXT]
#### GIVEN: Setting Mock Behaviour

* Define value to return for mocked method:
  - for all calls
  - for specific input arguments
* Define sequence of values to return
* Define `fn` or closure that transforms input args§

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

pub fn forecast_profit_over_time(forecaster: &ProfitForecaster,
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
<pre><code data-noescape class="rust">fn no_return_value_specified() {
  // GIVEN:
  let forecaster = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
  // default value of return type is used if no value is specified
<mark>  assert_eq!(vec!(0, 0, 0), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn single_return_value() {
  // GIVEN:
  let forecaster = MockForecaster::default();
<mark>  forecaster.profit_at.return_value(10);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(10, 10, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn multiple_return_values() {
  // GIVEN:
  let forecaster = MockForecaster::default();
<mark>  forecaster.profit_at.return_values(1, 5, 10);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(1, 5, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn return_value_for_specific_arguments() {
  // GIVEN:
  let forecaster = MockForecaster::default();
<mark>  forecaster.profit_at.return_value(10);</mark>
<mark>  forecaster.profit_at.return_value_for((1), 5);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(10, 5, 10), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn using_closure_to_compute_return_value() {
  // GIVEN:
  let forecaster = MockForecaster::default();
<mark>  forecaster.profit_at.use_closure(|t| t * 5 + 1);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(0, 6, 11), profit_over_time);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn using_closure_for_specific_return_value() {
  // GIVEN:
  let forecaster = MockForecaster::default();
<mark>  forecaster.profit_at.return_value(10);</mark>
<mark>  forecaster.profit_at.use_closure_for((2), |t| t * 5 + 1);</mark>

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
<mark>  assert_eq!(vec!(0, 10, 11), profit_over_time);</mark>
}
</code></pre>

[NEXT]
Highest precedence to lowest.

* Behaviour for specific input args:
  - `use_closure_for((args), closure)`
  - `use_fn_for((args), func)`
  - `return_value_for((args), value)`
* Behaviour for all input args:
  - `use_fn(func)`
  - `use_closure(closure)`
  - `return_value(value)`
* When no behaviour is set:
  - `ReturnType::default()`

[NEXT]
#### `Option` Helpers

Use `return_some` and `return_none` for `Option<T>` return values.

```rust
struct User { };

pub trait UserStore {
    fn get_user(&self, id: u64) -> Option<User>;
}

mock_trait!(
    MockUserStore,
    get_user(u64) -> Option<User>);

impl UserStore for MockUserStore {
    mock_method!(get_user(&self, id: u64) -> Option<User>);
}
```

[NEXT]
<pre><code data-noescape class="rust">fn returning_none() {
  // GIVEN:
  let store = MockUserStore::default();
<mark>  store.get_user.return_none();</mark>

  // WHEN:
  let output = store.get_user(42);

  // THEN:
<mark>  assert_eq!(None, output);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn returning_some() {
  // GIVEN:
  let store = MockUserStore::default();
<mark>  store.get_user.return_some(User{});</mark>

  // WHEN:
  let output = store.get_user(42);

  // THEN:
<mark>  assert_eq!(Some(User{}), output);</mark>
}
</code></pre>

[NEXT]
#### `Result` Helpers

Use `return_ok` and `return_err` for `Result<T>` return values.

```rust
struct User { };

pub trait UserStore {
    fn get_user(&self, id: u64) -> Result<User, String>;
}

mock_trait!(
    MockUserStore,
    get_user(u64) -> Result<User, String>);

impl UserStore for MockUserStore {
    mock_method!(get_user(&self, id: u64) -> Result<User, String>);
}
```

[NEXT]
<pre><code data-noescape class="rust">fn returning_error() {
  // GIVEN:
  let store = MockUserStore::default();
<mark>  store.get_user.return_err("could not connect to DB");</mark>

  // WHEN:
  let output = store.get_user(42);

  // THEN:
<mark>  assert_eq!(Err("could not connect to DB"), output);</mark>
}
</code></pre>

[NEXT]
<pre><code data-noescape class="rust">fn returning_ok() {
  // GIVEN:
  let store = MockUserStore::default();
<mark>  store.get_user.return_ok(User{});</mark>

  // WHEN:
  let output = store.get_user(42);

  // THEN:
<mark>  assert_eq!(Ok(User{}), output);</mark>
}
</code></pre>

[NEXT]
#### THEN: Asserting Mock Was Used in the Expected Way

Verify mocks are called the right number of times and with the right arguments.

[NEXT]
<pre><code data-noescape class="rust">fn asserting_mock_was_called() {
  // GIVEN:
  let forecaster = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
  // called at least once
<mark>  assert!(forecaster.profit_at.called());</mark>
  // called with argument 1 at least once
<mark>  assert!(forecaster.profit_at.called_with((1));</mark>
  // called at least once with argument 1 and 0
<mark>  assert!(forecaster.profit_at.has_calls((1), (0));</mark>
}
</code></pre>

[NEXT]
<pre class="medium"><code data-noescape class="rust">fn asserting_mock_was_called_with_precise_constraints() {
  // GIVEN:
  let forecaster = MockForecaster::default();

  // WHEN:
  let profit_over_time = forecast_profit_over_time(forecaster, 0, 3);

  // THEN:
  // called at least once with argument 0 and 1, in that order
<mark>  assert!(forecaster.profit_at.has_calls_in_order((0), (1));</mark>
  // called exactly three times, once with 0, once with 1 and once with 2
<mark>  assert!(forecaster.profit_at.has_calls_exactly(</mark>
<mark>      (1), (0), (2));</mark>
  // called exactly three times, once with 0, once with 1 and once with 2,
  // and the calls were made in the specified order
<mark>  assert!(forecaster.profit_at.has_calls_exactly_in_order(</mark>
<mark>      (0), (1), (2));</mark>
}
</code></pre>


[NEXT SECTION]
## 4. Pattern Matching

TODO


[NEXT SECTION]
## 5. Rust Limitations

[NEXT]
TODO: mention that the vision for this library that this must be usable in `stable`

TODO: there exist many other mocking libraries that use nightly compiler plugins

TODO: this makes supporting some features difficutl


[NEXT]
### `double` Limitations

* Argument/return value types must implement these traits:
  - `Clone`
  - `Debug`
  - `Eq`
  - `Hash`
* Return value type must also implement:
  - `Default`
* Only `pub trait`s can be mocked

_note_
TODO: add explanation in notes for why each is implemented

[NEXT]
### `&str` Arguments

TODO

[NEXT]
### Generic Type Arguments

TODO

[NEXT]
### Methods that Return References

TODO

[NEXT]
TODO: general takeaways of stable Rust limitations


[NEXT SECTION]
## Fin

[NEXT]
TODO: conclusion

[NEXT]
<!-- .slide: class="small" -->
### Example Code in this Talk
https://github.com/DonaldWhyte/mocking-in-rust-using-double/tree/master/code

### Slides
http://donsoft.io/mocking-in-rust-using-double

### double Repository
https://github.com/DonaldWhyte/double

[NEXT]
### Get In Touch

<table class="bio-table">
  <tr>
    <td>![small_portrait](images/donald.jpg)</td>
  </tr>
  <tr>
    <td>
      [don@donsoft.io](mailto:don@donsoft.io)<br />
      [@donald_whyte](http://twitter.com/donald_whyte)<br />
      <span class="github">https://github.com/DonaldWhyte</span>
    </td>
  </tr>
</table>

[NEXT]
### Sources

> TODO

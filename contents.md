## Mocking in Rust
#### Using Double

<p>
    <a href="http://donaldwhyte.co.uk">Donald Whyte</a>
    / <a href="http://twitter.com/donald_whyte">@donald_whyte</a>
</p>

[NEXT]
### About Us

<table class="bio-table">
  <tr>
    <td>![portrait](images/donald.jpg)</td>
  </tr>
  <tr>
    <td><strong>Donald Whyte</strong></td>
  </tr>
</table>

[NEXT]
TODO: intro

[NEXT SECTION]
## 1. Unit Testing

_note_
* classist vs mockist testing
    - look up newer literature for this
* say that we're going to start w/ classist testing then move to mockist
* basic Rust unit test
* chosen unit test framework
* same unit tests as before but in new framework

[NEXT SECTION]
## 2. Pattern Matching

_note_
* more complex examples that require pattern matching


[NEXT SECTION]
## 3. Doctests

_note_
* explain what doctests are and given an example
* discuss when you might want to use module tests vs. doctests


[NEXT SECTION]
## 4. Mocking

_note_
* Test double definition and types of doubles
* Introduce `double` crate
* Basic examples
    - use examples to explain how double works internally at a basic level
* More advanced usages
    - mutable and immutable functions
    - exactly call matching
    - "has call" matching
    - unordered calls
    - setting action as return value
        - single call
        - multiple calls
    - setting action as closure
    - has examples for return helpers (some/none/err)
    - cover mocking `&str` values explicitly (TODO: add a helper to the lib for this?)
    - mocking methods with a generic type parameter


[NEXT SECTION]
## 5. Fuzz Testing

_note_
TODO: find a tool that does this in Rust and base section structure on that


[NEXT SECTION]
## 6. Other Cool Things

__note_
Add anything here you find that's cool but doesn't fit into other sections.


[NEXT SECTION]
## Fin

[NEXT]
TODO: conclusion

[NEXT]
<!-- .slide: class="small" -->
### Code
https://github.com/DonaldWhyte/testing-and-mocking-in-rust

### Slides
http://donsoft.io/testing-and-mocking-in-rust

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

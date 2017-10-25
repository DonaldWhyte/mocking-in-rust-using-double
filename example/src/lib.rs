/* TODO: decide what the example should be. This is what we wish to demonstrate:
 *
 * non-trivial business logic
 *     - start off simple but complexity quickly grows
 *     - Rust test modules ---> then move to unit test framework
 *         TODO: only do above if unit testing framework actually adds significant
 *               value
 *     - requires argument matches (Hamcrest style)
 *     - TODO: maybe example that is suitable for other helper Rust tools?
 *     - should be an example that means using BOTH module-level tests and doctests
 *       are reasonable
 * talks to external DB/service dependencies
 *     - for mocking
 *     - hits many of the mocking use cases, such as:
 *         - mutable and immutable functions
 *         - exactly call matching
 *         - "has call" matching
 *         - unordered calls
 *         - setting action as return value
 *             - single call
 *             - multiple calls
 *         - setting action as closure
 *         - has examples for return helpers (some/none/err)
 *         - cover mocking `&str` values explicitly (TODO: add a helper to the lib for this?)
 *         - mocking methods with a generic type parameter
 * examples combiing hamcrest matchers + mock expectations!!!
 * fuzz testing (generating loads of different inputs)
*/

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }

}

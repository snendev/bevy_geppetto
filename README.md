## Bevy Geppetto

A proof-of-concept snapshot testing crate for Bevy.

This crate began as a simple end-to-end test runner forked from the ideas in
https://chadnauseam.com/coding/gamedev/automated-testing-in-bevy/.

After a while, I found it useful to capture the inputs of my test runs, so that
I did not have to repeatedly run the same inputs to evaluate test behavior. Long
term, I'd like to update this to add the capacity to capture and compare the
visual output of these test runs in some way, similar to end-to-end test suites
for JS/web ecosystem. For the time being, that's somewhat out of scope.

This crate is not published anywhere because it is not really ready for wide
use, and does very little other than attach common Bevy plugins. If you are
interested in using it, feel free to import it via `git` or let me know by
adding an issue.

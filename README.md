## Bevy Geppetto

A proof-of-concept snapshot testing crate for Bevy.

This crate began as a simple end-to-end test runner forked from the ideas in
https://chadnauseam.com/coding/gamedev/automated-testing-in-bevy/.

After a while, I found it useful to capture the inputs of my test runs, so that
I did not have to repeatedly run the same inputs to evaluate test behavior.
Similarly, after finding the bevy_media_capture crate, I thought it would be
cool to build a full snapshot testing mechanism.

Ultimately, when in test mode, this should compare video output. For now, it
only shows the bare bones of the ideal future.

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

### Running Tests

Tests run with `bevy_geppetto` cannot run on the default test harness since they
must be run on the main thread to satisfy winit. One approach is to add test
declarations in an isolated `integration` or `e2e` folder, with the following
declaration to `Cargo.toml`:

```toml
[[test]]
name = "<name>"
path = "e2e/<name>.rs"
```

Such a test could then be run with `cargo test --test <name>`. This runs the
given Bevy program with the following behaviors attached to `App`:

- DefaultPlugins added, with `return_from_run` specified
- bevy_inspector_egui::WorldInspectorPlugin added
- `bevy::window::close_on_esc` added in the `Update` schedule

Inputs are saved in the `snapshots` directory under a subdirectory `inputs`. The
root of the snapshot directory can be changed with the `SNAPSHOTS_DIR` env
variable. The filename of the snapshot will be a kebab-case version of the
string passed as the test's `label`, and contains one serialized RON object with
all input events per line.

To capture inputs during a test run, pass the `-c` flag like so:

```sh
cargo test --test $TEST_NAME -- -c
```

To replay inputs from a previous snapshot, pass the `-r` flag similarly:

```sh
cargo test --test $TEST_NAME -- -r
```

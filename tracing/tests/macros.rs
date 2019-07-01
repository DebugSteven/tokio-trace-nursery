#![deny(warnings)]
use tracing::Level;

#[macro_use]
extern crate tracing;
// Tests that macros work across various invocation syntax.
//
// These are quite repetitive, and _could_ be generated by a macro. However,
// they're compile-time tests, so I want to get line numbers etc out of
// failures, and producing them with a macro would muddy the waters a bit.

#[test]
fn span() {
    span!(target: "foo_events", Level::DEBUG, "foo", bar.baz = ?2, quux = %3, quuux = 4);
    span!(target: "foo_events", Level::DEBUG, "foo", bar.baz = 2, quux = 3);
    span!(target: "foo_events", Level::DEBUG, "foo", bar.baz = 2, quux = 4,);
    span!(target: "foo_events", Level::DEBUG, "foo");
    span!(target: "foo_events", Level::DEBUG, "bar",);
    span!(Level::DEBUG, "foo", bar.baz = 2, quux = 3);
    span!(Level::DEBUG, "foo", bar.baz = 2, quux = 4,);
    span!(Level::DEBUG, "foo", bar.baz = 2, quux = 3);
    span!(Level::DEBUG, "foo", bar.baz = 2, quux = 4,);
    span!(Level::DEBUG, "foo", bar.baz = ?2);
    span!(Level::DEBUG, "foo", bar.baz = %2);
    span!(Level::DEBUG, "foo");
    span!(Level::DEBUG, "bar",);
}

#[test]
fn trace_span() {
    trace_span!(target: "foo_events", "foo", bar.baz = ?2, quux = %3, quuux = 4);
    trace_span!(target: "foo_events", "foo", bar.baz = 2, quux = 3);
    trace_span!(target: "foo_events", "foo", bar.baz = 2, quux = 4,);
    trace_span!(target: "foo_events", "foo");
    trace_span!(target: "foo_events", "bar",);
    trace_span!("foo", bar.baz = 2, quux = 3);
    trace_span!("foo", bar.baz = 2, quux = 4,);
    trace_span!("foo", bar.baz = ?2);
    trace_span!("foo", bar.baz = %2);
    trace_span!("bar");
    trace_span!("bar",);
}

#[test]
fn debug_span() {
    debug_span!(target: "foo_events", "foo", bar.baz = ?2, quux = %3, quuux = 4);
    debug_span!(target: "foo_events", "foo", bar.baz = 2, quux = 3);
    debug_span!(target: "foo_events", "foo", bar.baz = 2, quux = 4,);
    debug_span!(target: "foo_events", "foo");
    debug_span!(target: "foo_events", "bar",);
    debug_span!("foo", bar.baz = 2, quux = 3);
    debug_span!("foo", bar.baz = 2, quux = 4,);
    debug_span!("foo", bar.baz = ?2);
    debug_span!("foo", bar.baz = %2);
    debug_span!("bar");
    debug_span!("bar",);
}

#[test]
fn info_span() {
    info_span!(target: "foo_events", "foo", bar.baz = ?2, quux = %3, quuux = 4);
    info_span!(target: "foo_events", "foo", bar.baz = 2, quux = 3);
    info_span!(target: "foo_events", "foo", bar.baz = 2, quux = 4,);
    info_span!(target: "foo_events", "foo");
    info_span!(target: "foo_events", "bar",);
    info_span!("foo", bar.baz = 2, quux = 3);
    info_span!("foo", bar.baz = 2, quux = 4,);
    info_span!("foo", bar.baz = ?2);
    info_span!("foo", bar.baz = %2);
    info_span!("bar");
    info_span!("bar",);
}

#[test]
fn warn_span() {
    warn_span!(target: "foo_events", "foo", bar.baz = ?2, quux = %3, quuux = 4);
    warn_span!(target: "foo_events", "foo", bar.baz = 2, quux = 3);
    warn_span!(target: "foo_events", "foo", bar.baz = 2, quux = 4,);
    warn_span!(target: "foo_events", "foo");
    warn_span!(target: "foo_events", "bar",);
    warn_span!("foo", bar.baz = 2, quux = 3);
    warn_span!("foo", bar.baz = 2, quux = 4,);
    warn_span!("foo", bar.baz = ?2);
    warn_span!("foo", bar.baz = %2);
    warn_span!("bar");
    warn_span!("bar",);
}

#[test]
fn error_span() {
    error_span!(target: "foo_events", "foo", bar.baz = ?2, quux = %3, quuux = 4);
    error_span!(target: "foo_events", "foo", bar.baz = 2, quux = 3);
    error_span!(target: "foo_events", "foo", bar.baz = 2, quux = 4,);
    error_span!(target: "foo_events", "foo");
    error_span!(target: "foo_events", "bar",);
    error_span!("foo", bar.baz = 2, quux = 3);
    error_span!("foo", bar.baz = 2, quux = 4,);
    error_span!("foo", bar.baz = ?2);
    error_span!("foo", bar.baz = %2);
    error_span!("bar");
    error_span!("bar",);
}

#[test]
fn span_root() {
    span!(target: "foo_events", parent: None, Level::TRACE, "foo", bar.baz = 2, quux = 3);
    span!(target: "foo_events", parent: None, Level::TRACE, "foo", bar.baz = 2, quux = 3);
    span!(target: "foo_events", parent: None, Level::TRACE, "foo", bar.baz = 2, quux = 4,);
    span!(target: "foo_events", parent: None, Level::TRACE, "foo");
    span!(target: "foo_events", parent: None, Level::TRACE, "bar",);
    span!(parent: None, Level::DEBUG, "foo", bar.baz = 2, quux = 3);
    span!(parent: None, Level::DEBUG, "foo", bar.baz = 2, quux = 4,);
    span!(parent: None, Level::DEBUG, "foo");
    span!(parent: None, Level::DEBUG, "bar",);
}

#[test]
fn trace_span_root() {
    trace_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 3);
    trace_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 4,);
    trace_span!(target: "foo_events", parent: None, "foo");
    trace_span!(target: "foo_events", parent: None, "bar",);
    trace_span!(parent: None, "foo", bar.baz = 2, quux = 3);
    trace_span!(parent: None, "foo", bar.baz = 2, quux = 4,);
    trace_span!(parent: None, "foo");
    trace_span!(parent: None, "bar",);
}

#[test]
fn debug_span_root() {
    debug_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 3);
    debug_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 4,);
    debug_span!(target: "foo_events", parent: None, "foo");
    debug_span!(target: "foo_events", parent: None, "bar",);
    debug_span!(parent: None, "foo", bar.baz = 2, quux = 3);
    debug_span!(parent: None, "foo", bar.baz = 2, quux = 4,);
    debug_span!(parent: None, "foo");
    debug_span!(parent: None, "bar",);
}

#[test]
fn info_span_root() {
    info_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 3);
    info_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 4,);
    info_span!(target: "foo_events", parent: None, "foo");
    info_span!(target: "foo_events", parent: None, "bar",);
    info_span!(parent: None, "foo", bar.baz = 2, quux = 3);
    info_span!(parent: None, "foo", bar.baz = 2, quux = 4,);
    info_span!(parent: None, "foo");
    info_span!(parent: None, "bar",);
}

#[test]
fn warn_span_root() {
    warn_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 3);
    warn_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 4,);
    warn_span!(target: "foo_events", parent: None, "foo");
    warn_span!(target: "foo_events", parent: None, "bar",);
    warn_span!(parent: None, "foo", bar.baz = 2, quux = 3);
    warn_span!(parent: None, "foo", bar.baz = 2, quux = 4,);
    warn_span!(parent: None, "foo");
    warn_span!(parent: None, "bar",);
}

#[test]
fn error_span_root() {
    error_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 3);
    error_span!(target: "foo_events", parent: None, "foo", bar.baz = 2, quux = 4,);
    error_span!(target: "foo_events", parent: None, "foo");
    error_span!(target: "foo_events", parent: None, "bar",);
    error_span!(parent: None, "foo", bar.baz = 2, quux = 3);
    error_span!(parent: None, "foo", bar.baz = 2, quux = 4,);
    error_span!(parent: None, "foo");
    error_span!(parent: None, "bar",);
}

#[test]
fn span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    span!(target: "foo_events", parent: &p, Level::TRACE, "foo", bar.baz = 2, quux = 3);
    span!(target: "foo_events", parent: &p, Level::TRACE, "foo", bar.baz = 2, quux = 4,);
    span!(target: "foo_events", parent: &p, Level::TRACE, "foo");
    span!(target: "foo_events", parent: &p, Level::TRACE, "bar",);
    span!(parent: &p, Level::DEBUG, "foo", bar.baz = 2, quux = 3);
    span!(parent: &p, Level::DEBUG, "foo", bar.baz = 2, quux = 4,);
    span!(parent: &p, Level::DEBUG, "foo");
    span!(parent: &p, Level::DEBUG, "bar",);
}

#[test]
fn trace_span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    trace_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 3);
    trace_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 4,);
    trace_span!(target: "foo_events", parent: &p, "foo");
    trace_span!(target: "foo_events", parent: &p, "bar",);

    trace_span!(parent: &p, "foo", bar.baz = 2, quux = 3);
    trace_span!(parent: &p, "foo", bar.baz = 2, quux = 4,);

    trace_span!(parent: &p, "foo");
    trace_span!(parent: &p, "bar",);
}

#[test]
fn debug_span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    debug_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 3);
    debug_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 4,);
    debug_span!(target: "foo_events", parent: &p, "foo");
    debug_span!(target: "foo_events", parent: &p, "bar",);

    debug_span!(parent: &p, "foo", bar.baz = 2, quux = 3);
    debug_span!(parent: &p, "foo", bar.baz = 2, quux = 4,);

    debug_span!(parent: &p, "foo");
    debug_span!(parent: &p, "bar",);
}

#[test]
fn info_span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    info_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 3);
    info_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 4,);
    info_span!(target: "foo_events", parent: &p, "foo");
    info_span!(target: "foo_events", parent: &p, "bar",);

    info_span!(parent: &p, "foo", bar.baz = 2, quux = 3);
    info_span!(parent: &p, "foo", bar.baz = 2, quux = 4,);

    info_span!(parent: &p, "foo");
    info_span!(parent: &p, "bar",);
}

#[test]
fn warn_span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    warn_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 3);
    warn_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 4,);
    warn_span!(target: "foo_events", parent: &p, "foo");
    warn_span!(target: "foo_events", parent: &p, "bar",);

    warn_span!(parent: &p, "foo", bar.baz = 2, quux = 3);
    warn_span!(parent: &p, "foo", bar.baz = 2, quux = 4,);

    warn_span!(parent: &p, "foo");
    warn_span!(parent: &p, "bar",);
}

#[test]
fn error_span_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    error_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 3);
    error_span!(target: "foo_events", parent: &p, "foo", bar.baz = 2, quux = 4,);
    error_span!(target: "foo_events", parent: &p, "foo");
    error_span!(target: "foo_events", parent: &p, "bar",);

    error_span!(parent: &p, "foo", bar.baz = 2, quux = 3);
    error_span!(parent: &p, "foo", bar.baz = 2, quux = 4,);

    error_span!(parent: &p, "foo");
    error_span!(parent: &p, "bar",);
}

#[test]
fn event() {
    event!(Level::DEBUG, foo = ?3, bar.baz = %2, quux = false);
    event!(Level::DEBUG, foo = 3, bar.baz = 2, quux = false);
    event!(Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(Level::DEBUG, "foo");
    event!(Level::DEBUG, "foo: {}", 3);
    event!(Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(Level::DEBUG, { foo = ?2, bar.baz = %78 }, "quux");
    event!(target: "foo_events", Level::DEBUG, foo = 3, bar.baz = 2, quux = false);
    event!(target: "foo_events", Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(target: "foo_events", Level::DEBUG, "foo");
    event!(target: "foo_events", Level::DEBUG, "foo: {}", 3);
    event!(target: "foo_events", Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(target: "foo_events", Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(target: "foo_events", Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(target: "foo_events", Level::DEBUG, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn trace() {
    trace!(foo = ?3, bar.baz = %2, quux = false);
    trace!(foo = 3, bar.baz = 2, quux = false);
    trace!(foo = 3, bar.baz = 3,);
    trace!("foo");
    trace!("foo: {}", 3);
    trace!({ foo = 3, bar.baz = 80 }, "quux");
    trace!({ foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!({ foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!({ foo = 2, bar.baz = 78 }, "quux");
    trace!({ foo = ?2, bar.baz = %78 }, "quux");
    trace!(target: "foo_events", foo = 3, bar.baz = 2, quux = false);
    trace!(target: "foo_events", foo = 3, bar.baz = 3,);
    trace!(target: "foo_events", "foo");
    trace!(target: "foo_events", "foo: {}", 3);
    trace!(target: "foo_events", { foo = 3, bar.baz = 80 }, "quux");
    trace!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!(target: "foo_events", { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn debug() {
    debug!(foo = ?3, bar.baz = %2, quux = false);
    debug!(foo = 3, bar.baz = 2, quux = false);
    debug!(foo = 3, bar.baz = 3,);
    debug!("foo");
    debug!("foo: {}", 3);
    debug!({ foo = 3, bar.baz = 80 }, "quux");
    debug!({ foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!({ foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!({ foo = 2, bar.baz = 78 }, "quux");
    debug!({ foo = ?2, bar.baz = %78 }, "quux");
    debug!(target: "foo_events", foo = 3, bar.baz = 2, quux = false);
    debug!(target: "foo_events", foo = 3, bar.baz = 3,);
    debug!(target: "foo_events", "foo");
    debug!(target: "foo_events", "foo: {}", 3);
    debug!(target: "foo_events", { foo = 3, bar.baz = 80 }, "quux");
    debug!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!(target: "foo_events", { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn info() {
    info!(foo = ?3, bar.baz = %2, quux = false);
    info!(foo = 3, bar.baz = 2, quux = false);
    info!(foo = 3, bar.baz = 3,);
    info!("foo");
    info!("foo: {}", 3);
    info!({ foo = 3, bar.baz = 80 }, "quux");
    info!({ foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!({ foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!({ foo = 2, bar.baz = 78 }, "quux");
    info!({ foo = ?2, bar.baz = %78 }, "quux");
    info!(target: "foo_events", foo = 3, bar.baz = 2, quux = false);
    info!(target: "foo_events", foo = 3, bar.baz = 3,);
    info!(target: "foo_events", "foo");
    info!(target: "foo_events", "foo: {}", 3);
    info!(target: "foo_events", { foo = 3, bar.baz = 80 }, "quux");
    info!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!(target: "foo_events", { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn warn() {
    warn!(foo = ?3, bar.baz = %2, quux = false);
    warn!(foo = 3, bar.baz = 2, quux = false);
    warn!(foo = 3, bar.baz = 3,);
    warn!("foo");
    warn!("foo: {}", 3);
    warn!({ foo = 3, bar.baz = 80 }, "quux");
    warn!({ foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!({ foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!({ foo = 2, bar.baz = 78 }, "quux");
    warn!({ foo = ?2, bar.baz = %78 }, "quux");
    warn!(target: "foo_events", foo = 3, bar.baz = 2, quux = false);
    warn!(target: "foo_events", foo = 3, bar.baz = 3,);
    warn!(target: "foo_events", "foo");
    warn!(target: "foo_events", "foo: {}", 3);
    warn!(target: "foo_events", { foo = 3, bar.baz = 80 }, "quux");
    warn!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!(target: "foo_events", { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn error() {
    error!(foo = ?3, bar.baz = %2, quux = false);
    error!(foo = 3, bar.baz = 2, quux = false);
    error!(foo = 3, bar.baz = 3,);
    error!("foo");
    error!("foo: {}", 3);
    error!({ foo = 3, bar.baz = 80 }, "quux");
    error!({ foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!({ foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!({ foo = 2, bar.baz = 78, }, "quux");
    error!({ foo = ?2, bar.baz = %78 }, "quux");
    error!(target: "foo_events", foo = 3, bar.baz = 2, quux = false);
    error!(target: "foo_events", foo = 3, bar.baz = 3,);
    error!(target: "foo_events", "foo");
    error!(target: "foo_events", "foo: {}", 3);
    error!(target: "foo_events", { foo = 3, bar.baz = 80 }, "quux");
    error!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!(target: "foo_events", { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!(target: "foo_events", { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn event_root() {
    event!(parent: None, Level::DEBUG, foo = ?3, bar.baz = %2, quux = false);
    event!(
        parent: None,
        Level::DEBUG,
        foo = 3,
        bar.baz = 2,
        quux = false
    );
    event!(parent: None, Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(parent: None, Level::DEBUG, "foo");
    event!(parent: None, Level::DEBUG, "foo: {}", 3);
    event!(parent: None, Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(parent: None, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(parent: None, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(parent: None, Level::DEBUG, { foo = ?2, bar.baz = %78 }, "quux");
    event!(target: "foo_events", parent: None, Level::DEBUG, foo = 3, bar.baz = 2, quux = false);
    event!(target: "foo_events", parent: None, Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(target: "foo_events", parent: None, Level::DEBUG, "foo");
    event!(target: "foo_events", parent: None, Level::DEBUG, "foo: {}", 3);
    event!(target: "foo_events", parent: None, Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(target: "foo_events", parent: None, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(target: "foo_events", parent: None, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(target: "foo_events", parent: None, Level::DEBUG, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn trace_root() {
    trace!(parent: None, foo = ?3, bar.baz = %2, quux = false);
    trace!(parent: None, foo = 3, bar.baz = 2, quux = false);
    trace!(parent: None, foo = 3, bar.baz = 3,);
    trace!(parent: None, "foo");
    trace!(parent: None, "foo: {}", 3);
    trace!(parent: None, { foo = 3, bar.baz = 80 }, "quux");
    trace!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!(parent: None, { foo = 2, bar.baz = 78 }, "quux");
    trace!(parent:None, { foo = ?2, bar.baz = %78 }, "quux");
    trace!(target: "foo_events", parent: None, foo = 3, bar.baz = 2, quux = false);
    trace!(target: "foo_events", parent: None, foo = 3, bar.baz = 3,);
    trace!(target: "foo_events", parent: None, "foo");
    trace!(target: "foo_events", parent: None, "foo: {}", 3);
    trace!(target: "foo_events", parent: None,  { foo = 3, bar.baz = 80 }, "quux");
    trace!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!(target: "foo_events", parent: None,  { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn debug_root() {
    debug!(parent: None, foo = ?3, bar.baz = %2, quux = false);
    debug!(parent: None, foo = 3, bar.baz = 2, quux = false);
    debug!(parent: None, foo = 3, bar.baz = 3,);
    debug!(parent: None, "foo");
    debug!(parent: None, "foo: {}", 3);
    debug!(parent: None, { foo = 3, bar.baz = 80 }, "quux");
    debug!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!(parent: None, { foo = 2, bar.baz = 78 }, "quux");
    debug!(parent: None, { foo = ?2, bar.baz = %78 }, "quux");
    debug!(target: "foo_events", parent: None, foo = 3, bar.baz = 2, quux = false);
    debug!(target: "foo_events", parent: None, foo = 3, bar.baz = 3,);
    debug!(target: "foo_events", parent: None, "foo");
    debug!(target: "foo_events", parent: None, "foo: {}", 3);
    debug!(target: "foo_events", parent: None, { foo = 3, bar.baz = 80 }, "quux");
    debug!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!(target: "foo_events", parent: None, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn info_root() {
    info!(parent: None, foo = ?3, bar.baz = %2, quux = false);
    info!(parent: None, foo = 3, bar.baz = 2, quux = false);
    info!(parent: None, foo = 3, bar.baz = 3,);
    info!(parent: None, "foo");
    info!(parent: None, "foo: {}", 3);
    info!(parent: None, { foo = 3, bar.baz = 80 }, "quux");
    info!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!(parent: None, { foo = 2, bar.baz = 78 }, "quux");
    info!(parent: None, { foo = ?2, bar.baz = %78 }, "quux");
    info!(target: "foo_events", parent: None, foo = 3, bar.baz = 2, quux = false);
    info!(target: "foo_events", parent: None, foo = 3, bar.baz = 3,);
    info!(target: "foo_events", parent: None, "foo");
    info!(target: "foo_events", parent: None, "foo: {}", 3);
    info!(target: "foo_events", parent: None, { foo = 3, bar.baz = 80 }, "quux");
    info!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!(target: "foo_events", parent: None, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn warn_root() {
    warn!(parent: None, foo = ?3, bar.baz = %2, quux = false);
    warn!(parent: None, foo = 3, bar.baz = 2, quux = false);
    warn!(parent: None, foo = 3, bar.baz = 3,);
    warn!(parent: None, "foo");
    warn!(parent: None, "foo: {}", 3);
    warn!(parent: None, { foo = 3, bar.baz = 80 }, "quux");
    warn!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!(parent: None, { foo = 2, bar.baz = 78 }, "quux");
    warn!(parent: None, { foo = ?2, bar.baz = %78 }, "quux");
    warn!(target: "foo_events", parent: None, foo = 3, bar.baz = 2, quux = false);
    warn!(target: "foo_events", parent: None, foo = 3, bar.baz = 3,);
    warn!(target: "foo_events", parent: None, "foo");
    warn!(target: "foo_events", parent: None, "foo: {}", 3);
    warn!(target: "foo_events", parent: None, { foo = 3, bar.baz = 80 }, "quux");
    warn!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!(target: "foo_events", parent: None, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn error_root() {
    error!(parent: None, foo = ?3, bar.baz = %2, quux = false);
    error!(parent: None, foo = 3, bar.baz = 2, quux = false);
    error!(parent: None, foo = 3, bar.baz = 3,);
    error!(parent: None, "foo");
    error!(parent: None, "foo: {}", 3);
    error!(parent: None, { foo = 3, bar.baz = 80 }, "quux");
    error!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!(parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!(parent: None, { foo = 2, bar.baz = 78 }, "quux");
    error!(parent: None, { foo = ?2, bar.baz = %78 }, "quux");
    error!(target: "foo_events", parent: None, foo = 3, bar.baz = 2, quux = false);
    error!(target: "foo_events", parent: None, foo = 3, bar.baz = 3,);
    error!(target: "foo_events", parent: None, "foo");
    error!(target: "foo_events", parent: None, "foo: {}", 3);
    error!(target: "foo_events", parent: None, { foo = 3, bar.baz = 80 }, "quux");
    error!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!(target: "foo_events", parent: None, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!(target: "foo_events", parent: None, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn event_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    event!(parent: &p, Level::DEBUG, foo = ?3, bar.baz = %2, quux = false);
    event!(parent: &p, Level::DEBUG, foo = 3, bar.baz = 2, quux = false);
    event!(parent: &p, Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(parent: &p, Level::DEBUG, "foo");
    event!(parent: &p, Level::DEBUG, "foo: {}", 3);
    event!(parent: &p, Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(parent: &p, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(parent: &p, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(parent: &p, Level::DEBUG, { foo = ?2, bar.baz = %78 }, "quux");
    event!(target: "foo_events", parent: &p, Level::DEBUG, foo = 3, bar.baz = 2, quux = false);
    event!(target: "foo_events", parent: &p, Level::DEBUG, foo = 3, bar.baz = 3,);
    event!(target: "foo_events", parent: &p, Level::DEBUG, "foo");
    event!(target: "foo_events", parent: &p, Level::DEBUG, "foo: {}", 3);
    event!(target: "foo_events", parent: &p, Level::DEBUG, { foo = 3, bar.baz = 80 }, "quux");
    event!(target: "foo_events", parent: &p, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    event!(target: "foo_events", parent: &p, Level::DEBUG, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    event!(target: "foo_events", parent: &p, Level::DEBUG, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn trace_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    trace!(parent: &p, foo = ?3, bar.baz = %2, quux = false);
    trace!(parent: &p, foo = 3, bar.baz = 2, quux = false);
    trace!(parent: &p, foo = 3, bar.baz = 3,);
    trace!(parent: &p, "foo");
    trace!(parent: &p, "foo: {}", 3);
    trace!(parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    trace!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!(parent: &p, { foo = 2, bar.baz = 78 }, "quux");
    trace!(parent: &p, { foo = ?2, bar.baz = %78 }, "quux");
    trace!(target: "foo_events", parent: &p, foo = 3, bar.baz = 2, quux = false);
    trace!(target: "foo_events", parent: &p, foo = 3, bar.baz = 3,);
    trace!(target: "foo_events", parent: &p, "foo");
    trace!(target: "foo_events", parent: &p, "foo: {}", 3);
    trace!(target: "foo_events", parent: &p,  { foo = 3, bar.baz = 80 }, "quux");
    trace!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    trace!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    trace!(target: "foo_events", parent: &p,  { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn debug_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    debug!(parent: &p, foo = ?3, bar.baz = %2, quux = false);
    debug!(parent: &p, foo = 3, bar.baz = 2, quux = false);
    debug!(parent: &p, foo = 3, bar.baz = 3,);
    debug!(parent: &p, "foo");
    debug!(parent: &p, "foo: {}", 3);
    debug!(parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    debug!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!(parent: &p, { foo = 2, bar.baz = 78 }, "quux");
    debug!(parent: &p, { foo = ?2, bar.baz = %78 }, "quux");
    debug!(target: "foo_events", parent: &p, foo = 3, bar.baz = 2, quux = false);
    debug!(target: "foo_events", parent: &p, foo = 3, bar.baz = 3,);
    debug!(target: "foo_events", parent: &p, "foo");
    debug!(target: "foo_events", parent: &p, "foo: {}", 3);
    debug!(target: "foo_events", parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    debug!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    debug!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    debug!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn info_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    info!(parent: &p, foo = ?3, bar.baz = %2, quux = false);
    info!(parent: &p, foo = 3, bar.baz = 2, quux = false);
    info!(parent: &p, foo = 3, bar.baz = 3,);
    info!(parent: &p, "foo");
    info!(parent: &p, "foo: {}", 3);
    info!(parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    info!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!(parent: &p, { foo = 2, bar.baz = 78 }, "quux");
    info!(parent: &p, { foo = ?2, bar.baz = %78 }, "quux");
    info!(target: "foo_events", parent: &p, foo = 3, bar.baz = 2, quux = false);
    info!(target: "foo_events", parent: &p, foo = 3, bar.baz = 3,);
    info!(target: "foo_events", parent: &p, "foo");
    info!(target: "foo_events", parent: &p, "foo: {}", 3);
    info!(target: "foo_events", parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    info!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    info!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    info!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn warn_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    warn!(parent: &p, foo = ?3, bar.baz = %2, quux = false);
    warn!(parent: &p, foo = 3, bar.baz = 2, quux = false);
    warn!(parent: &p, foo = 3, bar.baz = 3,);
    warn!(parent: &p, "foo");
    warn!(parent: &p, "foo: {}", 3);
    warn!(parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    warn!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!(parent: &p, { foo = 2, bar.baz = 78 }, "quux");
    warn!(parent: &p, { foo = ?2, bar.baz = %78 }, "quux");
    warn!(target: "foo_events", parent: &p, foo = 3, bar.baz = 2, quux = false);
    warn!(target: "foo_events", parent: &p, foo = 3, bar.baz = 3,);
    warn!(target: "foo_events", parent: &p, "foo");
    warn!(target: "foo_events", parent: &p, "foo: {}", 3);
    warn!(target: "foo_events", parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    warn!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    warn!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    warn!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn error_with_parent() {
    let p = span!(Level::TRACE, "im_a_parent!");
    error!(parent: &p, foo = ?3, bar.baz = %2, quux = false);
    error!(parent: &p, foo = 3, bar.baz = 2, quux = false);
    error!(parent: &p, foo = 3, bar.baz = 3,);
    error!(parent: &p, "foo");
    error!(parent: &p, "foo: {}", 3);
    error!(parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    error!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!(parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!(parent: &p, { foo = 2, bar.baz = 78 }, "quux");
    error!(parent: &p, { foo = ?2, bar.baz = %78 }, "quux");
    error!(target: "foo_events", parent: &p, foo = 3, bar.baz = 2, quux = false);
    error!(target: "foo_events", parent: &p, foo = 3, bar.baz = 3,);
    error!(target: "foo_events", parent: &p, "foo");
    error!(target: "foo_events", parent: &p, "foo: {}", 3);
    error!(target: "foo_events", parent: &p, { foo = 3, bar.baz = 80 }, "quux");
    error!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}", true);
    error!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 79 }, "quux {:?}, {quux}", true, quux = false);
    error!(target: "foo_events", parent: &p, { foo = 2, bar.baz = 78, }, "quux");
}

#[test]
fn field_shorthand_only() {
    #[derive(Debug)]
    struct Position {
        x: f32,
        y: f32,
    }
    let pos = Position {
        x: 3.234,
        y: -1.223,
    };

    trace!(?pos.x, ?pos.y);
    debug!(?pos.x, ?pos.y);
    info!(?pos.x, ?pos.y);
    warn!(?pos.x, ?pos.y);
    error!(?pos.x, ?pos.y);
    event!(Level::TRACE, ?pos.x, ?pos.y);
}

#[test]
fn callsite_macro_api() {
    // This test should catch any inadvertant breaking changes
    // caused bu changes to the macro.
    let _callsite = callsite! {
        name: "test callsite",
        kind: tracing::metadata::Kind::EVENT,
        target: "test target",
        level: tracing::Level::TRACE,
        fields: foo, bar,
    };
    let _callsite = callsite! {
        name: "test callsite",
        kind: tracing::metadata::Kind::SPAN,
        level: tracing::Level::TRACE,
        fields: foo,
    };
    let _callsite = callsite! {
        name: "test callsite",
        kind: tracing::metadata::Kind::SPAN,
        fields: foo,
    };
}

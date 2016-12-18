extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

use std::env;
use std::process;
use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT};
use std::thread;

use tokio_core::reactor::{Core, Handle};
#[allow(deprecated)]
use tokio_process::{Command, spawn_cmd};

static INIT: Once = ONCE_INIT;

fn init() {
    INIT.call_once(|| {
        let (tx, rx) = channel();
        thread::spawn(move || {
            let mut lp = Core::new().unwrap();
            let cmd = exit();
            let spawn = spawn_cmd(&lp.handle(), cmd);
            let mut child = lp.run(spawn).unwrap();
            drop(child.kill());
            lp.run(child).unwrap();
            tx.send(()).unwrap();
            drop(lp.run(futures::empty::<(), ()>()));
        });
        rx.recv().unwrap();
    });
}

#[allow(deprecated)]
fn exit_deprecated(handle: &Handle) -> Command {
    let mut me = env::current_exe().unwrap();
    me.pop();
    if me.ends_with("deps") {
        me.pop();
    }
    me.push("exit");
    Command::new(me, handle)
}

fn exit() -> process::Command {
    let mut me = env::current_exe().unwrap();
    me.pop();
    if me.ends_with("deps") {
        me.pop();
    }
    me.push("exit");
    process::Command::new(me)
}

#[test]
#[allow(deprecated)]
fn simple_deprecated() {
    init();

    let mut lp = Core::new().unwrap();
    let mut cmd = exit_deprecated(&lp.handle());
    cmd.arg("2");
    let mut child = lp.run(cmd.spawn()).unwrap();
    let id = child.id();
    assert!(id > 0);
    let status = lp.run(&mut child).unwrap();
    assert_eq!(status.code(), Some(2));
    assert_eq!(child.id(), id);
    drop(child.kill());
}

#[test]
fn simple() {
    init();

    let mut lp = Core::new().unwrap();
    let mut cmd = exit();
    cmd.arg("2");
    let spawn = spawn_cmd(&lp.handle(), cmd);
    let mut child = lp.run(spawn).unwrap();
    let id = child.id();
    assert!(id > 0);
    let status = lp.run(&mut child).unwrap();
    assert_eq!(status.code(), Some(2));
    assert_eq!(child.id(), id);
    drop(child.kill());
}

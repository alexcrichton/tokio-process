extern crate tokio_core;
extern crate tokio_process;

use tokio_core::reactor::Core;
use tokio_process::CommandExt;

mod support;

#[test]
fn simple() {
    let mut lp = Core::new().unwrap();
    let mut cmd = support::cmd("exit");
    cmd.arg("2");
    let mut child = cmd.spawn_async_with_handle(lp.handle().new_tokio_handle()).unwrap();
    let id = child.id();
    assert!(id > 0);
    let status = lp.run(&mut child).unwrap();
    assert_eq!(status.code(), Some(2));
    assert_eq!(child.id(), id);
    drop(child.kill());
}

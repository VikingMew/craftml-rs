use pbr::ProgressBar;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

pub fn draw_async_progress_bar(total: u64) -> Sender<u64> {
    let (sender, receiver) = channel();
    spawn(move || {
        let mut pb = ProgressBar::on(::std::io::stderr(), total);
        for count in receiver {
            pb.add(count);
        }
        pb.finish();
    });
    sender
}

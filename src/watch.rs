use crate::Result;
use crate::{evaluate_file, Watch};
use log::info;
use notify::{
    Config, EventKind::Create, PollWatcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind,
};
use std::{path::Path, sync::mpsc, time::Duration};

// example of detecting the recommended watcher kind
pub fn watch(args: Watch) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    // This example is a little bit misleading as you can just create one Config and use it for all watchers.
    // That way the pollwatcher specific stuff is still configured, if it should be used.
    let mut watcher: Box<dyn Watcher> = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        // custom config for PollWatcher kind
        // you
        let config = Config::default().with_poll_interval(Duration::from_secs(1));
        Box::new(PollWatcher::new(tx, config).unwrap())
    } else {
        // use default config for everything else
        Box::new(RecommendedWatcher::new(tx, Config::default()).unwrap())
    };

    // watch some stuff
    args.folder.iter().for_each(|path| {
        watcher
            .watch(
                Path::new(path),
                if args.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                },
            )
            .unwrap();
    });

    info!("Watching for files to render in {:?}…", args.folder);

    while let Ok(result) = rx.recv() {
        if let Ok(event) = result {
            if let Create(_) = event.kind {
                event.paths.iter().for_each(|path| {
                    let file = path.as_path().to_str().unwrap();
                    info!("Rendering '{file}'");
                    render(file, &args);
                })
            }
        } else {
            println!("Error");
        }
    }

    Ok(())
}

fn render(file_name: &str, args: &Watch) {
    let ctx = {
        let mut ctx_args = Vec::with_capacity(2);

        if args.cloud {
            ctx_args.push(nsi::integer!("cloud", true as _));
        } else if let Some(ref collective) = args.collective {
            ctx_args.push(nsi::string!("collective", collective.as_str()));
        }

        nsi::Context::new(Some(&ctx_args)).unwrap()
    };

    evaluate_file(&ctx, file_name, false);

    ctx.render_control(nsi::Action::Wait, None);
}

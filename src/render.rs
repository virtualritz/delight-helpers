use crate::{Render, Result};
use anyhow::anyhow;
use frame_sequence::parse_frame_sequence;
use log::info;

/*fn render(args: Render) -> Result<()> {
    let frame_sequence = if let Some(frame_sequence_string) = &args.frames {
        parse_frame_sequence(frame_sequence_string)
            .map_err(|e| anyhow!("Error in frame sequence expression{e}"))?
    } else {
        vec![]
    };

    let cloud = args.cloud;

    for maybe_glob in &args.file {
        let glob = Glob::from_str(maybe_glob)?;

        for file_name in glob.walk(".") {
            let file_name = file_name.map_err(|e| anyhow!("{e}"))?;
            let file_name = file_name.path().to_str().unwrap();

            let ctx = {
                let mut ctx_args = Vec::with_capacity(2);

                if cloud {
                    ctx_args.push(nsi::integer!("cloud", true as _));
                } else if let Some(ref collective) = args.collective {
                    ctx_args.push(nsi::string!("collective", collective.as_str()));
                }

                nsi::Context::new(Some(&ctx_args)).unwrap()
            };

            if let Some(pos) = file_name.find('@') {
                if frame_sequence.is_empty() {
                    return Err(anyhow!(
                    "[rdl] No frame sequence to fill placeholder `@` in `{file_name}` specified.",
                ));
                }

                let padding = if let Some(number) = file_name.get(pos + 1..pos + 2) {
                    number.parse::<usize>().unwrap_or(0)
                } else {
                    0
                };

                let frame_number_placeholder = if padding != 0 {
                    file_name.get(pos..pos + 2).unwrap()
                } else {
                    "@"
                };

                // Render frame sequence.
                for frame in &frame_sequence {
                    let frame_string = if padding != 0 {
                        format!("{:0width$}", frame, width = padding)
                    } else {
                        format!("{frame}")
                    };

                    let frame_file_name =
                        file_name.replace(frame_number_placeholder, &frame_string);

                    render_file(&ctx, &frame_file_name, &args);

                    ctx.render_control(&[nsi::integer!("frame", *frame as _)]);
                }
            } else {
                render_file(&ctx, file_name, args.dry_run, args.verbose);
            }

            //ctx.render_control(&[nsi::string!("action", "start")]);
            //ctx.render_control(&[nsi::string!("action", "wait")]);
        }
    }
    /*None => Err(eyre!(
        "[rdl] render subcommand requires specifying a file to render"
    )),*/
    Ok(())
}*/

pub fn render(args: Render) -> Result<()> {
    let frame_sequence = if let Some(frame_sequence_string) = &args.frames {
        parse_frame_sequence(frame_sequence_string)
            .map_err(|e| anyhow!("Error in frame sequence expression{e}"))?
    } else {
        vec![]
    };

    for file_name in &args.file {
        let ctx = {
            let mut ctx_args = Vec::with_capacity(2);

            if args.cloud {
                ctx_args.push(nsi::integer!("cloud", true as _));
            } else if let Some(ref collective) = args.collective {
                ctx_args.push(nsi::string!("collective", collective.as_str()));
            }

            nsi::Context::new(Some(&ctx_args)).unwrap()
        };

        if let Some(pos) = file_name.find('@') {
            if frame_sequence.is_empty() {
                return Err(anyhow!(
                    "[rdl] No frame sequence to fill placeholder `@` in `{file_name}` specified.",
                ));
            }

            let padding = if let Some(number) = file_name.get(pos + 1..pos + 2) {
                number.parse::<usize>().unwrap_or(0)
            } else {
                0
            };

            let frame_number_placeholder = if padding != 0 {
                file_name.get(pos..pos + 2).unwrap()
            } else {
                "@"
            };

            // Render frame sequence.
            for frame in &frame_sequence {
                let frame_string = if padding != 0 {
                    format!("{:0width$}", frame, width = padding)
                } else {
                    format!("{frame}")
                };

                let file_name = file_name.replace(frame_number_placeholder, &frame_string);

                render_file(&ctx, &file_name, args.dry_run);
            }
        } else {
            render_file(&ctx, file_name, args.dry_run);
        }

        if args.force_render {
            ctx.render_control(&[nsi::string!("action", "start")]);
            ctx.render_control(&[nsi::string!("action", "wait")]);
        }
    }

    Ok(())
}

pub fn render_file(ctx: &nsi::Context, file_name: &str, dry_run: bool) {
    if dry_run {
        info!("[rdl] Rendering `{}`.", file_name);
    } else {
        ctx.evaluate(&[
            nsi::string!(
                "type",
                if file_name.ends_with(".lua") {
                    "lua"
                } else {
                    "apistream"
                }
            ),
            nsi::string!("filename", file_name),
        ]);
    }
}

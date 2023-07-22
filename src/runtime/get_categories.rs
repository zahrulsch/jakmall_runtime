use std::path::PathBuf;

use tokio::fs;

use crate::{
    prerun::ArgsParsed,
    utils::{recurse_jakmall_cat, JakmallCat},
};

use super::{error_process::ErrorProcess, process::Process, Runtime};

impl Runtime {
    async fn __run_get_categories(args: ArgsParsed) -> Result<PathBuf, ErrorProcess> {
        let cat_path = args.categories_path.join("default").with_extension("txt");
        let cats = Process::get_categories_run()
            .await
            .map_err(ErrorProcess::from_e)?;

        let mut collector = vec![];

        for cat in cats {
            let slug = cat.url.split('?').next().unwrap_or(&cat.url);
            let jc = JakmallCat {
                name: cat.name,
                slug: slug.to_string(),
            };

            collector.push(jc);

            if !cat.children.is_empty() {
                for chil in cat.children {
                    recurse_jakmall_cat(chil, &mut collector)
                }
            }
        }

        let content = collector
            .iter()
            .map(|jc| format!("{},{}", jc.name, jc.slug))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&cat_path, content)
            .await
            .map_err(ErrorProcess::from_e)?;

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        Ok(cat_path)
    }

    pub async fn run_get_categories(&mut self) -> Result<Process, Process> {
        let mut current_process = self.process_in_run.lock().await;

        match current_process.clone() {
            Some(proc) => {
                let err = ErrorProcess::process_in_use(proc);
                Err(err)
            }
            None => {
                let defined_proc = Process::get_categories();

                tokio::spawn({
                    let rt = self.clone();
                    let ws = self.ws_pool.clone();
                    let defined_proc = defined_proc.clone();

                    async move {
                        let res = Runtime::__run_get_categories(rt.args).await;

                        match res {
                            Ok(_) => {
                                let finished = defined_proc.make_done();
                                ws.send_to_all(finished).await;
                            }
                            Err(e) => {
                                let failed = defined_proc.make_error(e);
                                ws.send_to_all(failed).await;
                            }
                        }

                        let mut current_process = rt.process_in_run.lock().await;
                        *current_process = None;
                    }
                });

                *current_process = Some(defined_proc.clone());

                Ok(defined_proc)
            }
        }
    }
}

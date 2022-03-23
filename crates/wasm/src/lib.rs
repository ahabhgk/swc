use std::sync::Arc;

use anyhow::{Context, Error};
use once_cell::sync::Lazy;
use swc::{
    config::{JsMinifyOptions, Options, ParseOptions, SourceMapsConfig},
    try_with_handler, Compiler,
};
use swc_common::{comments::Comments, FileName, FilePathMapping, SourceMap};
use swc_ecmascript::ast::{EsVersion, Program};
use wasm_bindgen::prelude::*;

fn convert_err(err: Error) -> JsValue {
    format!("{:?}", err).into()
}

#[wasm_bindgen(js_name = "minifySync")]
pub fn minify_sync(s: &str, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let c = compiler();

    try_with_handler(
        c.cm.clone(),
        swc::HandlerOpts {
            ..Default::default()
        },
        |handler| {
            c.run(|| {
                let opts: JsMinifyOptions = opts.into_serde().context("failed to parse options")?;

                let fm = c.cm.new_source_file(FileName::Anon, s.into());
                let program = c
                    .minify(fm, handler, &opts)
                    .context("failed to minify file")?;

                JsValue::from_serde(&program).context("failed to serialize json")
            })
        },
    )
    .map_err(convert_err)
}

#[wasm_bindgen(js_name = "parseSync")]
pub fn parse_sync(s: &str, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let c = compiler();

    try_with_handler(
        c.cm.clone(),
        swc::HandlerOpts {
            ..Default::default()
        },
        |handler| {
            c.run(|| {
                let opts: ParseOptions = opts.into_serde().context("failed to parse options")?;

                let fm = c.cm.new_source_file(FileName::Anon, s.into());

                let cmts = c.comments().clone();
                let comments = if opts.comments {
                    Some(&cmts as &dyn Comments)
                } else {
                    None
                };

                let program = c
                    .parse_js(
                        fm,
                        handler,
                        opts.target,
                        opts.syntax,
                        opts.is_module,
                        comments,
                    )
                    .context("failed to parse code")?;

                JsValue::from_serde(&program).context("failed to serialize json")
            })
        },
    )
    .map_err(convert_err)
}

#[wasm_bindgen(js_name = "printSync")]
pub fn print_sync(s: JsValue, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let c = compiler();

    try_with_handler(
        c.cm.clone(),
        swc::HandlerOpts {
            ..Default::default()
        },
        |_handler| {
            c.run(|| {
                let opts: Options = opts.into_serde().context("failed to parse options")?;

                let program: Program = s.into_serde().context("failed to deserialize program")?;

                let s = c
                    .print(
                        &program,
                        None,
                        None,
                        true,
                        opts.codegen_target().unwrap_or(EsVersion::Es2020),
                        opts.source_maps
                            .clone()
                            .unwrap_or(SourceMapsConfig::Bool(false)),
                        &Default::default(),
                        None,
                        opts.config.minify,
                        None,
                    )
                    .context("failed to print code")?;

                JsValue::from_serde(&s).context("failed to serialize json")
            })
        },
    )
    .map_err(convert_err)
}

#[wasm_bindgen(typescript_custom_section)]
const TRANSFORM_SYNC_DEFINITION: &'static str = r#"
/**
* @param {string} code
* @param {any} opts
* @param {Record<string, ArrayBuffer>} experimental_plugin_bytes_resolver An object contains bytes array for the plugin
* specified in config. Key of record represents the name of the plugin specified in config. Note this is an experimental
* interface, likely will change.
* @returns {any}
*/
export function transformSync(code: string, opts: any, experimental_plugin_bytes_resolver?: any): any;
"#;

#[wasm_bindgen(
    js_name = "transformSync",
    typescript_type = "transformSync",
    skip_typescript
)]
#[allow(unused_variables)]
pub fn transform_sync(
    s: &str,
    opts: JsValue,
    experimental_plugin_bytes_resolver: JsValue,
) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let c = compiler();

    #[cfg(feature = "plugin")]
    {
        // TODO: This is probably very inefficient, including each transform
        // deserializes plugin bytes.
        let plugin_bytes = if experimental_plugin_bytes_resolver.is_object() {
            JsValue::into_serde::<std::collections::HashMap<String, Vec<u8>>>(
                &experimental_plugin_bytes_resolver,
            )
            .expect("Object should be available")
        } else {
            Default::default()
        };

        // In here we 'inject' externally loaded bytes into the cache, so remaining
        // plugin_runner execution path works as much as similar between embedded
        // runtime.
        plugin_bytes.into_iter().for_each(|(key, bytes)| {
            swc_plugin_runner::cache::PLUGIN_MODULE_CACHE.store_once(&key, bytes.clone())
        });
    }

    try_with_handler(
        c.cm.clone(),
        swc::HandlerOpts {
            ..Default::default()
        },
        |handler| {
            c.run(|| {
                let opts: Options = opts.into_serde().context("failed to parse options")?;

                let fm = c.cm.new_source_file(
                    if opts.filename.is_empty() {
                        FileName::Anon
                    } else {
                        FileName::Real(opts.filename.clone().into())
                    },
                    s.into(),
                );
                let out = c
                    .process_js_file(fm, handler, &opts)
                    .context("failed to process input file")?;

                JsValue::from_serde(&out).context("failed to serialize json")
            })
        },
    )
    .map_err(convert_err)
}

/// Get global sourcemap
fn compiler() -> Arc<Compiler> {
    static C: Lazy<Arc<Compiler>> = Lazy::new(|| {
        let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

        Arc::new(Compiler::new(cm))
    });

    C.clone()
}

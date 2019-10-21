extern crate neon;
extern crate goblin;

use neon::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use goblin::{Object as Gob, elf, mach::Mach};

fn global_symbols (path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut symbols: Vec<String> = Vec::new();
    let mut buf = Vec::new();

    File::open(path)?.read_to_end(&mut buf)?;

    match Gob::parse(&buf)? {
        Gob::Elf (elf) => {
            for sym in elf.dynsyms.iter() {
                if sym.st_bind() == elf::sym::STB_GLOBAL && sym.st_value != 0 {
                    let name = elf.dynstrtab.get(sym.st_name).unwrap()?.to_string();

                    if !name.is_empty() {
                        symbols.push(name.to_string());
                    }
                }
            }
        }
        Gob::Mach (mach) => {
            match mach {
                Mach::Binary (macho) => {
                    for symbol in macho.symbols() {
                        // what does this do?
                        let (name, nlist) = symbol?;

                        if nlist.is_global() && !nlist.is_undefined() && !name.is_empty() {
                            symbols.push(name.to_string());
                        }
                    }
                }
                Mach::Fat (_) => {
                    panic!("Unsupported format");
                }
            }
        }
        _ => {
            panic!("Unsupported format");
        }
    }

    Ok(symbols)
}

fn js_global_symbols (mut cx: FunctionContext) -> JsResult<JsArray> {
    // TODO: take an (array)buffer instead
    let str = cx.argument::<JsString>(0)?.value();
    let path = Path::new(&str);
    let symbols = global_symbols(&path).or_else(|e| cx.throw_error(e.to_string()))?;
    let js_arr = JsArray::new(&mut cx, symbols.len() as u32);

    for (i, item) in symbols.iter().enumerate() {
        let js_str = cx.string(item);
        let _ = js_arr.set(&mut cx, i as u32, js_str);
    }

    Ok(js_arr)
}

register_module!(mut cx, {
    cx.export_function("syms", js_global_symbols)
});

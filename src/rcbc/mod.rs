extern crate getopts;

use self::getopts::{Options, Matches};
use std::env;
use std::process;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

mod compiler;
mod assembler;
mod linker;


const EXT_CFLAT_SOURCE:    &'static str = "cb";
const EXT_ASSEMBLY_SOURCE: &'static str = "s" ;
const EXT_OBJECT_FILE:     &'static str = "o" ;
// const EXT_STATIC_LIBRARY:  &'static str = "a" ;
// const EXT_SHARED_LIBRARY:  &'static str = "so";
// const EXT_EXECUTABLE_FILE: &'static str = ""  ;

const EXECUTABLE_FILE_DEFAULT: &'static str = "a.out";


/// The entry for the rcbc to be a compiler in command line.
///
/// It will not panic, but only get some options and flags and then handle some
/// source files, generate an executable file (ELF) at last.
///
pub fn cli() {
    let argv: Vec<String> = env::args().collect();
    let program: &str = &argv[0][..];

    let mut opts = Options::new();
    
    opts.optflag("h", "help", "Display this information");
    opts.optflag("", "version", "Display compiler version information");

    opts.optflag("", "dump-tokens", "Show the token stream by lexer.");
    
    let matches = match opts.parse(&argv[1..]) {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            print_usage(program, opts);
            return;
        },
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
    } else if matches.free.len() > 0 {
        cli_main(matches);
    } else {
        shutdown_for("no input files");
    }
}

fn print_usage(program: &str, opts: Options) {
    print!("{}", opts.usage(
        &format!("Usage: {} [options] <files>...", program)
    ));
}

/// throw fatal error and shutdown the program.
fn shutdown_for(reason: &str) -> ! {
    println!("rcbc: fatal error: {}", reason);
    process::exit(1)
}

/// The main process function including compilation, assembly and link.
fn cli_main(matches: Matches) {
    let src_files: Vec<PathBuf> = matches.free.iter().map(PathBuf::from).collect();
    let exec_file = PathBuf::from(EXECUTABLE_FILE_DEFAULT);
    
    let compiler = compiler::Compiler::new();
    let assembler = assembler::Assembler::new();
    let linker = linker::Linker::new();

    let compiler_opts = compiler::CompileOptionBuilder::new()
        .is_dump_tokens(matches.opt_present("dump-tokens")).finalize();

    for src_file in src_files.iter() {
        if !src_file.exists() {
            shutdown_for(&format!("`{}`: No such file or directory", 
                src_file.to_str().unwrap()));
        }
        if !is_source_file(src_file) {
            shutdown_for(&format!("`{}`: Not valid C-flat source file (*.cb)", 
                src_file.to_str().unwrap()));
        }
    }

    let mut obj_files = Vec::new();

    for src in src_files.iter() {
        let src_file = src.clone();
        let asm_file = asm_file_name_of(src);
        let obj_file = obj_file_name_of(src);

        let compile_result = compiler.compile(&src_file, &asm_file, compiler_opts);

        if let Err(err) = compile_result {
            println!("Compiler Error: {}", err);
            return;
        }

        let assemble_result = assembler.assemble(&asm_file, &obj_file);

        if let Err(err) = assemble_result {
            println!("Assembler Error: {}", err);
            return;
        }

        obj_files.push(obj_file);
    }

    let link_result = linker.link(&obj_files, &exec_file);

    if let Err(err) = link_result {
        println!("Assembler Error: {}", err);
        return;
    }
}

/// judge if the file in path `file` is a valid source file or not.
fn is_source_file(file: &Path) -> bool {
    let src_ext: &OsStr = OsStr::new(EXT_CFLAT_SOURCE);
    match file.extension() {
        Some(ext) if src_ext == ext => true,
        _                           => false,
    }
}

/// create the corresponding assembly file name.
/// make sure the file should be the cflat source file
fn asm_file_name_of(file: &Path) -> PathBuf {
    file.with_extension(EXT_ASSEMBLY_SOURCE)
}

/// create the corresponding object file name.
/// make sure the file should be the cflat source file
fn obj_file_name_of(file: &Path) -> PathBuf {
    file.with_extension(EXT_OBJECT_FILE)
}
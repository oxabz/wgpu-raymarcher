use std::borrow::Borrow;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::os::unix;
use std::path::{ Path, PathBuf};
use std::process::exit;
use thiserror::{Error};
use serde_json::{from_reader, StreamDeserializer, Value};
use serde::{Deserialize};
use crate::BuildError::IOError;

const SHADERS_PATH: &'static str = "./src/shaders";

fn write_section_title(file: &mut BufWriter<File>, title:&String) -> std::io::Result<()> {
    return write!(file, "\n\
        \n\
        /////////////////////////////////////////////\n\
        // {} \n\
        /////////////////////////////////////////////\n\
        \n\
        ",title)
}

#[derive(Clone, Debug, Deserialize)]
struct Template{
    target: String,
    components:Vec<Component>
}

#[derive(Clone, Debug, Deserialize)]
struct Component{
    title: String,
    path: String
}

#[derive(Debug, Error)]
enum BuildError {
    #[error("Fail to open a file : {0} \n {1}")]
    IOError(String, std::io::Error),
    #[error("Error parsing the template \"{0}\": \n {1}")]
    ParseError(PathBuf, serde_json::Error),
}

fn build_shader(path:&Path)-> Result<(),BuildError>{
    let template_file = match File::open(path) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(BuildError::IOError("".into(),err))
        }
    };
    let buf_read = BufReader::new(template_file);
    let template: Template = match from_reader(buf_read) {
        Ok(ok) =>ok,
        Err(err) => {
            return Err(BuildError::ParseError(path.to_path_buf(), err));
        }
    };

    let target_path = Path::new(&template.target);
    let mut target = match File::create(target_path) {
        Ok(ok) => {ok}
        Err(err) => {return Err(BuildError::IOError(template.target,err))}
    };
    let mut target = BufWriter::new(target);
    for component in template.components {
        let path = component.path;
        if let Err(err) = write_section_title(&mut target, &component.title) {
            return Err(IOError(path.into(),err));
        }
        let component_file = match File::open(path.clone()) {
            Ok(ok) => {ok}
            Err(err) => {
                return Err(BuildError::IOError(path,err))
            }
        };
        let mut reader = BufReader::new(component_file);
        if let Err(err)= io::copy(&mut reader, &mut target){
            return Err(IOError("".into(),err));
        }
    }
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=src/shaders/");

    let shader_path = Path::new(SHADERS_PATH);
    if !shader_path.is_dir() || !shader_path.exists(){
        eprintln!("Couldn't find the shader folder at {}", SHADERS_PATH);
        exit(1);
    }

    let mut failed = false;
    for entry in  fs::read_dir(shader_path).unwrap(){
        let entry = match entry {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("{}: {}",SHADERS_PATH, err);
                failed = true;
                continue;
            }
        }.path();


        if !entry.is_file() || !matches!( entry.extension(), Some(ext) if ext=="json"){
            continue;
        }

        if let Err(err) = build_shader(entry.borrow()) {
            eprintln!("{}", err);
            failed = true;
        }
    }
    if failed {
        exit(1);
    }
}
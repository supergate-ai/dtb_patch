mod dts_parser;
mod extlinux_parser;

use regex::Regex;

use std::process::Command;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use dts_parser::node::DtbNode;
use extlinux_parser::Extlinux;

fn main() {
    // check module type
    print!("Check Jetson module... ");
    let mut model = OpenOptions::new().read(true).open("/proc/device-tree/model").expect("Error : Cannot identify Jetson module type");
    let mut model_string = String::new();
    model.read_to_string(&mut model_string).expect("Error : Cannot read from /proc/device-tree/model");
    
    match model_string.find("Orin") {
        Some(_) => {
            println!("{model_string}");
        },
        None => {
            if model_string.find("Xavier").is_some() {
                println!("NVIDIA Jetson Xavier NX");
                println!("Jetson Xavier NX module does not require device-tree patch.");
            } else {
                println!("Unknown Device");
                println!("Cannot identify Jetson module type. Setup aborted.");
            }
            return;
        }
    }

    // check l4t type
    print!("Check L4T version... ");
    let mut nv_tegra_release = OpenOptions::new().read(true).open("/etc/nv_tegra_release").expect("Error : Cannot identify Jetson Linux version");
    let mut release_string = String::new();
    nv_tegra_release.read_to_string(&mut release_string).expect("Error : Cannot read from /etc/nv_tegra_release");

    let re = Regex::new(r"^# R(?P<main>\d*) (release), REVISION: (?P<rev>[^,]*),").unwrap();
    match re.captures(&release_string) {
        Some(caps) => {
            println!("Jetson Linux R{}.{}", &caps["main"], &caps["rev"]);
        },
        None => {
            println!("Error : Cannot parse Jetson Linux version from /etc/nv_tegra_release");
            return;
        }
    }
    
    // read extlinux and parse fdt path from default entry
    print!("Read boot entries... ");
    let extlinux = Extlinux::load("/boot/extlinux/extlinux.conf");

    let target_dtb = extlinux.default_entry().fdt.clone().expect("Default entry has no FDT data");
    let target_dts = target_dtb.as_str().strip_suffix(".dtb").unwrap().to_string() + ".dts";

    println!("OK");

    // decompile
    print!("Decompiling device tree blob file... ");
    let decompile = Command::new("dtc").args([
        "-I", "dtb",
        "-O", "dts",
        &target_dtb, "-o", &target_dts
    ]).output().expect("Error : Cannot decompile dtb file");

    match String::from_utf8(decompile.stderr).unwrap().find("No such file or directory") {
        Some(_) => {
            println!("");
            panic!("Error : Cannot decompile dtb file... No such file or directory");
        },
        _ => {
            println!("OK");
        }
    }

    // open decompiled dts file
    print!("Opening decompiled dts file... ");
    let mut dts = OpenOptions::new().read(true).create_new(false).open(&target_dts).expect("Error : Cannot open decompiled dts file");

    println!("OK");

    // initialize 
    print!("Parsing device tree from opened dts file... ");
    let mut buffer = String::new();
    dts.read_to_string(&mut buffer).expect("Error : Cannot read from dts file");

    let mut root = DtbNode::load(buffer);

    println!("OK");

    // patch csi camera node
    let cam_i2c0 = root.find_childnode("cam_i2cmux").unwrap()
                        .find_childnode("i2c@0").unwrap();

    let rbpcv3_imx477_a_1a = cam_i2c0.find_childnode("rbpcv3_imx477_a@1a").unwrap();

    rbpcv3_imx477_a_1a.find_property("status").unwrap()
                    .value = Some("\"okay\"".to_string());

    rbpcv3_imx477_a_1a.find_childnode("mode0").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv3_imx477_a_1a.find_childnode("mode1").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv3_imx477_a_1a.find_childnode("ports").unwrap()
                    .find_childnode("port@0").unwrap()
                    .find_childnode("endpoint").unwrap()
                    .find_property("port-index").unwrap()
                    .value = Some("<0x00>".to_string());

    let rbpcv2_imx219_a_10 = cam_i2c0.find_childnode("rbpcv2_imx219_a@10").unwrap();

    rbpcv2_imx219_a_10.find_childnode("mode0").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv2_imx219_a_10.find_childnode("mode1").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv2_imx219_a_10.find_childnode("mode2").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv2_imx219_a_10.find_childnode("mode3").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv2_imx219_a_10.find_childnode("mode4").unwrap()
                    .find_property("tegra_sinterface").unwrap()
                    .value = Some("\"serial_a\"".to_string());

    rbpcv2_imx219_a_10.find_childnode("ports").unwrap()
                    .find_childnode("port@0").unwrap()
                    .find_childnode("endpoint").unwrap()
                    .find_property("port-index").unwrap()
                    .value = Some("<0x00>".to_string());

    let rbpcv3_imx477_c_1a = root.find_childnode("cam_i2cmux").unwrap()
                    .find_childnode("i2c@1").unwrap()
                    .find_childnode("rbpcv3_imx477_c@1a").unwrap();
    
    rbpcv3_imx477_c_1a.find_property("status").unwrap()
                    .value = Some("\"okay\"".to_string());

    // apply root to new dts file
    let patched = root.stringify(0);
    let mut patched_dts = OpenOptions::new().write(true).truncate(true).open(&target_dts).expect("Error : Cannot open dts file writeonly");
    patched_dts.write_all(patched.as_bytes()).expect("Error : Cannot write to new dts file");

    // compile
    print!("Compile patched dts file... ");
    let _compile = Command::new("dtc").args([
        "-I", "dts",
        "-O", "dtb",
        &target_dts, "-o", &target_dtb
    ]).output().expect("Error : Failed to compile patched dts");

    println!("OK");

    println!("Patch finished successfully. Please reboot the device.");
}
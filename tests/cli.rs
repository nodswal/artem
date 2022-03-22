use std::fs;

fn load_correct_file() -> String {
    let desired_output = fs::read_to_string("assets/abraham_lincoln.txt").unwrap(); //ignore errors
    desired_output
}

pub mod input {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn input_does_not_exist() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("test/non-existing/file");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn input_is_dir() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("test/");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("does not exist"));
    }

    #[test]
    fn correct_input() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg");
        //check only the first line, the rest is likely to be correct as well
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod density {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("-c");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--characters <density>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_number() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-c 0.6");
        cmd.assert().success().stdout(predicate::str::starts_with(
            "                       0000  000000000000000000000000000000000000000            ",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-c", "M0123-."]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "MMMMMMMMM00000000000000000000000000000101111111111111111111110011000000000000000",
        ));
    }

    #[test]
    fn arg_preset_0_short_s() {
        for arg in ["short", "s", "0"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                "####WWWWW$$$$$999$9888777777777666667666555555655555543334556665667778888899$$$9",
            ));
        }
    }

    #[test]
    fn arg_preset_1_flat_f() {
        for arg in ["flat", "f", "1"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert()
                .success()
                .stdout(predicate::str::starts_with(load_correct_file()));
        }
    }

    #[test]
    fn arg_preset_2_long_l() {
        for arg in ["long", "l", "2"] {
            let mut cmd = Command::cargo_bin("artem").unwrap();
            cmd.arg("examples/abraham_lincoln.jpg").args(["-c", arg]);
            //only check first line
            cmd.assert().success().stdout(predicate::str::starts_with(
                "W&&WMMM#*oaaaahhaahbbdqqwwwppwwmwmmmwwZmO0O0Q0Z000000CUJCC0OZmm0Zmqqqdbbbbkkaoah",
            ));
        }
    }
}

pub mod size {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("-s");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s string");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s 0.6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg").arg("-s -6");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg(format!("-s {}", u32::MAX));
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with size input value",
        ));
    }

    #[test]
    fn arg_conflict_width() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-s", "75"])
            .arg("-w");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--width'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-s", "75"])
            .arg("-h");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--size <size>' cannot be used with '--height'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").args(["-s", "75"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNXXXXXXXXXKKK000O00000OOOOO0OOOkkkkkOkkkkkkxxxxkkOOOkOO000KKKKKKXXXX",
        ));
    }
}

pub mod width {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").args(["-w", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg("-w")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg").arg("-w").arg("-h");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--width' cannot be used with '--height'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--width");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod height {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").args(["-h", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_size() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .arg("-h")
            .args(["-s", "75"]);
        //should panic when trying using both args
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--size <size>'",
        ));
    }

    #[test]
    fn arg_conflict_height() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying using both args
        cmd.arg("examples/abraham_lincoln.jpg").arg("-h").arg("-w");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--height' cannot be used with '--width'",
        ));
    }

    #[test]
    #[should_panic]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--height");
        //should panic in the test case, since the terminal size is 0
        cmd.assert().success().stdout(predicate::str::starts_with(
            "WWWNNNNNNXXXXXXKXXXKK0000OO000OOOOOOOOOOOkkkkkkOkkkkkkxxxxxkkOOOkOO0000KKKKKKKXX",
        ));
    }
}

pub mod scale {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("--ratio");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--ratio <scale>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "string"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with ratio input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "-6"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '-6' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", f64::MAX.to_string().as_str()]);
        cmd.assert().success().stdout(predicate::str::starts_with(
            "NWWNNNNNXXXXKKKKXXKK00000O000OOOOOOkOOkkxxkxxxOkkkkkkxxdddkkOOOkOO000KKK00KKXXXX",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--ratio", "0.75"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "NWWNNNNNXXXXKKKKXXKKK0000OO00OOOOOOOOOOOxkkkxkOkkkkkkxxdxxkkOOOkOO000KKK0KKKXXXX",
        ));
    }
}

pub mod flip_x {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipX", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--flipX");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "XXXXKKKKKK0000OOkOOOOkxxxxxkkkkkkOkkkOkkOOO0OOOOO0000OOO000KKKXXKXXXXXXNNNNNNWWW",
        ));
    }
}

pub mod flip_y {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipY", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--flipY");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            ".....................................................                           ",
        ));
    }
}

pub mod flip_x_y {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--flipY", "--flipX"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "                           .....................................................",
        ));
    }
}

pub mod thread {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();

        cmd.arg("examples/abraham_lincoln.jpg").arg("--thread");
        cmd.assert().failure().stderr(predicate::str::contains(
            "The argument '--thread <threads>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_is_nan() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "string"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with thread input value",
        ));
    }

    #[test]
    fn arg_is_float() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "0.6"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "Could not work with thread input value",
        ));
    }

    #[test]
    fn arg_is_negative() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "-6"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: Found argument '-6' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_larger_max() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        //should panic when trying to convert the arg
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", u32::MAX.to_string().as_str()]);
        //since its clamped, it should return the normal img
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--thread", "3"]);
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod output_file {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::{fs, process::Command};

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("-o");
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--output <output-file>' requires a value but none was supplied",
        ));
    }

    #[test]
    //windows does not like this test, it can not create the file
    #[cfg(not(target_os = "windows"))]
    fn arg_is_correct() -> Result<(), std::io::Error> {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["-o", "/tmp/test.txt"]);
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "Written 3617 bytes to /tmp/test.txt",
        ));
        //delete output file
        fs::remove_file("/tmp/test.txt")
    }
}

pub mod invert {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--invert", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--invert");
        //only check first line
        cmd.assert().success().stdout(predicate::str::starts_with(
            "         ...............'''....'''''.''',,',,,',,,,,,;;;;;,'''',''..............",
        ));
    }
}

pub mod background {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--background", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_no_color() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--background", "--no-color"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--background' cannot be used with '--no-color'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--background");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod no_color {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    use crate::load_correct_file;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--no-color", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_conflict_background() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--no-color", "--background"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--no-color' cannot be used with '--background'",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--no-color");
        //only check first line
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(load_correct_file()));
    }
}

pub mod border {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_with_value() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--border", "123"]);
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: Found argument '123' which wasn't expected, or isn't valid in this context",
        ));
    }

    #[test]
    fn arg_is_correct() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--border");
        //only check first line
        cmd.assert()
            .success().stdout(predicate::str::starts_with(
                "╔══════════════════════════════════════════════════════════════════════════════╗",
            ))
            .success().stdout(predicate::str::ends_with(
                "╚══════════════════════════════════════════════════════════════════════════════╝\n",
            ));
    }
}

pub mod verbosity {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn arg_is_none() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg").arg("--verbose");
        cmd.assert().failure().stderr(predicate::str::starts_with(
            "error: The argument '--verbose <verbosity>' requires a value but none was supplied",
        ));
    }

    #[test]
    fn arg_info() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "info"]);
        //only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("INFO"));
    }

    #[test]
    fn arg_debug() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.jpg")
            .args(["--verbose", "debug"]);
        //only check first line
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("DEBUG"));
    }

    #[test]
    fn arg_error() {
        let mut cmd = Command::cargo_bin("artem").unwrap();
        cmd.arg("examples/abraham_lincoln.nonexisting") //this causes a fatal error
            .args(["--verbose", "error"]);
        //only check first line
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("ERROR"));
    }
}

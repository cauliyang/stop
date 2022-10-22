pub mod cmds {

    use std::process::Command;
    use assert_cmd::output;

    pub fn squeue(args: Option<&[&str]>) {

        let mut output = Command::new("squeue");

        if let Some(arguments) = args {
            let output = output.args(arguments);
        }

        let output =  output.output().expect("squeue failed to execute");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }


    pub fn sinfo(args: Option<&[&str]>){
        let mut output = Command::new("sinfo");

        if let Some(arguments) = args{
            let output = output.args(arguments);
        }

        let output = output.output().expect("sinfo failed to execuete");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
}

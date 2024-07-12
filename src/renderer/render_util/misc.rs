pub fn runtimeFmt(IN_str: &str, IN_args: &Vec<(&'static str, String)>) -> String{
    let mut OUT_fmtStr: String = IN_str.to_string();

    for ARG in IN_args{
        OUT_fmtStr = OUT_fmtStr.replace(ARG.0, &ARG.1);
    }

    return OUT_fmtStr;
}

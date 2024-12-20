pub struct ParamF<'a> {
    pub request: &'a str,
    pub reply: &'a str,
}

pub fn myf(args: ParamF) {
    println!("{}", args.reply);
    println!("{}", args.request);
}

use warn_example::foo;


foo!();  // will generate a warning

foo!(mod bar {}); // will not generate a warning


fn main() {}

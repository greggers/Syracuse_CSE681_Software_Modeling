sig Requirement {
  id: one String,
  text: one String
}

sig Block {
  name: one String,
  message: one String
}

sig Satisfies {
  client: one Block,
  supplier: one Requirement
}

pred showExample {
  some r: Requirement, b: Block, s: Satisfies |
    r.text = "Program shall output to command line 'Hello World'" and
    b.message = "Hello World" and
    s.client = b and
    s.supplier = r
}

run showExample


* lang: system, very fast, like modern c/functional/safe: start from c/python but like in different direction
* lang: type: dependent/refinement/unique type/proof assistance/similar stuff, mostly hard focused on types, theory
* lang: fast, effects, easier to bootstrap(lispy?) refinement, usage?, simple, gc, focus on types, but also on backend: start backend with some of c/asm/llvm and also try eventually interpreter

```c
function a(int* b, int* c) -> int {
	*b + *c
}
```

```rust
fn a(node: FlightToEurope) -> Time {
	node.time: proof
}
```

```scheme
@pure
(function parse (text: Text) Node?
  code)

@read(path)
(function resolve (path: Path) Node?
  node)

@pure
(function check (node: Node, env: Env) TypedNode?
  node .. )

@write
(function generateAsm (node: TypedNode, env: GenEnv) Empty?
  code)
```

types: 
  numbers
  units
  booleans
  text
  adt:
    simple
  records
  error
  results (?: with a big error enum-like type)
  lists
  maps

  no generics for now?

effects:
  simple: annotations for now

syntax:
  lispy?

  matching:
    simple, e.g. (match node 
    	            (NodeInt ->i) (print i)
    	          )


    	          for now no match-based overloading
    	          (function check (node: (NodeInt -> i))..)
                     i !

                   maybe type-based overloading
                   effect-based ? 
                   we need maps and filters tho

numbers
units
booleans
text
records
error
results
lists

effects:
  simple

syntax:
  lispy?

backend:
  todo: maybe c

gc:
  maybe placeholder: just leaking for now

very minimal

numbers
booleans
text

effects:
  simple

stdlib:
  print

syntax:
  lispy

backend:
  todo: maybe c or asm


  
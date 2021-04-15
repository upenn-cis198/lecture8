# hw3 Common Mistakes

Since we didn't get to it in lecture, here are some of the common mistakes that I noticed on hw3 so far.

## Trait Bounds

Trait bounds should be local to only the code that actually needs the bound. For example on the homework, `Serialize` and `Deserialize` should only be for the load/save code, not for the entire struct. You will also find this leads to cleaner, less cluttered code.

## Iterators

Don't create the things up front and then iterate, that defeats the purpose of having an iterator!
For example to implement `iter_edges` if you first create a `vec: Vec<&E>` with the results, and then call `vec.into_iter()` to return the results, then it would be better to just return the vector itself. The point of having a function like `iter_edges` is to lazily step over the edges (without generating them all up front)

## Working with Options

Lots of very useful functions in the Option API:
[here](https://doc.rust-lang.org/std/option/enum.Option.html)

In particular: `.expect()`, `.map()`, `.unwrap_or_else()`, `.map_or()`, `.cloned()`, `.is_some()`, `.is_none()`

You usually shouldn't have to manually `match opt` for an option, there is probably a method in the API that does what you want!

## Auxiliary Methods

If you have a lot of long nested method calls in your code like
```
self.get_id(self.my_map.get(x).unwrap().get(y)).unwrap()
```
then probably you need an auxiliary method!
Similarly, if you have overly long functions, with sequences of 3-4 repeated command blocks, probably those command blocks should be turned into auxiliary methods.

Also: think about which auxiliary methods should be private. If it is only for internal usage (e.g. manipulating vertex / edge identifiers), it should be ideally private, so that your only publicly exposed API is the add/remove and functions based on `V` and `E` types.

## Derived Methods in Traits

Traits allow a "default implementation" based on the provided functionality.
You can read about it [here](https://doc.rust-lang.org/book/ch10-02-traits.html).

For the "Reachable" trait, you can use this very profitably. What you can do is have only one or two function that the implementer has to implement, say `fn get_distance(&self, T, T)`. Then all the other methods can be derived functions provided with the trait, rather than provided by the type implementing that trait.

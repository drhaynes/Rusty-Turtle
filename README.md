# Rusty-Turtle
A minimal Logo programming environment written in Rust, using egui for UI.

### Compatibility

This version aims to be compatible with UCBLogo, treating it as the de-facto standard in lieu of an actual Logo standard.

---

### Current thoughts on unicode support:

* Treat the input source code string as UTF-8 (i.e. the same as rust's `String` and `&str` types).
* Iterate through codepoints when lexing. `Chars::next()`. Presumably this is how `rustc` does it - check.
* Match as we need against the UTF-8 values for our tokens.
* Any identifiers will be composed of UTF-8 encoded codepoints
* No need to worry about the complexity of grapheme clusters.
* Unicode literal encoding is out-of-scope.

---


### Parser and Grammar

The parser is a single-character look-ahead recursive-descent parser (vs something using a parser-generator). It uses [Lyn](https://github.com/rapodaca/lyn) for convenience of implementation.

Grammar rules are taken from the UCBLogo implementation source code, as their exists no official formal grammar for Logo. Parser code [can be found here (Github repo)](https://github.com/jrincayc/ucblogo-code/blob/master/parse.c).

See: "Tokenization" section of the [UCBLogo User Manual](https://people.eecs.berkeley.edu/~bh/usermanual).

Worth noting:
> Terrapin-style tokenization (e.g., [2+3] is a list with one member)
but LCSI-style syntax (no special forms except TO).  The best of
both worlds. - From the 'special features' introduction section of the UCBLogo User Manual.

Another nice reference can be found here: [https://www.calormen.com/jslogo/language.html](https://www.calormen.com/jslogo/language.html)

## TODO
* Draw turtle on canvas:
	* For how to achieve this, see: [https://github.com/emilk/egui/blob/master/egui_demo_lib/src/apps/fractal_clock.rs](https://github.com/emilk/egui/blob/master/egui_demo_lib/src/apps/fractal_clock.rs) (specifically use of `Painter` and `Stroke` and the `draw()` function).
	* egui's epaint API documented here: [https://docs.rs/epaint/0.14.0/epaint/](https://docs.rs/epaint/0.14.0/epaint/)
* Read input from buffer
* Keywords:
	* `fd`
	* `rt` `lt`
	* `home`
	* `cs`
	* `repeat n [ ... ]`
	* `to foo ... end`
	* `save str`
	* `load str`
	* `make`
	* `test`
	* `iftrue` `iffalse`
	* `print`
* Terminal / REPL:
    * I think this can be achieved mostly with: [https://github.com/kkawakam/rustyline](https://github.com/kkawakam/rustyline)
    * But maybe a better option would be Herbert Wolverson's crates (Hands-on Rust author)
    * References:
        * Jonathan Blow, drop down console implementation: [https://www.youtube.com/watch?v=jjTzVMq_M3M](https://www.youtube.com/watch?v=jjTzVMq_M3M)
        * [https://github.com/alacritty/alacritty](https://github.com/alacritty/alacritty)
    * Keep scrollback in terminal
    * `>` Prompt
    * Command history with up/down arrows
    * TAB completion

* Fullscreen mode (default)
* Procedure parameters/arguments, i.e. `to foo :bar`
* Arithmetic operations
* Lists
* GC... (perhaps use an off-the-shelf crate for this)
* Line colours
* Screenshots
* Printer support
* Tail recursion optimisation (see below references for implementation details)
* String interning (see below for rustc implementation details and discussion)

## Useful references:

#### Berkeley Logo (UCBLogo)

User Manual:
[https://people.eecs.berkeley.edu/~bh/usermanual](https://people.eecs.berkeley.edu/~bh/usermanual)

Source code:
[https://github.com/jrincayc/ucblogo-code](https://github.com/jrincayc/ucblogo-code)

Terminal implementation details:
[https://github.com/jrincayc/ucblogo-code/blob/master/newtermnotes](https://github.com/jrincayc/ucblogo-code/blob/master/newtermnotes)

Understanding the UCBLogo evaluator:
[https://github.com/jrincayc/ucblogo-code/blob/master/plm](https://github.com/jrincayc/ucblogo-code/blob/master/plm)

Quotes:

>I'm not trying to argue that dynamic scope is perfect, either.
I'm arguing that the unquestioned superiority of lexical scope for serious
programming has a high cost in required expertise, and that for beginners
the cost may outweigh the benefit.  Dynamic scope is, imho, part of the
"no threshold" promise of Logo. - Brian Harvey

> Serious versions of Logo (which included LCSI versions prior to
LogoWriter) have a pause-on-error feature that's a real help in
debugging.  In case of an error, you get a Logo prompt in the
environment in which the error occurred.  This lets you use Logo
itself, rather than some special debugging language, to investigate
the state that gave rise to the error -- specifically, the value of
variables.  But it's quite common that if procedure A calls B which
calls C which calls D, an error in A can give faulty inputs to B,
which aren't caught until one of them becomes an input to a
primitive down in D.  In dynamically scoped Logo, at the moment of
the error, all of the local variables of A, B, C, and D are
naturally available in the error environment.  In a lexically scoped
language, only D's local variables -- which won't help you find the
problem -- are available in the error environment.  This is why all
the Scheme de{*filter*}s I've seen have special instructions, not part
of Scheme itself, that say "switch to the dynamically previous
scope" and things like that.  Well, that's okay once you understand
what an environment *is*, which comes about half way through my
course for undergraduate CS majors.  It's no good for beginners. - Brian Harvey

Both from: [http://computer-programming-forum.com/25-logo/d60543e3208386a2.htm](http://computer-programming-forum.com/25-logo/d60543e3208386a2.htm)

---

### QLogo

_QLogo is a Logo interpreter developed using C++ and Qt. New features include hardware-accelerated graphics and Unicode text._

[https://github.com/jasonsikes/QLogo](https://github.com/jasonsikes/QLogo)

---

### c2 Logo Language page

> Logo is notable as one of the very few languages that ever had both DynamicScoping and TailCallOptimization at the same time. Over the years, more than one person has believed this combination to be impossible in the general case (see, for example, pages 12 and 13 of Lambda: The Ultimate Declarative

[http://wiki.c2.com/?LogoLanguage](http://wiki.c2.com/?LogoLanguage)


---

### Tail Recursion with Dynamic Scope by Darius Bacon

>This revision of a comp.lang.scheme article assumes you're fluent in Lisp and have a basic acquaintance with tail recursion and dynamic scoping of variables.

[http://web.archive.org/web/20090102070914/http://www.accesscom.com/~darius/writings/dynatail.html](http://web.archive.org/web/20090102070914/http://www.accesscom.com/~darius/writings/dynatail.html)

---

### IBM Logo Manual

_The manual included with a copy of Logo for IBM PCs from circa. 1983._

[https://archive.org/details/ibm_logo_manual](https://archive.org/details/ibm_logo_manual)

---

### History of LOGO (C. Solomon et al.)
[https://people.eecs.berkeley.edu/~bh/HOPL.pdf](https://people.eecs.berkeley.edu/~bh/HOPL.pdf)

---


### Computer Science Logo Style

Volume 1: Symbolic Computing
[https://people.eecs.berkeley.edu/~bh/v1-toc2.html](https://people.eecs.berkeley.edu/~bh/v1-toc2.html)

Volume 2: Advanced Techniques
[https://people.eecs.berkeley.edu/~bh/v2-toc2.html](https://people.eecs.berkeley.edu/~bh/v2-toc2.html)

Volume 3: Beyond Programming
[https://people.eecs.berkeley.edu/~bh/v3-toc2.html](https://people.eecs.berkeley.edu/~bh/v3-toc2.html)

---

### LOGO Foundation Books List

[https://el.media.mit.edu/logo-foundation/resources/books.html](https://el.media.mit.edu/logo-foundation/resources/books.html)

---

### Logo 15-word challenge

[http://www.mathcats.com/gallery/15wordcontest.html](http://www.mathcats.com/gallery/15wordcontest.html)

---

### String interning

> String interning is a method of storing only one copy of each distinct string value, which must be immutable. Interning strings makes some string processing tasks more time- or space-efficient at the cost of requiring more time when the string is created or interned [Wikipedia](https://en.wikipedia.org/wiki/String_interning)

As interning is a performance optimisation, it is not needed for the initial version.

When the time comes, the following links will be of use.

rustc uses a string interner internally, found here: [https://github.com/rust-lang/rust/blob/master/compiler/rustc_span/src/symbol.rs#L1783](https://github.com/rust-lang/rust/blob/master/compiler/rustc_span/src/symbol.rs#L1783) (as of Jan 2022).

There exists a crate that could be used for this:
[https://crates.io/crates/string-interner](https://crates.io/crates/string-interner)

and there's a post here: [https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html](https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html) which apparently has the same implementation as rustc and the above crate
	
and reddit discussion of same, here: [https://www.reddit.com/r/rust/comments/fn1jxf/blog_post_fast_and_simple_rust_interner/](https://www.reddit.com/r/rust/comments/fn1jxf/blog_post_fast_and_simple_rust_interner/)

and a gist with another implementation (discussed on reddit) here: [https://gist.github.com/CAD97/036c700fad1b4b159421eca089783122](https://gist.github.com/CAD97/036c700fad1b4b159421eca089783122)

This crate is also worth a look. Seems to have a decent API: [https://crates.io/crates/intaglio](https://crates.io/crates/intaglio)
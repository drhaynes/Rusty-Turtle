# Rusty-Turtle
A minimal Logo programming environment written in Rust, using egui for UI.

## TODO
* Draw turtle on canvas
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
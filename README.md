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
* Keep scrollback in terminal
* `>` Prompt
* Command history with up/down arrows
* TAB completion
* Fullscreen mode (default)

## Useful references:

#### Berkeley Logo

User Manual:
[https://people.eecs.berkeley.edu/~bh/usermanual](https://people.eecs.berkeley.edu/~bh/usermanual)

Source code:
[https://github.com/jrincayc/ucblogo-code](https://github.com/jrincayc/ucblogo-code)

Terminal implementation details:
[https://github.com/jrincayc/ucblogo-code/blob/master/newtermnotes](https://github.com/jrincayc/ucblogo-code/blob/master/newtermnotes)

Understanding the UCBLogo evaluator:
[https://github.com/jrincayc/ucblogo-code/blob/master/plm](https://github.com/jrincayc/ucblogo-code/blob/master/plm)

---

### c2 Logo Language page

> Logo is notable as one of the very few languages that ever had both DynamicScoping and TailCallOptimization at the same time. Over the years, more than one person has believed this combination to be impossible in the general case (see, for example, pages 12 and 13 of Lambda: The Ultimate Declarative

[http://wiki.c2.com/?LogoLanguage](http://wiki.c2.com/?LogoLanguage)


---

###Tail Recursion with Dynamic Scope by Darius Bacon

>This revision of a comp.lang.scheme article assumes you're fluent in Lisp and have a basic acquaintance with tail recursion and dynamic scoping of variables.

[http://web.archive.org/web/20090102070914/http://www.accesscom.com/~darius/writings/dynatail.html](http://web.archive.org/web/20090102070914/http://www.accesscom.com/~darius/writings/dynatail.html)

---

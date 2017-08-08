There is a lot to be done, but here is a quick example:

```
== A Lisp Interpreter ==
-- under construction --

>>> set 'defun (
...     lambda '(name params fn) '(global name (lambda params fn))
... )

    [nil]

>>> defun 'addtwo '(x) '(add x 2)

    [nil]

>>> addtwo 7

    9 [integer]

>>> '(1 2 3)

    {1 [integer], 2 [integer], 3 [integer]} [list]

>>> (1 2 3)

    Error: expected function name as first list item, found integer.

>>> / 2 -3

    -0.6666667 [float]

>>> defun 'fibo '(x) '(
...     cond
...         '((eq x 1) 1)
...         '((eq x 2) 1)
...         '(true (add (fibo (sub x 1)) (fibo (sub x 2))))
... )

    [nil]

>>> fibo 15

    610 [integer]

```
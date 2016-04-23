(((lambda (f) ((lambda (u) (u u)) (lambda (x) (f (lambda (v) ((x x) v)))))) (lambda (g) (lambda (ls) (if (null? ls) 0 (+ 1 (g (cdr ls))))))) '(1 2 3 4))

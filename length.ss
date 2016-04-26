(((lambda (f) ((lambda (u) (u u)) (lambda (x) (f (lambda (v) ((x x) v)))))) (lambda (g) (lambda (ls) (if (null? ls) 0 (add1 (g (cdr ls))))))) '(1 2 3 4))
(define sum (lambda (n acc) (if (= n 0) acc (sum (+ n -1) (+ n acc)))))
(define len (lambda (l) (if (null? l) 0 (add1 (len (cdr l))))))

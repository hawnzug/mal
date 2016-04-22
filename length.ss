(define len (lambda (l) (if (null? l) 0 (add1 (len (cdr l))))))

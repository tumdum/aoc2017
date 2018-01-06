#lang racket

(define (conv x)
  (- (char->integer x) (char->integer #\0)))

(define (sum-if-eq pair)
  (define a (car pair))
  (define b (cdr pair))
  (if (= a b) a 0))

(define (find-matching l to-skip)
  (define ll (take (drop (append l l) to-skip) (length l)))
  (define zipped  (map cons l ll))
  (foldl + 0 (map sum-if-eq zipped)))

(define (solution-a in)
  (find-matching (map conv (string->list in)) 1))

(define (solution-b in)
  (define to-skip (/ (string-length in) 2))
  (find-matching (map conv (string->list in)) to-skip))

(define input (string-trim (port->string)))
(println (solution-a input))
(println (solution-b input))

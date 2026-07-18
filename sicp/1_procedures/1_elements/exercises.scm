(/ (+ 5 4 (- 2 (- 3 (+ 6 (/ 4 5))))) (* 3 (- 6 2) (- 2 7)))
; -37/150

(define (sq a) (* a a))
(define (abs a) (if (< a 0) (- a) a))

; Exercise 1.3
(define (bigsq a b c) (
                       cond ((and (< a b) (< a c)) (+ (sq b) (sq c)))
                        ((and (< b a) (< b c)) (+ (sq a) (sq c)))
                        (else (if (> c a)(+ (sq c) (sq b))(+ (sq a) (sq b))))))

(define (bigsqd a b c) 
  (define gg (if (> a b) a b))
  (define gl (if (> a b) b a))
  (define gf (if (> c gl) c gl))
  (+ (sq gg) (sq gf)))

(define (test13)
  (display (bigsq 5 5 7))
  (display (bigsq 1 2 3))
  (newline)
  (display (bigsqd 5 5 7))
  (display (bigsqd 1 2 3))
  (newline)
  (display 7413)
  (newline))

; 1.4 is described in the title, a + b if b>0 a - b if b<0, thus it is a + abs(b)

; 1.5 has an infinite loop in the resolution of (p) when executed applicative-orderly

; 1.6
(define (improve guess x)
  (define (average x y) (/ (+ x y) 2))
  (average guess (/ x guess)))
(define (new-if pred thenc elsec) 
  (display 1.0)
  (cond (pred thenc)
    (else elsec)))
(define (sqrt-iter guess x)
  (define (good-enough? guess x)
   (< (abs (- (sq guess) x)) 0.001))
  (if
   (good-enough? guess x)
   guess
   (sqrt-iter (improve guess x) x)))
(define (sqrt a) (sqrt-iter 1.0 a))

; 1.7
(define (sqrt-beter guess x last)
   (define (good-enough? guess x last)
    (< (/ (abs (- guess last)) guess) 0.001))
  (if (= x 0) 0
   (if
    (good-enough? guess x last)
    guess
    (sqrt-beter (improve guess x) x guess))))
(define (sqrtb a) (sqrt-beter 1.0 a 0.0))

; 1.8
(define (cuber-beter guess x last)
   (define (good-enough? guess x last)
    (< (/ (abs (- guess last)) guess) 0.001))
   (define (cubeimpr y x) (/ (+ (/ x (* y y)) (* 2 y)) 3))
  (if (= x 0) 0
   (if
    (good-enough? guess x last)
    guess
    (cuber-beter (cubeimpr guess x) x guess))))
(define (cuber a) (cuber-beter 1.0 a 0.0))

; 1.9
; (+ a b)
; (inc (+ (a-1) b))
; (inc (inc (+ a-2 b)))
; recursive
; (+ a b)
; (+ (a-1) (b+1))
; (+ (a-2) (b+2))
; iterative

; 1.10
; if y is 0 => 0
; if x is 0 => 2*y
; if y is 1 => 2
; otherwise x -= 1; y = (A x y-1)
; (0 5)
; 10
; a: (A 0 n) = 2 * y
; (1 10)
; (0 (1 9))
; (0 (0 (1 8)))
; (0x9 (1 1))
; a: (A 1 n) = 2*2^(n-1) = 2^n
; (2 4)
; (1 (2 3))
; (0 (1 (2 3) - 1))
; | done on paper
; a: (A 2 n) = 2^2^n
(define (A x y) (cond ((= y 0) 0) ((= x 0) (* 2 y)) ((= y 1) 2) (else (A (- x 1) (A x (- y 1))))))

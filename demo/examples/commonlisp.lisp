(defpackage :hello
  (:use :cl))

(defun factorial (n)
  "Calculate factorial recursively"
  (if (<= n 1)
      1
      (* n (factorial (1- n)))))

(defclass person ()
  ((name :initarg :name :accessor person-name)
   (age :initarg :age :accessor person-age)))

(let ((result (factorial 5)))
  (format t "5! = ~a~%" result))

;;; hello.el --- Demo Emacs Lisp code -*- lexical-binding: t -*-

(defvar my-greeting "Hello"
  "The default greeting.")

(defun greet (name)
  "Greet NAME with a message."
  (interactive "sName: ")
  (message "%s, %s!" my-greeting name))

(defmacro with-timer (&rest body)
  "Execute BODY and report time taken."
  `(let ((start (current-time)))
     ,@body
     (message "Took %.3fs" (float-time (time-since start)))))

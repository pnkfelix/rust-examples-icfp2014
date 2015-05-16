(require 'htmlfontify)

;; If all you want to do is convert all the .rs files in a directory, then
;; htmlfontify-copy-and-link-dir is probably a much better choice.
;;
;; But I enjoy pain and want to have control over the individual files
;; that are processed, so I dived a little into the htmlfontify source
;; to identify the hfy-copy-and-fontify-file function used below.

(defun htmlfontify-file (file)
  (let ((dir (concat (file-name-as-directory default-directory) "src")))
    (let ((srcdir dir)
          (dstdir dir)
          (filename (file-name-nondirectory file)))
      (print `(hfy-copy-and-fontify-file ,srcdir ,dstdir ,filename))
      (hfy-copy-and-fontify-file srcdir dstdir filename))))



(defn board [size]
  (for [i (range size)]
    (.write js/document "Hell Yeah !")))

(board 10)

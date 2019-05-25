(ns pitch-lattice.core
  (:require [clojure.pprint :as p]))

(defrecord Pitch [cent ratio transposition limit degree])
(defrecord Set [pitches transposition limit])
(defrecord Lattice [scale set])

(defn range-inc
  "Make range upper bound inclusive"
  [num]
  (range 1 (+ num 1)))

(defn factors
  "Get the factors of a number"
  [num]
  (filter #(zero? (rem num %)) (range-inc num)))

(defn prime?
  "Determine if a number is prime"
  [num]
  (.isProbablePrime (BigInteger/valueOf num) 5))

(defn ratio-to-list
  "Converts a ratio to a list"
  [num]
  (if (ratio? num)
    (list (numerator num) (denominator num))
    (list num)))

(defn power-of-two?
  "Determines if a number is a power of two"
  [num]
  (and (not (zero? num)) (zero? (bit-and num (- num 1)))))

(defn recip
  "Returns the reciprocal of a ratio"
  [num]
  (if (ratio? num)
    (/ (denominator num) (numerator num))
    (/ 1 num)))

(defn ratio-to-cents
  "Converts a ratio to cents"
  [ratio]
  (* (Math/log10 ratio) (/ 1200 (Math/log10 2))))

(defn flatten-ratio
  "Brings the size of a ratio in between 1 and 2"
  [ratio]
  (cond
    (> ratio 2) (recur (/ ratio 2))
    (< ratio 1) (recur (* ratio 2))
    :else ratio))

(defn flip-ratio
  "Flips the numerator and denominator in a ratio"
  [ratio]
  (flatten-ratio (recip ratio)))

(defn limit
  "Calculates the partial limit of a ratio"
  [ratio]
  (apply max (filter prime? (mapcat factors (ratio-to-list ratio)))))

(defn sum-ratios
  "Add an arbitrary number of ratios"
  [& ratios]
  (flatten-ratio (apply * ratios)))

(defn diff-ratios
  "Get the difference between two ratios"
  [x y]
  (flatten-ratio (/ x y)))

(defn step
  "Interval size on the lattice based on direction and limit"
  [direction limit]
  (flatten-ratio
    (condp = direction
      :otonal limit
      :utonal (recip limit))))

(defn make-pitch
  "Constructs a Pitch record"
  [ratio]
  (->Pitch (ratio-to-cents ratio) ratio nil (limit ratio) nil))

;TODO - doesn't work for 7/5
(defn find-ordinal
  ""
  [ratio]
  (let [x (map #(power-of-two? (int %)) (ratio-to-list ratio))]
    (cond
      (first x) :utonal
      (last x) :otonal)))

(defn ordinal-walk
  "Creates a list of steps in one direction on the lattice"
  [step num]
  (map #(make-pitch (apply sum-ratios (replicate % step))) (range-inc num)))

(defn make-set
  ""
  [transposition limit steps]
  (->Set (ordinal-walk (step transposition limit) steps) transposition limit))

;TODO - get-scale = sort a collection of "make-set"s by ratio size

; ----
; test
; ----
(def pitches (list (make-set :otonal 3 3)
                   (make-set :utonal 3 3)
                   (make-set :otonal 5 3)
                   (make-set :utonal 5 3)))
(def testdata (->Lattice
                (map make-pitch (sort < (map :ratio (mapcat :pitches pitches))))
                pitches))

(p/pprint testdata)
; generate a 2-d list based on:
; - limits desired in tuning
; - number of steps out

; link any ratio, and generate a new lattice as root
; add a 'relative' record {:base :ratio}

; How to represent the data
; ---
; ({:ratio
;   :cent
;   :transposition
;   :limit
;   :scaledegree})
/// <reference types="typescript" />

declare global {

  // Callbacks
  /**
   * An interpolation function
   * @callback interpolationCallback
   * @param {number} a The minimum number
   * @param {number} b The maximum number
   * @param {number} i The interpolation value, should be in the interval [0, 1]
   * @return {number} The interpolated value in the interval [a, b]
   */
  type interpolationCallback = (a: number, b: number, i: number) => number;

  /**
   * A function for generating array values
   * @callback timesCallback
   * @param {number} i The array index
   * @return {*} The array value
   */
  type timesCallback = (i: number) => any;

  // Math functions
  interface Math {
    /**
     * Check if two numbers are approximately equal
     * @param {number} a Number a
     * @param {number} b Number b
     * @param {number} [p=Number.EPSILON] The precision value
     * @return {boolean} True if numbers a and b are approximately equal
     */
    floatEquals(a: number, b: number, p?: number): boolean,

    /**
     * Clamp a number between min and max
     * @param {number} a The number to clamp
     * @param {number} [min=0] The minimum value
     * @param {number} [max=1] The maximum value
     * @return {number} A clamped number
     */
    clamp(a: number, min?: number, max?: number): number,

    /**
     * Get the fractional part of a number
     * @param {number} a The number from which to get the fractional part
     * @return {number} The fractional part of the number
     */
    frac(a: number): number,

    /**
     * Do a linear interpolation between a and b
     * @param {number} a The minimum number
     * @param {number} b The maximum number
     * @param {number} i The interpolation value, should be in the interval [0, 1]
     * @return {number} An interpolated value in the interval [a, b]
     */
    lerp(a: number, b: number, i: number): number,

    /**
     * Get the position of i between a and b
     * @param {number} a The minimum number
     * @param {number} b The maximum number
     * @param {number} i The interpolated value in the interval [a, b]
     * @return {number} The position of i between a and b
     */
    unlerp(a: number, b: number, i: number): number,

    /**
     * Do a bilinear interpolation
     * @param {number} c00 Top-left value
     * @param {number} c10 Top-right value
     * @param {number} c01 Bottom-left value
     * @param {number} c11 Bottom-right value
     * @param {number} ix Interpolation value along x
     * @param {number} iy Interpolation value along y
     * @return {number} A bilinear interpolated value
     */
    blerp(c00: number, c10: number, c01: number, c11: number, ix: number, iy: number): number,

    /**
     * Re-map a number i from range a1...a2 to b1...b2
     * @param {number} i The number to re-map
     * @param {number} a1
     * @param {number} a2
     * @param {number} b1
     * @param {number} b2
     * @return {number}
     */
    remap(i: number, a1: number, a2: number, b1: number, b2: number): number,

    /**
     * Do a smooth interpolation between a and b
     * @param {number} a The minimum number
     * @param {number} b The maximum number
     * @param {number} i The interpolation value
     * @return {number} An interpolated value in the interval [a, b]
     */
    smoothstep(a: number, b: number, i: number): number,

    /**
     * Get an angle in radians
     * @param {number} degrees The angle in degrees
     * @return {number} The angle in radians
     */
    radians(degrees: number): number,

    /**
     * Get an angle in degrees
     * @param {number} radians The angle in radians
     * @return {number} The angle in degrees
     */
    degrees(radians: number): number,

    /**
     * Get a random float in the interval [min, max)
     * @param {number} min Inclusive min
     * @param {number} max Exclusive max
     * @return {number} A random float in the interval [min, max)
     */
    randomBetween(min: number, max: number): number,

    /**
     * Get a random integer in the interval [min, max]
     * @param {number} min Inclusive min
     * @param {number} max Inclusive max
     * @return {number} A random integer in the interval [min, max]
     */
    randomIntBetween(min: number, max: number): number,

    /**
     * Get a normally-distributed random number
     * @param {number} [mu=0.5] The mean value
     * @param {number} [sigma=0.5] The standard deviation
     * @param {number} [samples=2] The number of samples
     * @return {number} A normally-distributed random number
     */
    cltRandom(mu?: number, sigma?: number, samples?: number): number,

    /**
     * Get a normally-distributed random integer in the interval [min, max]
     * @param {number} min Inclusive min
     * @param {number} max Inclusive max
     * @return {number} A normally-distributed random integer
     */
    cltRandomInt(min: number, max: number): number,

    /**
     * Return a weighted random integer
     * @param {Array<number>} w An array of weights
     * @return {number} An index from w
     */
    weightedRandom(w: Array<number>): number,

    /**
     * Return an interpolated value from an array
     * @param {Array<number>} a An array of values interpolate
     * @param {number} i A number in the interval [0, 1]
     * @param {interpolationCallback} [f=Math.lerp] The interpolation function to use
     * @return {number} An interpolated value in the interval [min(a), max(a)]
     */
    lerpArray(a: Array<number>, i: number, f?: interpolationCallback): number,

    /**
     * Get the dot product of two vectors
     * @param {Array<number>} a Vector a
     * @param {Array<number>} b Vector b
     * @return {number} a ∙ b
     */
    dot(a: Array<number>, b: Array<number>): number,

    /**
     * Get the factorial of a number
     * @param {number} a
     * @return {number} a!
     */
    factorial(a: number): number,

    /**
     * Get the number of permutations of r elements from a set of n elements
     * @param {number} n
     * @param {number} r
     * @return {number} nPr
     */
    permutation(n: number, r: number): number,

    /**
     * Get the number of combinations of r elements from a set of n elements
     * @param {number} n
     * @param {number} r
     * @return {number} nCr
     */
    combination(n: number, r: number): number
  }

  // Array constructor functions
  interface ArrayConstructor {
    /**
     * Return a new array with length n by calling function f(i) on each element
     * @param {timesCallback} f
     * @param {number} n The size of the array
     * @return {Array<*>}
     */
    times(f: timesCallback, n: number): Array<any>,

    /**
     * Return an array containing numbers 0->(n - 1)
     * @param {number} n The size of the array
     * @return {Array<number>} An array of integers 0->(n - 1)
     */
    range(n: number): Array<number>,

    /**
     * Zip 2 arrays together, i.e. ([1, 2, 3], [a, b, c]) => [[1, a], [2, b], [3, c]]
     * @param {Array<*>} a
     * @param {Array<*>} b
     * @return {Array<*>}
     */
    zip(a: Array<any>, b: Array<any>): Array<Array<any>>
  }

  // Array instance functions
  interface Array<T> {
    /**
     * Return array[i] with positive and negative wrapping
     * @param {number} i The positively/negatively wrapped array index
     * @return {*} An element from the array
     */
    at(i: number): T,

    /**
     * Chop an array into chunks of size n
     * @param {number} n The chunk size
     * @return {Array<Array<*>>} An array of array chunks
     */
    chunk(n: number): Array<Array<any>>,

    /**
     * Randomly shuffle an array in-place
     * @return {Array<*>} The shuffled array
     */
    shuffle(): Array<any>
  }
}

/**
 * A 2d vector
 * @typedef {Object} vec
 * @property {number} x The x component of the vector
 * @property {number} y The y component of the vector
 */
declare type vec = {
  x: number,
  y: number
}

/**
 * A function to call on each component of a vector
 * @callback vectorMapCallback
 * @param {number} value The component value
 * @param {'x' | 'y'} label The component label (x or y)
 * @return {number} The mapped component
 */
type vectorMapCallback = (i: number) => any;

declare const vec: {
  /**
   * Create a new vector
   * @param {number|vec} [x] The x component of the vector, or a vector to copy
   * @param {number} [y] The y component of the vector
   * @return {vec} A new vector
   * @example <caption>Various ways to initialise a vector</caption>
   * let a = vec(3, 2);  // (3, 2)
   * let b = vec(4);     // (4, 4)
   * let c = vec(a);     // (3, 2)
   * let d = vec();      // (0, 0)
   */
  (x?: number, y?: number): vec,

  /**
   * Get the components of a vector as an array
   * @param {vec} a The vector to get components from
   * @return {Array<number>} The vector components as an array
   */
  components(a: vec): Array<number>,

  /**
   * Return a unit vector (1, 0)
   * @return {vec} A unit vector (1, 0)
   */
  ux(): vec,

  /**
   * Return a unit vector (0, 1)
   * @return {vec} A unit vector (0, 1)
   */
  uy(): vec,

  /**
   * Add vectors
   * @param {vec} a Vector a
   * @param {vec} b Vector b
   * @return {vec} a + b
   */
  add(a: vec, b: vec): vec,

  /**
   * Scale a vector
   * @param {vec} a Vector a
   * @param {number} b Scalar b
   * @return {vec} a * b
   */
  mul(a: vec, b: number): vec,

  /**
   * Subtract vectors
   * @param {vec} a Vector a
   * @param {vec} b Vector b
   * @return {vec} a - b
   */
  sub(a: vec, b: vec): vec,

  /**
   * Get the length of a vector
   * @param {vec} a Vector a
   * @return {number} |a|
   */
  len(a: vec): number,

  /**
   * Get the length of a vector using taxicab geometry
   * @param {vec} a Vector a
   * @return {number} |a|
   */
  manhattan(a: vec): number,

  /**
   * Normalise a vector
   * @param {vec} a The vector to normalise
   * @return {vec} ^a
   */
  nor(a: vec): vec,

  /**
   * Get a dot product of vectors
   * @param {vec} a Vector a
   * @param {vec} b Vector b
   * @return {number} a ∙ b
   */
  dot(a: vec, b: vec): number,

  /**
   * Rotate a vector by r radians
   * @param {vec} a The vector to rotate
   * @param {number} r The angle to rotate by, measured in radians
   * @return {vec} A rotated vector
   */
  rot(a: vec, r: number): vec,

  /**
   * Check if two vectors are equal
   * @param {vec} a Vector a
   * @param {vec} b Vector b
   * @return {boolean} True if vectors a and b are equal, false otherwise
   */
  eq(a: vec, b: vec): boolean,

  /**
   * Get the angle of a vector
   * @param {vec} a Vector a
   * @return {number} The angle of vector a in radians
   */
  rad(a: vec): number,

  /**
   * Copy a vector
   * @param {vec} a The vector to copy
   * @return {vec} A copy of vector a
   */
  cpy(a: vec): vec,

  /**
   * Call a function on each component of a vector and build a new vector from the results
   * @param {vec} a Vector a
   * @param {vectorMapCallback} f The function to call on each component of the vector
   * @return {vec} Vector a mapped through f
   */
  map(a: vec, f: vectorMapCallback): vec,

  /**
   * Convert a vector into a string
   * @param {vec} a The vector to convert
   * @param {string} [s=', '] The separator string
   * @return {string} A string representation of the vector
   */
  str(a: vec, s?:string): string,
}

/**
 * A matrix
 * @typedef {Object} mat
 * @property {number} m The number of rows in the matrix
 * @property {number} n The number of columns in the matrix
 * @property {Array<number>} entries The matrix values
 */
declare type mat = {
  m: number,
  n: number,
  entries: Array<number>
}

/**
 * A function to call on each entry of a matrix
 * @callback matrixMapCallback
 * @param {number} value The entry value
 * @param {number} index The entry index
 * @param {Array<number>} entries The array of matrix entries
 * @return {number} The mapped entry
 */
type matrixMapCallback = (value: number, index: number, entries: Array<number>) => any;

declare const mat: {
  /**
   * Create a new matrix
   * @param {number} [m=4] The number of rows
   * @param {number} [n=4] The number of columns
   * @param {Array<number>} [entries=[]] Matrix values in reading order
   * @return {mat} A new matrix
   */
  (m?: number, n?: number, entries?: Array<number>): mat,

  /**
   * Get an identity matrix of size n
   * @param {number} n The size of the matrix
   * @return {mat} An identity matrix
   */
  identity(n: number): mat,

  /**
   * Get an entry from a matrix
   * @param {mat} a Matrix a
   * @param {number} i The row offset
   * @param {number} j The column offset
   * @return {number} The value at position (i, j) in matrix a
   */
  get(a: mat, i: number, j: number): number,

  /**
   * Set an entry of a matrix
   * @param {mat} a Matrix a
   * @param {number} i The row offset
   * @param {number} j The column offset
   * @param {number} v The value to set in matrix a
   */
  set(a: mat, i: number, j: number, v: number): void,

  /**
   * Get a row from a matrix as an array
   * @param {mat} a Matrix a
   * @param {number} m The row offset
   * @return {Array<number>} Row m from matrix a
   */
  row(a: mat, m: number): Array<number>,

  /**
   * Get a column from a matrix as an array
   * @param {mat} a Matrix a
   * @param {number} n The column offset
   * @return {Array<number>} Column n from matrix a
   */
  col(a: mat, n: number): Array<number>,

  /**
   * Add matrices
   * @param {mat} a Matrix a
   * @param {mat} b Matrix b
   * @return {mat} a + b
   */
  add(a: mat, b: mat): mat,

  /**
   * Subtract matrices
   * @param {mat} a Matrix a
   * @param {mat} b Matrix b
   * @return {mat} a - b
   */
  sub(a: mat, b: mat): mat,

  /**
   * Multiply matrices
   * @param {mat} a Matrix a
   * @param {mat} b Matrix b
   * @return {mat|boolean} ab or false if the matrices cannot be multiplied
   */
  mul(a: mat, b: mat): mat | boolean,

  /**
   * Scale a matrix
   * @param {mat} a Matrix a
   * @param {number} b Scalar b
   * @return {mat} a * b
   */
  scale(a: mat, b: number): mat,

  /**
   * Transpose a matrix
   * @param {mat} a The matrix to transpose
   * @return {mat} A transposed matrix
   */
  trans(a: mat): mat,

  /**
   * Get the minor of a matrix
   * @param {mat} a Matrix a
   * @param {number} i The row offset
   * @param {number} j The column offset
   * @return {mat|boolean} The (i, j) minor of matrix a or false if the matrix is not square
   */
  minor(a: mat, i: number, j: number): mat | boolean,

  /**
   * Get the determinant of a matrix
   * @param {mat} a Matrix a
   * @return {number|boolean} |a| or false if the matrix is not square
   */
  det(a: mat): number | boolean,

  /**
   * Normalise a matrix
   * @param {mat} a The matrix to normalise
   * @return {mat|boolean} ^a or false if the matrix is not square
   */
  nor(a: mat): mat | boolean,

  /**
   * Get the adjugate of a matrix
   * @param {mat} a The matrix from which to get the adjugate
   * @return {mat} The adjugate of a
   */
  adj(a: mat): mat,

  /**
   * Get the inverse of a matrix
   * @param {mat} a The matrix to invert
   * @return {mat|boolean} a^-1 or false if the matrix has no inverse
   */
  inv(a: mat): mat | boolean,

  /**
   * Check if two matrices are equal
   * @param {mat} a Matrix a
   * @param {mat} b Matrix b
   * @return {boolean} True if matrices a and b are identical, false otherwise
   */
  eq(a: mat, b: mat): boolean,

  /**
   * Copy a matrix
   * @param {mat} a The matrix to copy
   * @return {mat} A copy of matrix a
   */
  cpy(a: mat): mat,

  /**
   * Call a function on each entry of a matrix and build a new matrix from the results
   * @param {mat} a Matrix a
   * @param {matrixMapCallback} f The function to call on each entry of the matrix
   * @return {mat} Matrix a mapped through f
   */
  map(a: mat, f: matrixMapCallback): mat,

  /**
   * Convert a matrix into a string
   * @param {mat} a The matrix to convert
   * @param {string} [ms=', '] The separator string for columns
   * @param {string} [ns='\n'] The separator string for rows
   * @return {string} A string representation of the matrix
   */
  str(a: mat, ms?: string, ns?: string): string,
}

export { vec, mat }

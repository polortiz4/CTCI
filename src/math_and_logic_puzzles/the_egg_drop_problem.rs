// Solution to Exercise 6.8

// There is a 100 story building. Dropping an egg from the Nth floor or higher will break the egg,
// With only two eggs, find N minimizing the number of drops you make

// Drop from 14, 27, 39, 50, 60, 69, 77, 84, 90, 95, 99, 100 until it breaks
// If it breaks, throw the other one incrementally from the previously successfuly throw with increments of 1.
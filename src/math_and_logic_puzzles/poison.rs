// Solution to Exercise 6.10

// For each bottle, add a drop to each strip in a unique order.
// i.e.
// Bottle 1 adds a drop only to strip 1,
// Bottle 2 adds a drop to strip 2,
// Bottle 3 adds a drop to strip 1 AND 2
// Bottle 4 adds a drop to strip 3,
// Bottle 5 adds a drop to strips 3 AND 1,
// ...
// The signature of the strips that activate match exactly with the corresponding bottle after 7 days
// This works if the number of bottles is less than 2^(n_strips)
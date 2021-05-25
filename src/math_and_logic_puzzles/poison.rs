// Solution to Exercise 6.10

// For each strip in the first day, add one drop from 100 different bottles (each strip get a different 100 bottles).
// In the second day, for each group of 100 bottles (one per strip), make 9 groups of 11 and taint the other strips with one group each, leaving the last bottle out.
// In the third day, for each group of eleven, split into 8 and taint the other strips (the ones where said bottle hasn't been tested), leaving 3 out.
// In the fourth day, taint 3 of the remaining 7 strips for each bottle and taint them with the 3 that were left out before.

// After 7 days, if no strip activates, all bottles are fine
// After 8 days, if no strip activates, the poisoned bottle was the one left out which belonged to the group of 100 that activated the previous day.
// After 9 days, if no strip activates, you need to wait for the 10th day to figure out which one it is.
// After 9 days, if a strip activates, the poisoned bottle is the one that was present in all 3 activated strips.
// After 10 days, the poisoned bottle is the one that belonged to the last activated strip (each strip has stuff from only one bottle)
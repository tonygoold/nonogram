# Ways to Solve a Black and White Nonogram

## Terminology

There are three states a cell can be in:

* Blank: No decision has been made.
* Filled: The cell is black.
* Clear: The cell is white.

A run refers to the row and column constraints, indicating the number of cells
in a row that must be fulled. For example, a row with a constraint of "1 7 3"
has three runs of length 1, 7, and 3 respectively. In a black and white
Nonogram, the runs must be separated from each other by at least one clear
cell.

## Identify Required Cells

From a blank grid, start by looking at all possible placements of the runs in
each row and column. If all possible placements fill a given cell, then that
cell must be filled. This step cannot identify clear cells.

## Refine Required Cells

When the grid has some filled or clear cells, repeat the Identiy Required
Cells method, rejecting placements that overlap clear cells. This time,
clear cells can also be identified when no possible placements fill a given
cell.

## Extend Filled Cells

If a run necessarily overlaps a filled cell then, at its earliest placement,
any cells it covers after the overlapped cell must also be filled. Similarly,
at its latest placement, any cells it covers before the overlapped cell must
also be filled.

## Reject Invalid Conjunctions

If a placement of a run would cause it to abut or overlap one or more filled
cells, such that it would create an overall run longer than the run being
placed, the placement should be rejected as invalid.

## Clear Placed Run Borders

Once the unique placement of a run has been identified, the cells immediately
before and after it (if they exist) can be cleared.

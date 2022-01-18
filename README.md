# WordleSolver
Automated solving for wordle

The basic idea is that a word is a good guess if it maximizes the expected information gain, or equivalently minimizes the remaining possible words. As the answer is not known, we sum the info gain across all possible answers and choose the guess with the highest expected return. This is equivalent to minimizing the average number of total guesses to solve the problem. 

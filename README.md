# Guessing Game

A Rust implementation of an enhanced guessing game based on the Rust documentation tutorial.

## Overview

This project implements an interactive number guessing game where players attempt to guess a randomly selected number between 1 and 100.

## Project Information

- **Language**: Rust
- **Edition**: 2024
- **Main Dependency**: `rand` crate (v0.8.5)

## Enhancements from the Original Rust Documentation

This implementation extends the basic guessing game from the Rust book with the following improvements:

### 1. **Attempt Limit System**
- **Change**: Added a 10-attempt limit with a counter that decrements after each guess
- **Original**: The standard tutorial allowed unlimited guesses
- **Benefit**: Adds challenge and increases game difficulty

### 2. **Multiple Exit Commands**
- **Change**: Implemented support for multiple quit commands: `exit`, `q`, `quit`, `end`, and `cancel`
- **Original**: No built-in way to quit the game gracefully
- **Benefit**: Provides user flexibility and improves user experience with familiar command options

### 3. **Enhanced User Prompts**
- **Change**: Added comprehensive welcome messages explaining the game rules and constraints upfront
- **Original**: Minimal feedback to the player
- **Includes**:
  - Welcome message
  - Number range information (1-100)
  - Attempt limit notification
  - Quit command instructions

### 4. **Attempt Counter Display**
- **Change**: Shows remaining attempts after each guess with `"You have {counter} turns left."`
- **Original**: No tracking or display of remaining attempts
- **Benefit**: Gives players strategic awareness of their remaining opportunities

### 5. **Explicit Game Over Condition**
- **Change**: Displays "Game over!" message when attempts are exhausted before finding the number
- **Original**: Game would simply continue indefinitely
- **Benefit**: Provides clear feedback about losing the game

### 6. **Improved Input Validation**
- **Change**: Specific error message for invalid input with suggestion to enter a valid number or quit command
- **Original**: Generic error handling
- **Benefit**: Better user guidance when invalid input is provided

### 7. **Number Reveal on Exit**
- **Change**: Displays the secret number when player exits using quit commands
- **Original**: Secret never revealed unless guessed correctly
- **Benefit**: Satisfies player curiosity and provides learning opportunity

## Building and Running

To build and run the project:

```bash
cargo build
cargo run
```

## Game Instructions

1. The game will prompt you to enter a number between 1 and 100
2. You have 10 attempts to guess correctly
3. After each guess, you'll receive feedback: "Too small!" or "Too big!"
4. Type your guess and press Enter
5. To exit early, type: `exit`, `q`, `quit`, `end`, or `cancel`

## Example Gameplay

```
Welcome to the Guessing Game!
I have selected a random number between 1 and 100.
You have 10 attempts to guess the number.
Type 'exit', 'q', 'quit', 'end', or 'cancel' to give up and reveal the number.

Guess the number!
Please input your guess.
50
You have 9 turns left.
Too big!
Please input your guess.
25
You have 8 turns left.
Too small!
...
```

## Key Features

✅ Random number generation between 1-100  
✅ Limited attempts (10 guesses)  
✅ Multiple ways to quit gracefully  
✅ Clear feedback on guesses (too high/too low)  
✅ Automatic game over detection  
✅ User-friendly prompts and instructions  

## Learning Outcomes

This project demonstrates:
- Rust's `match` expressions for pattern matching
- Error handling with the `Result` type
- String parsing and validation
- Random number generation with the `rand` crate
- User input handling with `std::io`
- Control flow with loops and conditionals

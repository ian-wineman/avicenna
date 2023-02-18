# avicenna
text editor written in Rust

## todo
1. Properly update cursor_x & cursor_y as new characters are added and removed
2. Use cursor_x, cursor_y, & char_width to draw cursor in correct position
3. Implement arrow keys to move cursor position

## future features
- input
  - use of shift + key to insert a different key
    - lower and upper case letters
    - some special characters
  - paste from clipboard
  - hold delete key 
    - maybe do a while loop for while it is pressed and in the while loop you have a push Backspace then a wait function
- cursor
  - move cursor by clicking mouse somewhere
  - cursor follows text with typing
- selection
  - select text with mouse
  - copy selected text
// Non ASCII
type ウニット {
    .作: ウニット,
}

// Non BMP
let 😊🙃 = ウニット.作;

// This "è" is a single character (no combining marks), so this is legal.
let è = 😊🙃;

// Comments can contain combining marks:
// àら゙ 

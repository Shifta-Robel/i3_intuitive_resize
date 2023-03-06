 # i3_intuitive_resize
 
 Resize the focused i3 window based on its position inside the workspace.

![Demo](demo/demo.gif)

## When Intuition fails
- If window is neither on the left corner or the right corner
   - left -> means decrease width
   - right -> means increase width
- If window is neither on the top corner or the bottom corner
   - up -> means increase height
   - down -> means decrease height 

## Known problems
- Couldn't figure out how to get the <code>inner gaps</code> used, so I just hacked my way around and created a
  <code>ASSUMED_MAX_INNER_GAP</code> variable, the code will fail to deduce position of window if <code>inner gaps</code> used are bigger
  than that value.

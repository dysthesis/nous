# Things that we must enforce

- Loss of data must be explicit. Nous must not cause the loss of any amount of
  data outside explicit operations such as deletion.
- Addition of data must be explicit. Nous must not add any amount of data 
  outside of explicit operations such as addition
  - Should this include metadata as well? _E.g._ would it be valid to add 
    comment strings to keep track of block IDs?

---
title: TEXT Function
---

# The `text`/`textln` functions

Output your information in `Text` format. This internally uses `document.body.innerText`. 
Using these functions will escape HTML characters.

## Example

```pascal:line-numbers
program san_textln(input,output);
   var user_name: string;
begin
  readln(user_name);
  textln('Hi ', user_name); // [!code focus]
end;
```

<style>
    * {
        scroll-behavior: smooth;
    }
</style>

---
title: CLEAR Function
---

# The `clear`/`clean` functions

Helps to clear the old outputs from the screen. This internally uses `document.body.innerHTML = ""`.

## Example

```pascal:line-numbers
program san_clearln(input,output);
   var user_name: string;
begin
  html('Lets make the screen full of text 😅 \n');
  html('Lets make the screen full of text 😅 \n');
  html('Lets make the screen full of text 😅 \n');
  html('Lets make the screen full of text 😅 \n');
  html('Lets make the screen full of text 😅 \n');
  clear(); // [!code focus]
  html('Follow me at https://instagram.com/sanjaiyan_dev \n');
end;
```

<style>
    * {
        scroll-behavior: smooth;
    }
</style>

---
title: HTML Function
---

# The `html`/`htmln` functions

Output your information in `HTML` format. This internally uses `document.body.innerHTML`.

## Example

```pascal:line-numbers
program san_htmln(input,output);
   var user_name: string;
begin
  htmln('<button> Greet Me </button>'); // [!code focus]
  readln(user_name);
  html('<h1> Hi ', user_name, '</h1>'); // [!code focus]
end;
```

<style>
    * {
        scroll-behavior: smooth;
    }
</style>

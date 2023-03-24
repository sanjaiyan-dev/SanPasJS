---
title: Caveats
---

# Caveats related to syntaxes

## Issue when doing multiplication

Currently `sanpasjs` doesn't support `*` character to multiply but, you can overcome this issue using `mul` key word.

```pascal:line-numbers
program multiply_san(input,output);
    var num: integer;
    var user_num: integer;
begin
   num := 3;
   writeln('Enter a number to multiply by 03');
   readln(user_num)
   num := num * user_num; // [!code --]
   num := num mul user_num; // [!code ++]
   html('Answer is ', num);
end;
```

## Outputting function in next line

Currently there is no affect in any function which ends with `ln`. To overcome this issue please type `"\n"`.

```pascal:line-numbers{4,6}
program nxt_line_san(input,output);
begin
  htmln('I wont change the cursor to next line :(');
  htmln('I will change the cursor to next line :) \n');
  text('I wont change the cursor to next line :(');
  textln('I will change the cursor to next line :) \n');
end;
```

## Lack of syntax support

Currently some sytax won't be supported :worried: which includes

- `switch` statement
- `procedure` with more arguments (single argument supported)
- `function` keyword
  <br/>
  etc...

<style>
    * {
        scroll-behavior: smooth;
    }
</style>

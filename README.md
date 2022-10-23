# Why?

Sometimes I can't decide how to spend a holiday or a night.

# How to Use

1. Create a plain text file with your options, one per line.
2. Run `./help-me-decide <PATH_TO_FILE> [NUM_RESULTS]` at root. If `NUM_RESULTS` is not specified, `3` is the default value.
3. *Cosmic magic shall give what thou seek.*

# Example

```
$ printf "McDonald's\nKFC\nBurger King\nWendy's\nShake Shack\n" >| what-to-eat.txt
$ ./help-me-decide what-to-eat.txt 3
🥇 Shake Shack
🥈 Wendy's
🥉 Burger King
```
